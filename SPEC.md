# CyberPowerPC Skorpion K2 RGB — vendor protocol notes

Working notes for reverse-engineering the SK2RGB keyboard's key-remap and
LED protocol. Driven by the plan at
`~/.claude/plans/i-had-a-note-reflective-pixel.md`.

## Device identity

- **VID:PID**: `04b4:5004` (Cypress Semiconductor Corp., generic reference)
- **bcdDevice**: 3.04
- **Speed**: USB 2.0, Full Speed (12 Mbps)
- **No string descriptors**: `iManufacturer = iProduct = iSerial = 0`. Vendor
  didn't populate them — typical for cheap OEM keyboards built on a Cypress
  reference design. The `04b4:5004` PID is **not unique to CyberPowerPC**;
  several rebadges of the same hardware exist. Worth searching other vendor
  forks of any RE work we find under that VID:PID.

## Interface layout

Two interfaces, both HID-class:

| iface | class/sub/proto         | endpoints              | what it really is              |
|-------|-------------------------|------------------------|--------------------------------|
| 0     | HID / Boot / Keyboard   | EP1 IN  (8 B int, 1ms) | Boot-protocol keyboard scancodes |
| 1     | HID / Boot / **Mouse**  | EP2 IN (64 B int, 1ms), **EP3 OUT** (64 B int, 1ms) | **Vendor command channel** disguised as mouse |

### The smoking gun

A real keyboard never has a **64-byte interrupt OUT endpoint**. LED state
(caps/num/scroll lock) is a 1-byte output report on the keyboard interface,
not a separate 64-byte interrupt endpoint.

Interface 1 calls itself "Mouse" but the report descriptor (176 B,
`UNAVAILABLE` without root) almost certainly defines vendor-specific reports.
The pairing of EP2 IN + EP3 OUT, both 64 B / 1 ms, is the **vendor command
channel** — exactly where `SK2RGB driver.exe` will send key-remap and LED
programming commands.

### Implications for the Linux tool

- It's HID-class, so `hidapi-rs` works (no kernel driver detach needed —
  `hidraw` exposes interface 1 directly).
- Commands are 64-byte interrupt OUT reports. Replies are 64-byte interrupt
  IN reports on EP2.
- The "Mouse" subclass means `hidraw` will likely bind to interface 1 without
  fighting the kernel HID driver — mouse boot subclass on a device with no
  pointing-device usages is a no-op for the input subsystem.

## Captured snapshots

Pre-RE baselines (do not delete — these are the "before" reference):

### `lsusb -v -d 04b4:5004` — 2026-05-13

```
Bus 001 Device 003: ID 04b4:5004 Cypress Semiconductor Corp.
Negotiated speed: Full Speed (12Mbps)
Device Descriptor:
  bLength                18
  bDescriptorType         1
  bcdUSB               2.00
  bDeviceClass            0 [unknown]
  bDeviceSubClass         0 [unknown]
  bDeviceProtocol         0
  bMaxPacketSize0         8
  idVendor           0x04b4 Cypress Semiconductor Corp.
  idProduct          0x5004
  bcdDevice            3.04
  iManufacturer           0
  iProduct                0
  iSerial                 0
  bNumConfigurations      1
  Configuration Descriptor:
    bLength                 9
    bDescriptorType         2
    wTotalLength       0x0042
    bNumInterfaces          2
    bConfigurationValue     1
    iConfiguration          2
    bmAttributes         0xa0
      (Bus Powered)
      Remote Wakeup
    MaxPower              100mA
    Interface Descriptor:
      bLength                 9
      bDescriptorType         4
      bInterfaceNumber        0
      bAlternateSetting       0
      bNumEndpoints           1
      bInterfaceClass         3 Human Interface Device
      bInterfaceSubClass      1 Boot Interface Subclass
      bInterfaceProtocol      1 Keyboard
      iInterface              2
        HID Device Descriptor:
          bLength                 9
          bDescriptorType        33
          bcdHID               1.11
          bCountryCode           33 US
          bNumDescriptors         1
          bDescriptorType        34 Report
          wDescriptorLength      63
          Report Descriptors:
            ** UNAVAILABLE **
      Endpoint Descriptor:
        bLength                 7
        bDescriptorType         5
        bEndpointAddress     0x81  EP 1 IN
        bmAttributes            3
          Transfer Type            Interrupt
          Synch Type               None
          Usage Type               Data
        wMaxPacketSize     0x0008  1x 8 bytes
        bInterval               1
    Interface Descriptor:
      bLength                 9
      bDescriptorType         4
      bInterfaceNumber        1
      bAlternateSetting       0
      bNumEndpoints           2
      bInterfaceClass         3 Human Interface Device
      bInterfaceSubClass      1 Boot Interface Subclass
      bInterfaceProtocol      2 Mouse
      iInterface              4
        HID Device Descriptor:
          bLength                 9
          bDescriptorType        33
          bcdHID               1.11
          bCountryCode           33 US
          bNumDescriptors         1
          bDescriptorType        34 Report
          wDescriptorLength     176
          Report Descriptors:
            ** UNAVAILABLE **
      Endpoint Descriptor:
        bLength                 7
        bDescriptorType         5
        bEndpointAddress     0x82  EP 2 IN
        bmAttributes            3
          Transfer Type            Interrupt
          Synch Type               None
          Usage Type               Data
        wMaxPacketSize     0x0040  1x 64 bytes
        bInterval               1
      Endpoint Descriptor:
        bLength                 7
        bDescriptorType         5
        bEndpointAddress     0x03  EP 3 OUT
        bmAttributes            3
          Transfer Type            Interrupt
          Synch Type               None
          Usage Type               Data
        wMaxPacketSize     0x0040  1x 64 bytes
        bInterval               1
```

### `sudo usbhid-dump -d 04b4:5004` — 2026-05-13

#### Interface 0 raw

```
05 01 09 06 A1 01 05 07 19 E0 29 E7 15 00 25 01
75 01 95 08 81 02 95 01 75 08 81 01 95 05 75 01
05 08 19 01 29 05 91 02 95 01 75 03 91 01 95 06
75 08 15 00 25 65 05 07 19 00 29 65 81 00 C0
```

Plain boot-protocol keyboard: modifier byte + reserved + 5-key array,
output LED-state byte. Nothing surprising.

#### Interface 1 raw

```
05 01 09 02 A1 01 85 01 09 01 A1 00 05 09 19 01
29 05 15 00 25 01 75 01 95 05 81 02 75 01 95 03
81 01 05 01 09 30 09 31 09 38 15 81 25 7F 75 08
95 03 81 06 05 0C 0A 38 02 75 08 95 01 81 06 C0
C0 05 01 09 80 A1 01 85 02 19 81 29 83 15 00 25
01 75 01 95 03 81 02 75 01 95 05 81 01 C0 05 0C
09 01 A1 01 85 03 19 00 2A FF 10 15 00 26 FF 10
75 10 95 01 81 00 C0 06 1C FF 09 92 A1 01 85 04
19 00 2A FF 00 15 00 26 FF 00 75 08 95 3F 91 00
19 00 29 FF 81 00 C0 05 01 09 06 A1 01 85 05 05
07 15 00 25 01 19 04 29 70 75 01 95 78 81 02 C0
```

Five report IDs:

| ID  | Type             | Layout                                             |
|-----|------------------|----------------------------------------------------|
| 0x01 | Mouse           | 5 buttons + 3-byte X/Y/Wheel (boot mouse)          |
| 0x02 | System Control  | 3 bits: Power / Sleep / Wake                       |
| 0x03 | Consumer Control| 16-bit usage code (multimedia)                     |
| **0x04** | **Vendor**  | **Usage page 0xFF1C, usage 0x92; 63 B Output + 63 B Input** |
| 0x05 | Keyboard NKRO   | 120-bit bitmap, usage page 7 keys 0x04–0x70        |

### Report ID 0x04 — vendor command channel

Big finding. The vendor protocol lives on Report ID 0x04, interface 1:

```
06 1C FF       Usage Page 0xFF1C  (vendor-defined)
09 92          Usage 0x92
A1 01          Collection (Application)
85 04          Report ID 4
19 00 2A FF 00 Usage Min 0, Usage Max 255
15 00 26 FF 00 Logical Min 0, Logical Max 255
75 08          Report Size 8
95 3F          Report Count 63
91 00          Output (Data, Array, Absolute)
19 00 29 FF    Usage 0..255
81 00          Input  (Data, Array, Absolute)   (same 63x8 layout)
C0             End Collection
```

- **Frame size**: 1 B report ID + 63 B payload = 64 B (matches EP3 OUT
  max packet size exactly).
- **Bidirectional**: same report ID for host→device and device→host.
- **No structure imposed by the descriptor** — vendor used an array of
  generic 8-bit usages, so the actual command framing is whatever the
  firmware decides. We have to recover it from observed traffic.

### Key index space — from `Skins/Main.ini`

Decisive find. The shipped `Skins/Main.ini` (UTF-16LE) inside the wine-
extracted vendor tool enumerates every physical key with a 3-hex-digit
internal index. **Those indices match PS/2 set-1 (XT) scancodes 1-to-1
in the default mapping.** So the on-device remap table is indexed by
XT scancode, and each slot stores the scancode the firmware emits when
that key is pressed. Default = identity.

A few entries from the table:

| Key       | `KEY_xxx` | XT scancode |
|-----------|-----------|------------:|
| Esc       | `KEY_001` | `0x01`      |
| BackSpace | `KEY_00E` | `0x0E`      |
| Enter     | `KEY_01C` | `0x1C`      |
| Left-Ctrl | `KEY_01D` | `0x1D`      |
| Shift     | `KEY_02A` | `0x2A`      |
| Left-Alt  | `KEY_038` | `0x38`      |
| CapsLock  | `KEY_03A` | `0x3A`      |
| ScrollLock| `KEY_046` | `0x46`      |
| Right-Ctrl| `KEY_09D` | extended (`0xE0 0x1D`, stored as `0x9D`) |
| Right-Alt | `KEY_0B8` | extended (`0xE0 0x38`, stored as `0xB8`) |
| Macro     | `KEY_100` | vendor-defined, beyond 1 byte |
| Fn        | `KEY_FN`  | special, no scancode (firmware modifier) |

Full table in the extracted `Skins/Main.ini`.

### The three target keys

User's current (broken) remap, what we need to restore:

| Key       | Slot   | Default | Currently emits         |
|-----------|--------|---------|-------------------------|
| CapsLock  | `0x3A` | `0x3A`  | `0x1D` (Left-Ctrl)      |
| Left-Ctrl | `0x1D` | `0x1D`  | `0x46` (ScrollLock)     |
| Left-Alt  | `0x38` | `0x38`  | `0x01` (Esc)            |

Factory reset = three writes, each setting slot N to value N. LED state
lives in a separate command space; a key-only reset shouldn't touch
LEDs. Confirming the "separate command space" assumption is on the
to-do list (see Open Questions).

### Reference — `macropad_tool` structural template

The user has a Rust HID-reprogrammer at `~/code/rust/macropad_tool/`
targeting a *different* OEM (VID `0x1189`, PIDs `0x884[02]` / `0x8890`).
The protocol differs, but the **structural pattern transfers directly**
when we go to build our own tool. Things worth carrying over:

- **Wire shape**: 65-byte packets `[report_id, magic, ...payload]` via
  libusb interrupt OUT. Our SK2RGB equivalent is 64 bytes:
  `[0x04, opcode, ...62 B payload]` on Report-ID 4 (interface 1, EP3 OUT).
- **Save-to-flash idiom**: macropad's `end_program()` writes
  `[0x03, 0xAA, 0xAA, 0..]` (or `0xA1` in slot 3 if LEDs were also
  touched). The K2 firmware almost certainly has an equivalent commit
  command. **This is the must-find for our protocol writeup** — without
  it, key writes won't survive an unplug.
- **Trait split**: `Messages` (wire-format ops: `read_config`,
  `device_type`, `program_led`, `end_program`) vs `Configuration`
  (semantic ops like `read_macropad_config(layer)`). Clean abstraction
  we should mirror.
- **udev**: macropad ships `80-macropad.rules` granting `MODE="666"`.
  K2 already gets user rw via `uaccess` ACL — no extra udev rule
  needed.
- **Library choice**: macropad uses `rusb` (raw libusb), claims the
  interface, does interrupt OUT directly. We could also use `hidapi-rs`
  for SK2RGB — the K2 is HID-class with a clean Report-ID-4 channel
  and `/dev/hidraw1` is already user-writable. `hidapi` is the simpler
  surface; choice deferred to when we write the tool.

### Strategy (current)

Primary path is now **static RE first**, VM/usbmon as a fallback:

1. Static RE on `SK2RGB-driver.exe` (extracted under wine; see
   `/tmp/sk2rgb/README.md` for unpacking details). The r2 trail below
   is partway in.
2. If r2 yields the per-key remap opcode + the commit-magic, write the
   reset bytes directly to `/dev/hidraw1` (user-writable; uaccess ACL).
3. Fall back to qemu + usbmon only if static RE stalls past the
   command-framing boundary.

Compared to the original plan, this skips the 23 GB WinDev image
extraction and the USB-passthrough fiddling. The VM stack is in
`/tmp/packages.sh` tier 3, ready to install if needed; we're hoping
not to need it.

### r2 trail (parked at `CFwCustomDlg::virtual_372`)

Where to resume static RE on `/tmp/sk2rgb/extracted/SK2RGB-driver.exe`:

| Symbol                       | vaddr        | xref'd by code at |
|------------------------------|-------------:|-------------------|
| str `RequestWrite`           | `0x00648738` | `0x00426428`      |
| str `RequestWriteOne`        | `0x00648728` | `0x00426436`      |
| str `RequestStatusSetFeature`| `0x006486d4` | `0x00426481`      |

All three GetProcAddress lookups happen inside the same method:
`method.CFwCustomDlg.virtual_372` (vaddr `0x00426250`, ~3.9 KB).
Function pointers get stored in `CFwCustomDlg` member fields after the
GetProcAddress chain. Forward xrefs from those storage offsets lead to
every HID-frame-emitting call site.

Open r2 sub-tasks:

- Print full disasm of `virtual_372`; identify the destination offsets
  of each `mov [this+N], eax` after a `GetProcAddress` call. That gives
  us the vtable.
- Pick the offset for `RequestWrite` (or `RequestWriteOne`), use
  `axt @ <addr>` to find callers. Disassemble each caller and identify
  the buffer-fill pattern preceding the call.
- Look for immediate `0x3A`, `0x1D`, `0x38` constants in those callers
  — the three target slots are statically referenced in any factory-
  reset path (if one exists).

## Protocol — decoded from captures

Two pcap captures form the basis: `/tmp/sk2rgb-2026-05-13_145353.pcap`
(partial; LED basics + 4 key remaps + factory reset) and
`/tmp/sk2rgb-2026-05-13_171737.pcap` (comprehensive: per-key LED rotation,
mode/brightness/speed/direction sweeps, 3 profile switches, 9 key-remap
applies, factory reset). Should be rsync'd to corky alongside other
artifacts (see `/tmp/sk2rgb/README.md`).

### Evidence-strength summary

Confidence tag I'll use in the sections below:

- **[V]** verified — derived from the captures with no ambiguity; safe to
  hard-code in the tool.
- **[P]** plausible — inferred from one or two clear observations; safe to
  use but worth a sanity check on first device contact.
- **[?]** weak — best guess based on adjacent evidence; the tool should
  either avoid relying on this until verified, or expose it as a tunable
  with a known-good default.

Overall confidence map:

| Topic                                | Strength | Why |
|--------------------------------------|----------|-----|
| Frame layout                         | **[V]**  | Every frame in both pcaps matches; checksum holds universally |
| Checksum algorithm                   | **[V]**  | Confirmed across hundreds of frames |
| BEGIN/END framing (`op=01/02`)       | **[V]**  | Every transactional change wrapped |
| LED single-key set (`op=11`)         | **[V]**  | Frame-for-frame verified against UI actions |
| LED offsets for F1/F2/W/Caps/A/S/D   | **[V]**  | Direct correlation with user-driven captures |
| Full position→LED-offset map         | **[?]**  | Only 7 of ~104 keys observed; other keys inferable but not captured |
| LED color linearity                  | **[?]**  | One mid-value tested; gamma/clamping not characterized |
| LED mode-id `0x01`=wave              | **[V]**  | Direct |
| LED mode-id `0x04`=reaction          | **[V]**  | Direct |
| LED mode-id `0x05`=custom            | **[V]**  | Direct |
| LED mode-ids for breathing / rainbow / static / off | **[?]** | **Not exercised** in either capture; UI lacked some of these. Values 0x00, 0x02, 0x03 etc. unobserved. |
| Brightness range                     | **[P]**  | 0 and 3 seen; max could be 7, 15, 31, 255 — no idea |
| Speed range                          | **[P]**  | Single value (3) seen; range unknown |
| Direction (0=left, 1=right)          | **[P]**  | 1=right is direct; 0=left inferred by elimination |
| Color-mode (0=single, 1=RGB)         | **[V]**  | Both states observed |
| Trigger (0=keys, 1=grids)            | **[P]**  | Only "→ keys" change captured; the "grids" baseline unmeasured |
| LED effect descriptor block (in op=0a 62-B header) | **[?]** | Structure inferred (8 × 4-byte slots), semantics unknown |
| Key-remap transaction shape          | **[V]**  | Identical structure across all 9 Apply events |
| `02 02 NN` = single-key, XT scancode | **[P]**  | Cross-references with `Skins/Main.ini` indices; not yet diffed cluster-to-cluster to absolutely pin the slot for caps |
| `02 01 NN` pattern                   | **[?]**  | Seen once or twice, semantics not isolated |
| Macro / multi-byte entry layout      | **[?]**  | C33 (caps→media) and C34 (caps→macro abc×7) contain it; not yet decoded |
| `aa 55 ff 02 b4 04 23 08 01 00 00 20` magic prefix | **[V]** | Appears identically across captures; clearly a device/schema stamp |
| `op=03/04` = profile-switch / commit | **[P]**  | 4 pair instances; 3 are expected profile switches; one extra; payload contains profile-list-looking sequence |
| Per-pair profile-id encoding         | **[?]**  | Not yet diffed across the 4 op=04 payloads |
| Startup state-read on EP3 OUT (op=0f/03/07 mix) | **[?]** | Pattern observed but bytes are zeros; the meaningful responses presumably arrive on **EP2 IN which we never filtered** |
| Save-to-flash needed at all          | **[?]**  | Captures show op=03/04 firing on profile boundary and after factory reset — but unclear whether normal Applies need it. Possible the firmware auto-persists. Critical to verify before our tool's first write that intends to survive an unplug. |

### Frame structure (universal)

Every Report-ID-4 frame on EP3 OUT:

```
byte 0     0x04             Report ID
bytes 1-2  16-bit LE        CHECKSUM = sum of bytes 3..62 truncated to u16
byte 3     u8               opcode
byte 4     u8               data length (bytes carried in this frame)
bytes 5-6  16-bit LE        offset (meaning depends on opcode)
bytes 7..  data payload     length bytes of useful content
bytes ..62 zero padding
byte 63    in practice 0    (never observed nonzero)
```

Checksum verified against every frame in both captures. Includes the data
bytes (since most non-zero frames also satisfy the formula).

### Transactions

Almost all updates are wrapped in BEGIN/END:

| Opcode | Length | Offset | Meaning |
|---|---|---|---|
| `0x01` | 0 | 0 | BEGIN_TX |
| `0x02` | 0 | 0 | END_TX |

The two opcodes that fire *outside* transactions are `0x03`/`0x04`
(the save/profile pair) and a few startup probes.

### Single-LED set — `op=0x11`

```
[0x04] [csum] [0x11] [0x03] [off_lo off_hi] [R G B] [zeros...]
```

- `offset` = physical-position-index × 3; the LED buffer is a flat
  3-byte-per-key array indexed by physical position (not by XT scancode).
- `R G B` = 0..255 each. UI accepts decimal 0..255; "80" in the UI
  arrived on the wire as `0x50`. Linearity unverified.

Known LED offsets (more derivable by capturing distinct-color-per-key):

| Key   | Offset | Note |
|-------|--------|------|
| F1    | `0x06` | top row, after Esc + 1 skip |
| F2    | `0x09` | F1 + 3 |
| W     | `0x6f` | row 3 |
| Caps  | `0x9c` | row 4 left edge |
| A     | `0x9f` | Caps + 3 |
| S     | `0xa2` | A + 3 |
| D     | `0xa5` | S + 3 |

#### "Set background" is incremental, not bulk

When the user changes the "background" color in custom mode, the tool only
issues `op=0x11` frames for keys whose color *differs* from the new
background. 7 colored keys → 7 frames, not 104. So the tool keeps a local
model of device state — our Rust tool should too, or at least read it back.

### LED mode/effect — `op=0x06`

```
[0x04] [csum] [0x06] [0x01] [off_lo off_hi] [value] [zeros...]
```

One byte of the effect-register block at each call. Verified offsets:

| Off  | Param           | Values seen |
|------|-----------------|-------------|
| `0x00` | **mode ID**     | `0x01`=wave, `0x04`=reaction, `0x05`=custom |
| `0x01` | **brightness**  | `0x00`=off, `0x03`=non-zero level (range unverified, possibly 0..7) |
| `0x02` | **speed**       | `0x03` (reaction "below half"); range unverified |
| `0x04` | **direction**   | `0x01`=right (`0x00`=left presumed) |
| `0x12` | **color mode**  | `0x00`=single, `0x01`=RGB (reaction-mode-specific) |
| `0x13` | **trigger**     | `0x00`=keys (`0x01`=grids presumed) |

Other mode IDs (rainbow, breathing, static) were not exercised in the
captures — would need a follow-up session OR enumeration by trying values
0x00..0x10 against a known-good device.

Mode and parameter changes apply immediately (no separate Apply); each
tweak is wrapped in its own BEGIN/END.

### Key remap — full table push

Every key-remap Apply pushes the entire key-map plus a config header
in one transaction:

```
op=0x01  BEGIN
op=0x0a  CONFIG_HEADER (16 B for key-only change; 62 B for factory
                        reset, split across 2 frames)
op=0x08  KEYMAP_CHUNK × 9   (56 B each at offsets 0..0x188, total 504 B)
op=0x07  KEYMAP_TAIL × 1    (32 B at offset 0x1c0)
op=0x02  END
```

Total payload: 536 bytes. Layout inside: 3-byte entries indexed by
physical position (same indexing concept as the LED buffer but
1-byte-per-entry-stride internal to each 3-byte slot).

#### Config header — short form (16 B, op=0x0a)

```
aa 55 10 00 00 00 01 00 ...
```

- `aa 55` magic
- `10 00` LE16 total header length (16)
- `00 00 01 00 ...` reserved / format version

#### Config header — full form (62 B, op=0x0a × 2 frames, 56+6 B)

```
aa 55 3e 00 01 00 01 00 <33|17> 00 00 00 01 00 00 00 12 00 08 00
01 04 05 a0  02 04 05 20  02 04 05 a0  02 05 05 20  02 05 05 a0
02 06 05 20  02 06 05 a0  02 07 00 20  02 07 64 00
6f 00 6f 00 64 00
```

- `aa 55 3e 00` magic + LE16 length (62)
- 4-byte entries `01 04 05 a0 / 02 04 05 20 / …` look like **LED effect
  descriptor slots** — possibly per-profile × per-mode (8 entries × 4
  bytes = 32 B fits)
- `33` (second session) vs `17` (first session) at byte 8 — likely the
  active profile or preset index

#### Keymap entry patterns (3 bytes each)

| Pattern | Likely meaning |
|---|---|
| `02 02 NN` | single keypress, NN = PS/2 XT scancode |
| `02 01 NN` | uncommon; possibly alternate usage table |
| `00 00 00` | empty / reserved slot |
| `05 00 00` | seen 6 times at start of chunk — may be "function" or layer markers |
| `03 b6 00`, `03 cd 00`, `03 b5 00` | multi-byte entries late in the table — likely **macro pointers** or **media-key** encodings (caps→media-player capture would isolate this) |

#### Per-cluster diff to isolate the caps slot

C29 (P1 caps→`a`) vs C30 (P1 caps→`b`) in the second pcap are a
minimal-diff pair: only caps's value byte should differ. Compare the
hex byte-by-byte (frame 1437 vs 1497) to pin down the exact slot offset
for caps. **Easy 5-min job left for the tool author** — captures preserved.

#### Trailing config marker

Every keymap push ends its last data frame with:

```
aa 55 ff 02 b4 04 23 08 01 00 00 20 <profile-specific bytes>
```

The 12-byte prefix appears in every full config write and in op=0x04
commit data — a device-identity / schema-version stamp.

### Save / profile-switch — `op=0x03` + `op=0x04`

A 2-frame pair appearing both after factory reset and four times in
isolation in the second pcap (matching the user's three profile
switches plus one extra event).

```
op=0x03  len=0x1c  off=0  data=zeros          REQUEST
op=0x04  len=0x1c  off=0  data=<28 bytes>     DATA
```

28-byte payload of op=0x04 after factory reset:

```
aa 55 ff 02 b4 04 23 08 01 00 00 20 00 00 00 00
01 02 03 04 05 0c 0d 0e
```

Trailing `00 01 02 03 04 05 0c 0d 0e` looks like a list of profile slot
indices (possibly "active profile + valid neighbors").

**Working hypothesis**: this pair is "load profile / commit profile state".
Used at every profile boundary (P1→P2, P2→P3, P3→P1) AND at the end of
factory reset. **Diffing the four op=0x04 payloads in pcap 2 should
isolate the byte encoding destination profile** — another easy follow-up.

### Initialization (tool launch)

Clusters 1-2 of both captures show the tool's startup probe:

- Several `op=0x0f` writes of zeros at offsets `0..0x1e0` — likely
  "clear the staging buffer"
- `op=0x03` with all-zero data — request current device state
- `op=0x07` with 32 bytes at offset `0x1c0` — possibly config-status read

Actual state responses come back on **EP2 IN** which we haven't filtered
yet. A follow-up `tshark -r <pcap> -Y 'usb.endpoint_address.direction == 1
and usb.endpoint_address.number == 2'` should expose them.

### Cluster → action map (second capture, pcap 2026-05-13_171737)

For future cross-reference:

| Cluster | t (s)        | What user did                          |
|---------|--------------|----------------------------------------|
| C1-C2   | 17-25        | Tool startup, auto-state-read          |
| C3      | 37.2         | F1 → LED (fe,11,00)                    |
| C4      | 44.4         | F1 → LED green                         |
| C5      | 52.1         | F1 → LED blue                          |
| C6      | 84.7         | F1 → LED (0x50,0,0) "half red"         |
| C7      | 91.7         | F1 → LED (fe,11,00) (repeat)           |
| C8      | 98.9         | F2 → LED (fe,11,00)                    |
| C9      | 137.0        | Mode → wave                            |
| C10     | 151.8        | Wave → right                           |
| C11     | 178.8        | Brightness changes (4 sub-pulses, all `off=0x01 val=0x03`) |
| C12     | 186.0        | Brightness down (val=0)                |
| C13     | 223.8        | Mode → reaction                        |
| C14     | 243.8        | Reaction speed change                  |
| C15     | 262.7        | Reaction trigger grids → keys          |
| C16     | 292.8        | Reaction color single → RGB            |
| C17     | 300.1        | Reaction color RGB → single            |
| C18     | 312.5        | Mode → custom + bg dark blue           |
| C19     | 373-378      | Background → black (7 keys → 0,0,0)    |
| C20-C22 | 413-486      | Brightness back up + bg restoration    |
| C23     | 503.6        | P1 → P2 (op=03/04 only)                |
| C24     | 558.5        | P2 → P3                                |
| C25     | 566.0        | P3 → P1                                |
| C26     | 575.7        | P2: caps→a Apply (?) (also has op=03/04 inline) |
| C27     | 581.6        | P3: caps→b Apply                       |
| C28     | 585.0        | extra op=03/04                         |
| C29     | 623.3        | P1: caps→a Apply                       |
| C30     | 630.1        | P1: caps→b Apply                       |
| C31     | 664.4        | P1: caps→F8 Apply                      |
| C32     | 701.6        | P1: caps→PgUp Apply                    |
| C33     | 741.4        | P1: caps→media-player Apply            |
| C34     | 808.0        | P1: caps→macro(abc, 7 cycles) Apply    |
| C35     | 855.3        | Factory reset (auto-Apply)             |

## Open questions / unknowns (remaining)

Priority tags:
- **(P0)** blocks first useful tool write — must resolve before sending
  bytes to a real keyboard.
- **(P1)** needed for full feature parity with the vendor tool.
- **(P2)** nice-to-have / future cleanup.

### (P0) Decode the caps slot in the keymap table

C29 (P1 caps→`a`) vs C30 (P1 caps→`b`) is a minimal-diff pair. Diff the
536-byte data across both captures byte by byte; **exactly one byte
should differ** (caps's value). That tells us the slot offset of caps in
the table, which generalizes to every other key (slots are 3 bytes apart,
in physical-position order — same indexing as LED). Without this, our
tool can't safely fix the user's bad capslock/lctrl/lalt remap by
modifying only those slots.

### (P0) Verify save / persistence behavior of `op=0x03/0x04`

Does a normal "key remap Apply" write to flash, or does it just go to
RAM and require the op=03/04 commit to persist? Captures suggest the
pair fires only at profile boundaries and after factory reset — implying
normal Applies *may* auto-persist. **Confirm before our tool's first
intent-to-persist write.** Cheapest test: write one remap, unplug the K2,
replug, see if the change survived.

### (P1) Per-pair profile-id encoding in op=0x04

Diff the four 28-byte op=0x04 payloads in pcap 2 (C23, C24, C25, C28).
The byte(s) that differ encode the target profile (and possibly the
"are we doing a save or a switch" flag).

### (P1) Keymap entry pattern catalog

- What does `02 01 NN` mean?
- What's the macro storage layout? C34 has caps→macro(a,b,c × 7
  cycles, named "dood"). Look in the trailing op=07 chunk and possibly
  the `aa 55 ff …` config block tail for macro byte storage.
- What's the media-key encoding? C33 has caps→media-player; diff
  against any plain-key Apply.
- The leading `05 00 00` ×6 at chunk start — is that a magic header
  inside the chunk, or 6 reserved slots for layer keys / Fn-layer?

### (P1) Full LED-effect descriptor block (32 B of 4-byte entries)

Inside the 62-byte op=0x0a header at factory reset. 8 entries × 4 bytes.
Likely "mode/profile × params" or "per-mode default settings". Decode by:
1. capturing factory-reset (have it)
2. capturing again after changing one mode's parameters
3. diffing — the parameters' bytes should sit in just one of the 8 slots

### (P1) Mode IDs we didn't capture

Wave (0x01), reaction (0x04), and custom (0x05) are confirmed. Probable
missing values: rainbow, breathing, static, "off". Enumerate by trying
values 0x00..0x0F and watching the device's lights. Low-risk if our tool
exposes "set mode" as a CLI subcommand and the user can experiment.

### (P2) Brightness / speed numeric ranges

We saw brightness 0 and 3. Speed 3 once. Probably 0..N where N is small
(7 or 15 — typical for cheap firmware). Establish empirically.

### (P2) Full position → LED-offset map

Currently have 7 of ~104 keys mapped. Easiest harvest: write a tiny
script that sets each key to a unique RGB and dumps the resulting
op=0x11 frames. One capture session yields the entire map.

### (P2) EP2 IN traffic (device → host)

We never filtered for it. The startup state-read on op=0x0f/op=0x03
sends zeros to the device; the real state responses probably arrive on
EP2 IN. Filter:

```bash
tshark -r <pcap> -Y 'usb.endpoint_address.direction == 1 and usb.endpoint_address.number == 2' \
       -T fields -e frame.number -e frame.time_relative -e usbhid.data
```

### (P2) DFU bootloader on a separate PID

Cypress chips sometimes expose a USB DFU bootloader for firmware
recovery — usually on a different PID, sometimes only when a button
combo is held during plug-in. Not relevant for our use case unless we
brick something. Worth knowing as a recovery path.

## Tool design — `cyberpower_tool`

Sketch only. Code not started; gating on having at least the per-key
remap opcode + the commit-magic. Once those land, this turns into
`cargo new` and we move fast.

### Naming + location

- Crate name: `cyberpower_tool`. Generic enough to absorb future
  CyberPower keyboard rebadges; not pinned to one device variant.
- Path: `~/code/rust/cyberpower_tool/`, sibling to `macropad_tool/`.

### Crate layout (mirror `macropad_tool/src/`)

```
src/
  main.rs              CLI (clap derive)
  consts.rs            VID/PID, packet size, timeouts, report ID
  config.rs            on-disk config (yaml or ron), key labels
  keyboard/
    mod.rs             Messages + Configuration traits
    sk2rgb.rs          K2 impl — Report-ID-4 frame composition
                       (future: sibling files for other devices)
  options.rs           CLI option types shared by subcommands
  decoder.rs           response parser for EP2 IN frames
```

### Traits (verbatim from `macropad_tool`)

```rust
pub trait Messages {
    fn read_config(&self, keys: u8, encoders: u8, layer: u8) -> Vec<u8>;
    fn device_type(&self) -> Vec<u8>;
    fn program_led(&self, mode: u8, layer: u8, color: LedColor) -> Vec<u8>;
    fn end_program(&self) -> Vec<u8>;
}

pub trait Configuration {
    fn read_macropad_config(&mut self, layer: &u8) -> Result<Macropad>;
    // …
}
```

For SK2RGB we add `set_key(slot: u8, scancode: u16)` to `Messages`
(macropad uses a layered mapping concept; the K2 doesn't have layers,
just one 256-slot table — different shape, same trait family).

### CLI surface (initial)

```
cyberpower_tool list-keys                          # dump the KEY_xxx table
cyberpower_tool get-key   <slot>                   # query one slot
cyberpower_tool reset-key <slot>                   # write slot → slot
cyberpower_tool set-key   <slot> <scancode>        # arbitrary remap
cyberpower_tool reset-three                        # CapsLock + LCtrl + LAlt
                                                   # (the primary use case)
cyberpower_tool led set    <slot> <rrggbb>
cyberpower_tool led effect <name> [brightness]
cyberpower_tool commit                             # explicit save-to-flash
```

`reset-three` is the user-facing reason this project exists; keep it
as a top-level command, not buried under `key reset --all-modifiers`.

### Library choice

`hidapi-rs`. The K2 is HID-class with a vendor-defined Report ID 4
channel; `hidapi` is the clean API for "open device by VID/PID, send
Report-ID N output, read input". No interface-claim wrestling needed.

`rusb` only if we discover the firmware ignores the report-ID prefix
and wants raw 64-byte interrupt OUT — which would be unusual but not
impossible. Cheap to switch later.

### Bootstrap order, when opcodes land

1. `cargo init`, dependencies: `hidapi`, `clap` (derive), `anyhow`,
   `log`, `env_logger`. Skip serde/ron until config-file work.
2. Implement `device_type()` and a read probe first — pure observation,
   no writes. Confirms we can talk to the K2.
3. Implement `set_key()` + `commit()`. Test against the **three target
   slots only** (`0x1D`, `0x38`, `0x3A`), all writing the slot value
   to itself. Worst case: the user's keyboard is unchanged. Best case:
   keyboard restored.
4. Implement `program_led()` last. LED state is what we're guarding
   against losing; treat it as opt-in via an explicit subcommand.

### Safety rails baked into v1

- **No "write whole table"** subcommand until we've confirmed reads
  give us the existing table to diff against. Easy way to brick a
  remap if the user invokes it before the read path is solid.
- **No silent commit on every write.** Require explicit `commit` (or
  a `--commit` flag on individual ops). Macropad's `end_program()`
  hints that the firmware needs an explicit commit anyway — but until
  we know the K2 doesn't auto-persist per-write, explicit-only is
  the safe default.
- **Dry-run / `--print-only` mode** that emits the bytes that *would*
  be sent without actually writing. Trivially useful while RE'ing the
  last few opcodes.

### Stretch goals (post-v1)

- OpenRGB plugin for the LED side once `program_led` is solid.
- Config-file mode (RON/YAML) like macropad_tool, with `apply` and
  `dump` subcommands.
- Macro recording (the K2 has a `KEY_100`-style Macro slot — punt
  until everything else works).
