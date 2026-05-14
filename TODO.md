# TODO

Pending work. Priority tags mirror SPEC.md: (P0) blocks first useful write,
(P1) feature parity with vendor app, (P2) nice-to-have.

## (P0) RE work that must land before any write hits a real K2

- [ ] **Decode the caps slot in the keymap table** — diff C29 (P1 caps→`a`)
      vs C30 (P1 caps→`b`) in `/tmp/sk2rgb-2026-05-13_171737.pcap`. Exactly
      one byte should differ; that byte's position is caps's slot. Without
      it, only identity remaps (slot N → value N) are safe.
- [ ] **Verify persistence semantics of op=0x03/0x04.** Write one remap,
      unplug, replug, check whether it survived. If yes, commit becomes
      optional polish; if no, every write path must end with the pair.

## (P0) Tool plumbing for the first real write

- [ ] `probe` — open device by VID/PID, dump descriptors, no writes.
      Confirms `hidapi` binding works against the actual K2.
- [ ] **Hidraw open path** + write helpers (`Device::write_frame`,
      `Device::write_transaction`).
- [ ] `factory-reset` — three identity writes for `0x1D`, `0x38`, `0x3A`
      wrapped in BEGIN/END. Skip the commit pair until persistence is
      verified.
- [ ] Wire `--dry-run` through every write path. (Done for `led custom
      paint`; replicate for `factory-reset`, `set-key`, all the `led ...`
      effect-mode verbs.)

## (P1) RE follow-ups (parked, can land in either order)

- [ ] **Per-pair profile-id encoding in op=0x04.** Diff the four 28-byte
      payloads at C23/C24/C25/C28 to isolate the profile-target byte(s).
- [ ] **Keymap entry pattern catalog**:
      - what does `02 01 NN` mean?
      - macro storage layout (C34: caps→macro abc×7 "dood")
      - media-key encoding (C33: caps→media-player)
      - leading `05 00 00 ×6` at chunk start — header or Fn-layer slots?
- [ ] **LED effect descriptor block** — 32 B of 4-byte entries inside the
      62-byte op=0x0a header at factory reset. Capture factory-reset,
      change one mode param, capture again, diff.
- [ ] **Missing mode IDs** — rainbow / breathing / static / off were not
      exercised in either pcap. Enumerate empirically via `led
      <modename>` once the CLI can talk to the device.

## (P1) Tool feature parity once protocol is complete

- [ ] **Reuse the paint DSL grammar for keymap remap.** Same parser
      (statements, comma-unioned targets, `@`-groups, comments), values
      are *target scancodes* instead of colors. e.g. `caps=lctrl;
      lctrl=caps; lalt=esc`.
- [ ] `set-key <slot> <scancode>` — single-key remap (gated on the
      caps-slot decode).
- [ ] `led wave/reaction/rainbow/breathing/static/off` — actual writes.
- [ ] `led brightness <N>` — actual write.
- [ ] `led custom set/all/paint` — actual writes (currently only
      `paint --dry-run` works).
- [ ] `commit` — explicit save-to-flash. Becomes optional or always-on
      depending on the persistence finding above.

## (P2) Range/coverage harvests (post-MVP)

- [ ] Brightness range — only 0 and 3 seen. Probe upper bound.
- [ ] Speed range — only 3 seen.
- [ ] **Full position → LED-offset map.** Currently 7/~104. Easiest
      harvest: write a small CLI subcommand that walks every Key, sends
      a unique RGB via op=0x11, captures the resulting frames, then
      builds the position table. One pcap session yields all entries.
- [ ] **EP2 IN traffic** — never filtered. Startup state-read responses
      live there.

## (P2) Stretch / polish

- [ ] **Config-file mode for macros + persistent keymap layouts.**
      Pulls in `serde` + a format choice (RON probably). `apply
      layouts/dev.skin` / `dump > layouts/current.skin`.
- [ ] **OpenRGB integration — research first, choose path.** Options:
      (a) implement the OpenRGB SDK network protocol ourselves (cleanest
      from a Rust standpoint), (b) C++/Qt plugin that FFIs to our lib
      via cxx, (c) upstream the driver to OpenRGB in C++. Need to
      confirm with `/chai` or the OpenRGB repo whether plugin-side
      device drivers are a supported surface.
- [ ] **Tune named-color RGB values empirically.** Current palette is a
      best-guess "looks like the name at full saturation" set. Once we
      can see them on the actual diodes, adjust per-color. K2 LEDs may
      have a green-heavy white point.
- [ ] **`led palette` subcommand** — flash each named color in turn so
      we can eyeball-tune the values without writing throwaway test
      scripts.
- [ ] Macro recording for the `KEY_100`-style Macro slot.
- [ ] DFU bootloader path documented as a recovery option.
