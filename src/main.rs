mod options;

use anyhow::{Result, bail};
use clap::Parser as _;
use log::debug;

use crate::options::{Command, CustomCommand, LedCommand, Options};
use sk2rgb_tool::{device, paint};

fn main() -> Result<()> {
    env_logger::init();
    let opts = Options::parse();
    debug!("options: {opts:?}");

    match opts.command {
        Command::Probe => probe(),
        Command::FactoryReset => bail!("factory-reset: not implemented yet — see TODO.md"),
        Command::SetKey { .. } => bail!("set-key: not implemented yet — see TODO.md"),
        Command::Led(led) => handle_led(led, opts.dry_run),
    }
}

fn handle_led(cmd: LedCommand, dry_run: bool) -> Result<()> {
    match cmd {
        LedCommand::Wave { .. } => bail!("led wave: not implemented yet"),
        LedCommand::Reaction { .. } => bail!("led reaction: not implemented yet"),
        LedCommand::Rainbow => bail!("led rainbow: not implemented yet"),
        LedCommand::Breathing => bail!("led breathing: not implemented yet"),
        LedCommand::Static { .. } => bail!("led static: not implemented yet"),
        LedCommand::Off => bail!("led off: not implemented yet"),
        LedCommand::Brightness { .. } => bail!("led brightness: not implemented yet"),
        LedCommand::Custom(c) => handle_custom(c, dry_run),
    }
}

fn handle_custom(cmd: CustomCommand, dry_run: bool) -> Result<()> {
    match cmd {
        CustomCommand::Enter => bail!("led custom: not implemented yet"),
        CustomCommand::Set { .. } => bail!("led custom set: not implemented yet"),
        CustomCommand::All { .. } => bail!("led custom all: not implemented yet"),
        CustomCommand::Paint { program, file } => paint_cmd(program, file, dry_run),
    }
}

fn paint_cmd(program: Option<String>, file: Option<String>, dry_run: bool) -> Result<()> {
    let src = match (program, file) {
        (Some(p), None) => p,
        (None, Some(path)) => read_program(&path)?,
        (Some(_), Some(_)) => bail!("pass either an inline program OR -f FILE, not both"),
        (None, None) => bail!("no program given; pass an inline string or -f FILE / -f -"),
    };

    let statements = paint::parse_program(&src)?;
    if dry_run {
        for (i, s) in statements.iter().enumerate() {
            println!("stmt {i}: color={:?}", s.color);
            for k in &s.keys {
                let pos = k.led_position();
                println!(
                    "  {} (xt={:?}, led_pos={:?})",
                    k.name(),
                    k.xt_scancode(),
                    pos
                );
            }
        }
        return Ok(());
    }
    bail!("non-dry-run paint not implemented yet — device write path is gated on SPEC P0");
}

fn probe() -> Result<()> {
    let dev = device::Device::open()?;
    let manufacturer = dev.handle.get_manufacturer_string().ok().flatten();
    let product = dev.handle.get_product_string().ok().flatten();
    let serial = dev.handle.get_serial_number_string().ok().flatten();
    println!(
        "opened K2 ({:04x}:{:04x})",
        sk2rgb_tool::consts::VENDOR_ID,
        sk2rgb_tool::consts::PRODUCT_ID
    );
    println!(
        "  manufacturer: {}",
        manufacturer.as_deref().unwrap_or("(none)")
    );
    println!("  product:      {}", product.as_deref().unwrap_or("(none)"));
    println!("  serial:       {}", serial.as_deref().unwrap_or("(none)"));
    Ok(())
}

fn read_program(path: &str) -> Result<String> {
    if path == "-" {
        use std::io::Read as _;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s)?;
        return Ok(s);
    }
    Ok(std::fs::read_to_string(path)?)
}
