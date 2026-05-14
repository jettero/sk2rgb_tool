# sk2rgb_tool

Linux tool for programming the **CyberPowerPC Skorpion K2 RGB** keyboard
(USB `04b4:5004`, a generic Cypress reference design rebadged by CyberPower
and others). The vendor only ships a Windows `SK2RGB driver.exe`; this is
the Linux replacement.

The immediate reason this exists: restoring a broken keymap where CapsLock,
Left-Ctrl, and Left-Alt got remapped to weird things and the vendor tool
won't run under wine cleanly. See `SPEC.md` for the full reverse-engineering
story; `TODO.md` for what's still in flight.

## Status

**Pre-alpha.** Protocol is documented in `SPEC.md`; the parser/CLI shell is
in place but no command yet writes to a real device — that's gated on a
couple of P0 questions in TODO.md. The paint DSL works under `--dry-run`.

## CLI surface

```
sk2rgb_tool probe
sk2rgb_tool factory-reset                       # keys only, NOT LEDs
sk2rgb_tool set-key <slot-hex> <scancode-hex>

sk2rgb_tool led wave        [--speed N] [--direction left|right]
sk2rgb_tool led reaction    [--speed N] [--color-mode single|rgb] [--trigger keys|grids]
sk2rgb_tool led rainbow
sk2rgb_tool led breathing
sk2rgb_tool led static <color>
sk2rgb_tool led off
sk2rgb_tool led brightness <0-N>

sk2rgb_tool led custom enter
sk2rgb_tool led custom set <key> <color>
sk2rgb_tool led custom all <color>
sk2rgb_tool led custom paint '<DSL>'
sk2rgb_tool led custom paint -f layouts/dev.skin
sk2rgb_tool led custom paint -f -               # read DSL from stdin
```

`--dry-run` works on any write command — emits the bytes that *would* be
sent without touching the device.

## Custom-mode paint DSL

Per-key colors only matter in custom mode (the K2's mode 0x05). The
paint DSL lets you express a full layout in one invocation:

```
@all       = off
wasd       = lime
@arrows    = orange
@fkeys     = sky
lctrl,lalt,caps = red
f11,f12    = blood
qwertyuiop = #1e90ff
```

### Grammar

- `;` or newline separates statements; `#` starts a comment to end-of-line
- `<targets> = <color>` per statement
- targets joined with `,` are unioned
- later statements overwrite earlier ones (so `@all=off` first, then
  overpaint groups/keys)

### Targets

| form              | meaning                                                                                |
|-------------------|----------------------------------------------------------------------------------------|
| `caps`, `f12`, …  | reserved key name → one key                                                            |
| `wasd`, `paul`    | bare letter run, not a reserved name → expand to each char as a key (`p,a,u,l`)        |
| `123`             | bare digit run → `1,2,3`                                                               |
| `@fkeys`          | group reference (`@`-prefixed)                                                         |

Groups: `@all @arrows @fkeys @numrow @numpad @letters @modifiers @nav @qwerty_row @asdf_row @zxcv_row`

### Colors

Named: `red green blue yellow magenta cyan white orange pink purple violet
lime sky ocean lightblue blood brown umber gray/grey black/off`. Or hex:
`#ff8800` / `#f80`. Or triplet: `rgb(255,128,0)` / `255,128,0`.

(The named palette is a first-pass. Once we can see them on the actual
diodes, expect tuning — see TODO.)

## Build

```sh
cargo build --release
```

## Permissions

The K2 gets a `uaccess` ACL automatically (modern systemd-udev), so
`/dev/hidraw1` is user-writable when you're at the seat. No custom udev
rule needed.

## How it talks to the keyboard

HID-class, Report ID 4, EP3 OUT (64-byte interrupt). The "Mouse" interface
on the device is a disguised vendor-command channel. `hidapi-rs` with the
`linux-native` backend (pure-Rust, no libusb/libudev) handles it cleanly.

See `SPEC.md` for the wire format.
