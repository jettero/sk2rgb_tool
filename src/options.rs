use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub command: Command,

    /// Print the bytes that would be sent without writing to the device.
    #[arg(long, global = true)]
    pub dry_run: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Open the device, read identity, dump anything legible. No writes.
    Probe,

    /// Restore the keymap to factory defaults — every slot writes its
    /// identity scancode. DOES NOT TOUCH LEDS; use the `led` subcommands
    /// for lights.
    FactoryReset,

    /// Set keymap slot `slot` to emit XT scancode `scancode`. Slot and
    /// scancode are hex (e.g. `3a 1d`). UNSAFE for arbitrary slots until
    /// SPEC.md (P0) caps-slot decode is resolved.
    SetKey { slot: String, scancode: String },

    /// LED commands — effect modes and custom-mode painting.
    #[command(subcommand)]
    Led(LedCommand),
}

#[derive(Subcommand, Debug)]
pub enum LedCommand {
    /// Switch to "wave" effect mode (mode 0x01). Per-key color is ignored
    /// in this mode.
    Wave {
        #[arg(long)]
        speed: Option<u8>,
        #[arg(long)]
        direction: Option<Direction>,
    },

    /// Switch to "reaction" effect mode (mode 0x04).
    Reaction {
        #[arg(long)]
        speed: Option<u8>,
        #[arg(long)]
        color_mode: Option<ColorMode>,
        #[arg(long)]
        trigger: Option<Trigger>,
    },

    /// Switch to "rainbow" mode (mode TBD — see SPEC.md "Mode IDs we
    /// didn't capture").
    Rainbow,

    /// Switch to "breathing" mode (mode TBD).
    Breathing,

    /// Switch to "static" mode (mode TBD) at the given color.
    Static { color: String },

    /// Turn LEDs off (brightness 0 or dedicated off mode).
    Off,

    /// Set global brightness. Range likely 0..7 — empirical, SPEC P2.
    Brightness { level: u8 },

    /// Custom-mode operations. Each subcommand implicitly switches the
    /// device to mode 0x05 (custom) before painting.
    #[command(subcommand)]
    Custom(CustomCommand),
}

#[derive(Subcommand, Debug)]
pub enum CustomCommand {
    /// Enter custom mode, keep current per-key buffer.
    Enter,

    /// Paint one key. Color is a name (`red`, `lime`, `off`, …) or
    /// `#rrggbb` / `#rgb` / `rgb(r,g,b)` / `r,g,b`.
    Set { key: String, color: String },

    /// Paint every key the same color.
    All { color: String },

    /// Apply a paint program written in the DSL. Either pass the program
    /// inline (positional arg) or read from a file (`-f FILE`) or stdin
    /// (`-f -`).
    Paint {
        /// Inline program string. Use either this OR `-f`.
        program: Option<String>,

        /// Read program from FILE. Use `-` for stdin.
        #[arg(short = 'f', long)]
        file: Option<String>,
    },
}

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
pub enum ColorMode {
    Single,
    Rgb,
}

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
pub enum Trigger {
    Keys,
    Grids,
}
