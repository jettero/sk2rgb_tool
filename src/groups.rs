use crate::keys::{ALL_KEYS, Key};

/// `@`-prefixed group names. Resolution is case-insensitive.
pub fn resolve(name: &str) -> Option<Vec<Key>> {
    use Key::*;
    let name = name.to_ascii_lowercase();
    Some(match name.as_str() {
        "all" => ALL_KEYS.to_vec(),

        "fkeys" => vec![F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12],
        "arrows" => vec![Up, Down, Left, Right],
        "modifiers" => vec![LCtrl, LShift, LAlt, LWin, RCtrl, RShift, RAlt, RWin, Caps, Fn, Menu],
        "nav" => vec![Ins, Home, PgUp, Del, End, PgDn],

        "letters" => (b'a'..=b'z')
            .map(|c| crate::keys::resolve_bare(&(c as char).to_string()).unwrap()[0])
            .collect(),

        "numrow" => vec![N1, N2, N3, N4, N5, N6, N7, N8, N9, N0, Minus, Equals],
        "numpad" => vec![
            NumLock, KpDiv, KpMul, KpSub,
            Kp7, Kp8, Kp9, KpAdd,
            Kp4, Kp5, Kp6,
            Kp1, Kp2, Kp3, KpEnter,
            Kp0, KpDot,
        ],

        "qwerty_row" => vec![Q, W, E, R, T, Y, U, I, O, P],
        "asdf_row" => vec![A, S, D, F, G, H, J, K, L],
        "zxcv_row" => vec![Z, X, C, V, B, N, M],

        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fkeys_has_twelve() {
        assert_eq!(resolve("fkeys").unwrap().len(), 12);
        assert_eq!(resolve("FKEYS").unwrap().len(), 12);
    }

    #[test]
    fn letters_has_26() {
        assert_eq!(resolve("letters").unwrap().len(), 26);
    }

    #[test]
    fn unknown_group_is_none() {
        assert!(resolve("bogus").is_none());
    }
}
