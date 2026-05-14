# sk2rgb_tool — project notes for Claude

## What this is

A Linux tool for the **CyberPowerPC Skorpion K2 RGB** keyboard
(USB `04b4:5004`). Replaces the Windows-only vendor app. Built on the
reverse-engineering captured in `SPEC.md`.

## Read SPEC.md first

`SPEC.md` is the durable design doc. Frame layout, opcode catalog, key /
LED position tables, and the open-question list with `(P0)/(P1)/(P2)`
priority tags all live there.

`/spec` the relevant nouns before assuming. New findings → `/spec
"<fact>"`. Action items → `/todo`.

## Module layout

```
src/
  lib.rs                public re-exports (color, keys, groups, paint, …)
  main.rs               CLI shim — parses options, dispatches to library
  consts.rs             VID/PID, report ID, packet size, opcode table
  options.rs            clap subcommands
  color.rs              Rgb struct, named palette, hex/triplet parser
  keys.rs               Key enum + XT scancodes + LED positions + bare-ident resolver
  groups.rs             @-group definitions
  paint.rs              paint-DSL parser → Vec<Statement>
  keyboard/
    mod.rs              Messages + Configuration traits
    sk2rgb.rs           frame composition (checksum, op=0x11 builder, …)
```

Library (`lib.rs`) is the API surface for any future consumer (OpenRGB
plugin, third-party UI). Binary stays a thin shim — keep it that way.

## Design decisions

- **`hidapi-rs` with `linux-native` backend.** Pure-Rust hidraw, no
  pkg-config, no libudev. K2 is HID-class with a clean Report-ID-4
  channel; `/dev/hidraw1` is user-writable via uaccess.
- **No udev rule shipped.** Modern systemd-udev applies the ACL.
- **Lean dep tree.** `hidapi clap anyhow log env_logger`. Do not add
  `serde`/`ron`/`nom`/`enumset`/`strum` until a concrete feature pulls
  them in.
- **Single 256-slot key table — no layers.** K2 firmware doesn't expose
  layers like macropad_tool's targets do.
- **Mode-aware LED CLI.** Per-key painting only works in mode 0x05
  (custom); `led custom paint` switches mode and writes. Other modes
  (wave/reaction/rainbow/breathing/static) have their own verbs with
  the params that actually apply.
- **Explicit commit only.** Never auto-fire op=0x03/0x04 after a
  write. Persistence semantics unresolved (SPEC P0); surprise commits
  could trash a known-good config.
- **`--dry-run` on every write path.** Prints frames; doesn't send.

## Paint DSL invariants

- Bare identifier resolution: reserved key → single key; else
  all-alnum → char-sequence; else error
- Groups MUST be `@`-prefixed (avoids `caps` vs `c,a,p,s` ambiguity)
- `;` or `\n` separates statements; `#` starts a comment to EOL
- Later statements overwrite earlier — paint background first, then
  overpaint specifics
- Case-insensitive throughout
- Same grammar is intended to be reused for the keymap remap DSL
  (`caps=lctrl; lctrl=caps; lalt=esc`) — keep the parser tidy

## Safety rails for early development

- **(P0) caps-slot decode** is unresolved. Until it is, `set-key`
  against arbitrary slots is unsafe. Only safe write surface is
  `factory-reset` (identity writes for the three known slots) and the
  paint DSL with the 7 known LED positions.
- First writes against a real device should go through `--dry-run`
  first, then a single identity write, then unplug/replug to confirm
  whether normal Applies auto-persist (SPEC P0).
- LED position map is mostly empty (7 of ~104). `Key::led_position()`
  returns `None` for everything else. Until SPEC P2 lands, the paint
  DSL silently skips keys with no known position — TODO to surface
  this as a warning.

## Don't add

- AGENTS.md as a separate file. If we want one, symlink it to
  CLAUDE.md so both names work.
- Backwards-compat shims, fallback paths, or feature flags for things
  the device doesn't do yet. We're pre-alpha; just change the code.
