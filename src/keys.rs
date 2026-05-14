/// One physical key on the K2.
///
/// The enum is comprehensive enough for a 104-ish full-size layout. Each
/// variant has:
///
/// - `name()` — canonical lowercase identifier used in the paint DSL
/// - `xt_scancode()` — PS/2 set-1 (XT) scancode used in keymap-remap writes;
///   `None` for keys without a single-byte XT code (e.g. firmware-only Fn)
/// - `led_position()` — physical-position index into the LED buffer
///   (offset = position × 3). Mostly `None` for now — see SPEC.md (P2)
///   "Full position → LED-offset map". The 7 we know are baked in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // Top row
    Esc,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrtSc,
    ScrLk,
    Pause,

    // Number row
    Grave,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
    Minus,
    Equals,
    Bksp,

    // QWERTY row
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LBracket,
    RBracket,
    Backslash,

    // ASDF row
    Caps,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Apostrophe,
    Enter,

    // ZXCV row
    LShift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Period,
    Slash,
    RShift,

    // Bottom row
    LCtrl,
    LWin,
    LAlt,
    Space,
    RAlt,
    RWin,
    Menu,
    RCtrl,
    Fn,

    // Nav island
    Ins,
    Home,
    PgUp,
    Del,
    End,
    PgDn,
    Up,
    Down,
    Left,
    Right,

    // Numpad
    NumLock,
    KpDiv,
    KpMul,
    KpSub,
    Kp7,
    Kp8,
    Kp9,
    KpAdd,
    Kp4,
    Kp5,
    Kp6,
    Kp1,
    Kp2,
    Kp3,
    KpEnter,
    Kp0,
    KpDot,
}

impl Key {
    pub fn name(self) -> &'static str {
        use Key::*;
        match self {
            Esc => "esc",
            F1 => "f1",
            F2 => "f2",
            F3 => "f3",
            F4 => "f4",
            F5 => "f5",
            F6 => "f6",
            F7 => "f7",
            F8 => "f8",
            F9 => "f9",
            F10 => "f10",
            F11 => "f11",
            F12 => "f12",
            PrtSc => "prtsc",
            ScrLk => "scrlk",
            Pause => "pause",

            Grave => "grave",
            N1 => "1",
            N2 => "2",
            N3 => "3",
            N4 => "4",
            N5 => "5",
            N6 => "6",
            N7 => "7",
            N8 => "8",
            N9 => "9",
            N0 => "0",
            Minus => "minus",
            Equals => "equals",
            Bksp => "bksp",

            Tab => "tab",
            Q => "q",
            W => "w",
            E => "e",
            R => "r",
            T => "t",
            Y => "y",
            U => "u",
            I => "i",
            O => "o",
            P => "p",
            LBracket => "lbracket",
            RBracket => "rbracket",
            Backslash => "backslash",

            Caps => "caps",
            A => "a",
            S => "s",
            D => "d",
            F => "f",
            G => "g",
            H => "h",
            J => "j",
            K => "k",
            L => "l",
            Semicolon => "semicolon",
            Apostrophe => "apostrophe",
            Enter => "enter",

            LShift => "lshift",
            Z => "z",
            X => "x",
            C => "c",
            V => "v",
            B => "b",
            N => "n",
            M => "m",
            Comma => "comma",
            Period => "period",
            Slash => "slash",
            RShift => "rshift",

            LCtrl => "lctrl",
            LWin => "lwin",
            LAlt => "lalt",
            Space => "space",
            RAlt => "ralt",
            RWin => "rwin",
            Menu => "menu",
            RCtrl => "rctrl",
            Fn => "fn",

            Ins => "ins",
            Home => "home",
            PgUp => "pgup",
            Del => "del",
            End => "end",
            PgDn => "pgdn",
            Up => "up",
            Down => "down",
            Left => "left",
            Right => "right",

            NumLock => "numlock",
            KpDiv => "kpdiv",
            KpMul => "kpmul",
            KpSub => "kpsub",
            Kp7 => "kp7",
            Kp8 => "kp8",
            Kp9 => "kp9",
            KpAdd => "kpadd",
            Kp4 => "kp4",
            Kp5 => "kp5",
            Kp6 => "kp6",
            Kp1 => "kp1",
            Kp2 => "kp2",
            Kp3 => "kp3",
            KpEnter => "kpenter",
            Kp0 => "kp0",
            KpDot => "kpdot",
        }
    }

    pub fn xt_scancode(self) -> Option<u16> {
        use Key::*;
        Some(match self {
            Esc => 0x01,
            N1 => 0x02,
            N2 => 0x03,
            N3 => 0x04,
            N4 => 0x05,
            N5 => 0x06,
            N6 => 0x07,
            N7 => 0x08,
            N8 => 0x09,
            N9 => 0x0a,
            N0 => 0x0b,
            Minus => 0x0c,
            Equals => 0x0d,
            Bksp => 0x0e,
            Tab => 0x0f,
            Q => 0x10,
            W => 0x11,
            E => 0x12,
            R => 0x13,
            T => 0x14,
            Y => 0x15,
            U => 0x16,
            I => 0x17,
            O => 0x18,
            P => 0x19,
            LBracket => 0x1a,
            RBracket => 0x1b,
            Enter => 0x1c,
            LCtrl => 0x1d,
            A => 0x1e,
            S => 0x1f,
            D => 0x20,
            F => 0x21,
            G => 0x22,
            H => 0x23,
            J => 0x24,
            K => 0x25,
            L => 0x26,
            Semicolon => 0x27,
            Apostrophe => 0x28,
            Grave => 0x29,
            LShift => 0x2a,
            Backslash => 0x2b,
            Z => 0x2c,
            X => 0x2d,
            C => 0x2e,
            V => 0x2f,
            B => 0x30,
            N => 0x31,
            M => 0x32,
            Comma => 0x33,
            Period => 0x34,
            Slash => 0x35,
            RShift => 0x36,
            KpMul => 0x37,
            LAlt => 0x38,
            Space => 0x39,
            Caps => 0x3a,
            F1 => 0x3b,
            F2 => 0x3c,
            F3 => 0x3d,
            F4 => 0x3e,
            F5 => 0x3f,
            F6 => 0x40,
            F7 => 0x41,
            F8 => 0x42,
            F9 => 0x43,
            F10 => 0x44,
            NumLock => 0x45,
            ScrLk => 0x46,
            Kp7 => 0x47,
            Kp8 => 0x48,
            Kp9 => 0x49,
            KpSub => 0x4a,
            Kp4 => 0x4b,
            Kp5 => 0x4c,
            Kp6 => 0x4d,
            KpAdd => 0x4e,
            Kp1 => 0x4f,
            Kp2 => 0x50,
            Kp3 => 0x51,
            Kp0 => 0x52,
            KpDot => 0x53,
            F11 => 0x57,
            F12 => 0x58,
            // E0-prefixed extended scancodes stored as 0x80+code in the table:
            KpDiv => 0xb5,
            KpEnter => 0x9c,
            RCtrl => 0x9d,
            RAlt => 0xb8,
            Home => 0xc7,
            Up => 0xc8,
            PgUp => 0xc9,
            Left => 0xcb,
            Right => 0xcd,
            End => 0xcf,
            Down => 0xd0,
            PgDn => 0xd1,
            Ins => 0xd2,
            Del => 0xd3,
            LWin => 0xdb,
            RWin => 0xdc,
            Menu => 0xdd,
            PrtSc => 0xb7,
            Pause => 0xc5,
            Fn => return None, // firmware-only modifier; no scancode
        })
    }

    /// Physical position index into the LED buffer. SPEC.md only has 7
    /// confirmed; everything else returns `None` until SPEC P2 lands.
    pub fn led_position(self) -> Option<u8> {
        use Key::*;
        Some(match self {
            F1 => 0x02,
            F2 => 0x03,
            W => 0x25,
            Caps => 0x34,
            A => 0x35,
            S => 0x36,
            D => 0x37,
            _ => return None,
        })
    }
}

/// Resolve a single bare identifier per the DSL rules:
///
/// 1. Exact name match in the reserved table → single-key vector
/// 2. All-ASCII-alnum and every char individually a key → char-sequence
/// 3. Otherwise → `None`
pub fn resolve_bare(s: &str) -> Option<Vec<Key>> {
    let s = s.to_ascii_lowercase();
    if let Some(k) = lookup(&s) {
        return Some(vec![k]);
    }
    if s.is_empty() || !s.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    let mut out = Vec::with_capacity(s.len());
    for c in s.chars() {
        let one = c.to_string();
        let k = lookup(&one)?;
        out.push(k);
    }
    Some(out)
}

fn lookup(name: &str) -> Option<Key> {
    // Tiny linear table; ~100 entries, called O(n) times during parsing.
    // Skip the HashMap; not worth the constant-factor overhead.
    ALL_KEYS.iter().copied().find(|k| k.name() == name)
}

pub const ALL_KEYS: &[Key] = &[
    Key::Esc,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::F11,
    Key::F12,
    Key::PrtSc,
    Key::ScrLk,
    Key::Pause,
    Key::Grave,
    Key::N1,
    Key::N2,
    Key::N3,
    Key::N4,
    Key::N5,
    Key::N6,
    Key::N7,
    Key::N8,
    Key::N9,
    Key::N0,
    Key::Minus,
    Key::Equals,
    Key::Bksp,
    Key::Tab,
    Key::Q,
    Key::W,
    Key::E,
    Key::R,
    Key::T,
    Key::Y,
    Key::U,
    Key::I,
    Key::O,
    Key::P,
    Key::LBracket,
    Key::RBracket,
    Key::Backslash,
    Key::Caps,
    Key::A,
    Key::S,
    Key::D,
    Key::F,
    Key::G,
    Key::H,
    Key::J,
    Key::K,
    Key::L,
    Key::Semicolon,
    Key::Apostrophe,
    Key::Enter,
    Key::LShift,
    Key::Z,
    Key::X,
    Key::C,
    Key::V,
    Key::B,
    Key::N,
    Key::M,
    Key::Comma,
    Key::Period,
    Key::Slash,
    Key::RShift,
    Key::LCtrl,
    Key::LWin,
    Key::LAlt,
    Key::Space,
    Key::RAlt,
    Key::RWin,
    Key::Menu,
    Key::RCtrl,
    Key::Fn,
    Key::Ins,
    Key::Home,
    Key::PgUp,
    Key::Del,
    Key::End,
    Key::PgDn,
    Key::Up,
    Key::Down,
    Key::Left,
    Key::Right,
    Key::NumLock,
    Key::KpDiv,
    Key::KpMul,
    Key::KpSub,
    Key::Kp7,
    Key::Kp8,
    Key::Kp9,
    Key::KpAdd,
    Key::Kp4,
    Key::Kp5,
    Key::Kp6,
    Key::Kp1,
    Key::Kp2,
    Key::Kp3,
    Key::KpEnter,
    Key::Kp0,
    Key::KpDot,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_key_wins() {
        assert_eq!(resolve_bare("caps"), Some(vec![Key::Caps]));
        assert_eq!(resolve_bare("Caps"), Some(vec![Key::Caps]));
        assert_eq!(resolve_bare("f12"), Some(vec![Key::F12]));
        assert_eq!(resolve_bare("F12"), Some(vec![Key::F12]));
    }

    #[test]
    fn letter_run_expands() {
        assert_eq!(
            resolve_bare("wasd"),
            Some(vec![Key::W, Key::A, Key::S, Key::D])
        );
        assert_eq!(
            resolve_bare("paul"),
            Some(vec![Key::P, Key::A, Key::U, Key::L])
        );
    }

    #[test]
    fn digit_run_expands() {
        assert_eq!(resolve_bare("123"), Some(vec![Key::N1, Key::N2, Key::N3]));
    }

    #[test]
    fn xt_scancodes_match_spec() {
        // The three target keys SPEC.md is built around.
        assert_eq!(Key::Caps.xt_scancode(), Some(0x3a));
        assert_eq!(Key::LCtrl.xt_scancode(), Some(0x1d));
        assert_eq!(Key::LAlt.xt_scancode(), Some(0x38));
    }
}
