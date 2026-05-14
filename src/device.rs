//! Device-open helpers with human-readable error messages.
//!
//! hidapi-rs's `linux-native` backend silently skips Boot-subclass
//! hidraws during enumeration, which means it can't find the K2 via
//! `open(vid, pid)`. We walk sysfs ourselves, pick the hidraw whose USB
//! interface number is 1 (the vendor command channel — interface 0 is
//! the boot keyboard), and `open_path` that.

use std::ffi::CString;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use hidapi::{HidApi, HidDevice};

use crate::consts::{PRODUCT_ID, VENDOR_ID};

/// Which USB interface on the K2 we want.
const VENDOR_INTERFACE: u8 = 1;

pub struct Device {
    pub handle: HidDevice,
}

impl Device {
    pub fn open() -> Result<Self> {
        let api = HidApi::new().context("hidapi init failed")?;
        let path = find_vendor_hidraw()?;
        let cpath = CString::new(path.as_os_str().as_encoded_bytes())
            .context("hidraw path contained NUL byte")?;
        match api.open_path(&cpath) {
            Ok(handle) => Ok(Device { handle }),
            Err(e) => Err(diagnose_open_err(&path, e)),
        }
    }

    pub fn write(&self, frame: &[u8]) -> Result<()> {
        self.handle.write(frame).map(|_| ()).map_err(|e| {
            anyhow!(
                "write to K2 failed (keyboard is read-only for this process?): {e}\n\
                 \n\
                 If this is a permissions issue, see the contrib/70-sk2rgb.rules\n\
                 file shipped with sk2rgb_tool."
            )
        })
    }
}

/// Walk `/sys/class/hidraw/` to find the K2's vendor-channel hidraw.
fn find_vendor_hidraw() -> Result<PathBuf> {
    let want_hid_id = format!("HID_ID=0003:{VENDOR_ID:08X}:{PRODUCT_ID:08X}");
    let mut saw_k2_at_all = false;

    let entries = fs::read_dir("/sys/class/hidraw")
        .context("can't read /sys/class/hidraw — is sysfs mounted?")?;
    for entry in entries {
        let entry = entry?;
        let hidraw_name = entry.file_name();
        let hid_device = match fs::canonicalize(entry.path().join("device")) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let uevent = match fs::read_to_string(hid_device.join("uevent")) {
            Ok(s) => s,
            Err(_) => continue,
        };
        if !uevent.lines().any(|l| l == want_hid_id) {
            continue;
        }
        saw_k2_at_all = true;

        // Walk up: the HID device's parent is the USB interface dir
        // (something like `.../1-5:1.1/`), which has a bInterfaceNumber.
        let usb_iface = hid_device
            .parent()
            .ok_or_else(|| anyhow!("HID device {hid_device:?} has no parent"))?;
        let iface_num: u8 = fs::read_to_string(usb_iface.join("bInterfaceNumber"))
            .with_context(|| format!("can't read bInterfaceNumber under {usb_iface:?}"))?
            .trim()
            .parse()
            .context("bInterfaceNumber wasn't a number")?;

        if iface_num == VENDOR_INTERFACE {
            return Ok(PathBuf::from("/dev").join(hidraw_name));
        }
    }

    if saw_k2_at_all {
        Err(anyhow!(
            "found a K2 ({VENDOR_ID:04x}:{PRODUCT_ID:04x}) on USB but couldn't locate its vendor-command interface (#{VENDOR_INTERFACE}).\n\
             \n\
             This shouldn't happen on a stock K2. The kernel may have failed to bind hidraw to interface {VENDOR_INTERFACE}, or the device firmware differs from what SPEC.md describes."
        ))
    } else {
        Err(anyhow!(
            "couldn't find a K2 keyboard on USB ({VENDOR_ID:04x}:{PRODUCT_ID:04x}).\n\
             \n\
             Is it plugged in? Check `lsusb | grep {VENDOR_ID:04x}:{PRODUCT_ID:04x}`."
        ))
    }
}

fn diagnose_open_err(path: &std::path::Path, e: hidapi::HidError) -> anyhow::Error {
    let msg = e.to_string();
    let lower = msg.to_ascii_lowercase();

    if lower.contains("permission") || lower.contains("eacces") {
        return anyhow!(
            "permission denied opening {path:?} — the keyboard is read-only for your user.\n\
             \n\
             Either:\n\
              • Make sure you're logged in at a local seat (modern systemd-udev grants a uaccess ACL automatically), or\n\
              • Install the udev rule shipped with this tool:\n\
                  sudo cp contrib/70-sk2rgb.rules /etc/udev/rules.d/\n\
                  sudo udevadm control --reload\n\
                  sudo udevadm trigger --subsystem-match=hidraw\n\
             \n\
             Then unplug/replug the keyboard or rerun this command.\n\
             \n\
             (underlying error: {msg})"
        );
    }

    if lower.contains("busy") || lower.contains("ebusy") {
        return anyhow!(
            "K2 at {path:?} is busy — another process has it open. Common culprits: OpenRGB,\n\
             ckb-next, the vendor app running under wine. Close whichever owns it.\n\
             \n\
             (underlying error: {msg})"
        );
    }

    anyhow!("couldn't open K2 at {path:?}: {msg}")
}
