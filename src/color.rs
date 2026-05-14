use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub const BLACK: Rgb = Rgb(0, 0, 0);
}

/// First-pass palette. Values picked for "looks like the color name on an
/// RGB LED" rather than ANSI escape sequences. Tune empirically once we
/// can see them on the K2.
pub fn named(name: &str) -> Option<Rgb> {
    Some(match name.to_ascii_lowercase().as_str() {
        "black" | "off" => Rgb(0, 0, 0),
        "white" => Rgb(255, 255, 255),
        "gray" | "grey" => Rgb(128, 128, 128),

        "red" => Rgb(255, 0, 0),
        "green" => Rgb(0, 255, 0),
        "blue" => Rgb(0, 0, 255),
        "yellow" => Rgb(255, 255, 0),
        "magenta" => Rgb(255, 0, 255),
        "cyan" => Rgb(0, 255, 255),

        "orange" => Rgb(255, 96, 0),
        "pink" => Rgb(255, 64, 160),
        "purple" => Rgb(160, 0, 255),
        "violet" => Rgb(200, 80, 255),
        "lime" => Rgb(160, 255, 0),
        "sky" => Rgb(80, 180, 255),
        "ocean" => Rgb(0, 96, 200),
        "lightblue" => Rgb(160, 220, 255),
        "blood" => Rgb(160, 0, 0),
        "brown" => Rgb(120, 64, 0),
        "umber" => Rgb(96, 48, 16),

        _ => return None,
    })
}

/// Parse `red` / `#ff8800` / `#f80` / `rgb(255,128,0)` / `255,128,0`.
pub fn parse_color(s: &str) -> Result<Rgb> {
    let s = s.trim();
    if let Some(rgb) = named(s) {
        return Ok(rgb);
    }
    if let Some(hex) = s.strip_prefix('#') {
        return parse_hex(hex);
    }
    if let Some(inner) = s.strip_prefix("rgb(").and_then(|t| t.strip_suffix(')')) {
        return parse_triplet(inner);
    }
    if s.contains(',') {
        return parse_triplet(s);
    }
    Err(anyhow!("unknown color: {s:?}"))
}

fn parse_hex(h: &str) -> Result<Rgb> {
    let bytes = match h.len() {
        3 => {
            let r = u8::from_str_radix(&h[0..1], 16)?;
            let g = u8::from_str_radix(&h[1..2], 16)?;
            let b = u8::from_str_radix(&h[2..3], 16)?;
            (r * 17, g * 17, b * 17)
        }
        6 => (
            u8::from_str_radix(&h[0..2], 16)?,
            u8::from_str_radix(&h[2..4], 16)?,
            u8::from_str_radix(&h[4..6], 16)?,
        ),
        n => return Err(anyhow!("hex color must be 3 or 6 chars, got {n}")),
    };
    Ok(Rgb(bytes.0, bytes.1, bytes.2))
}

fn parse_triplet(s: &str) -> Result<Rgb> {
    let parts: Vec<&str> = s.split(',').map(str::trim).collect();
    if parts.len() != 3 {
        return Err(anyhow!(
            "rgb triplet needs 3 components, got {}",
            parts.len()
        ));
    }
    Ok(Rgb(parts[0].parse()?, parts[1].parse()?, parts[2].parse()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_colors_resolve() {
        assert_eq!(parse_color("red").unwrap(), Rgb(255, 0, 0));
        assert_eq!(parse_color("OFF").unwrap(), Rgb(0, 0, 0));
        assert_eq!(parse_color("Lime").unwrap(), Rgb(160, 255, 0));
    }

    #[test]
    fn hex_forms() {
        assert_eq!(parse_color("#ff8800").unwrap(), Rgb(0xff, 0x88, 0x00));
        assert_eq!(parse_color("#f80").unwrap(), Rgb(0xff, 0x88, 0x00));
    }

    #[test]
    fn triplet_forms() {
        assert_eq!(parse_color("rgb(10,20,30)").unwrap(), Rgb(10, 20, 30));
        assert_eq!(parse_color("10,20,30").unwrap(), Rgb(10, 20, 30));
    }
}
