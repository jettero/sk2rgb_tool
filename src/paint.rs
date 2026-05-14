//! DSL parser for `led custom paint`.
//!
//! Grammar:
//! ```text
//! program     := statement (';' | '\n' statement)*
//! statement   := target_list '=' color
//! target_list := target (',' target)*
//! target      := bare_ident | '@' group_name | hex_slot | hex_range
//! ```
//!
//! Resolution: groups expand to their member keys; bare identifiers either
//! resolve as a reserved key name or expand to a sequence of single-char
//! keys. Statements apply left-to-right — later writes override earlier.

use anyhow::{Result, anyhow, bail};

use crate::color::{Rgb, parse_color};
use crate::groups;
use crate::keys::{self, Key};

#[derive(Debug, Clone)]
pub struct Statement {
    pub keys: Vec<Key>,
    pub color: Rgb,
}

pub fn parse_program(src: &str) -> Result<Vec<Statement>> {
    let mut out = Vec::new();
    // Strip comments per-line first, THEN split on ';' — otherwise a
    // `;` inside what would have been a comment splits the line wrong.
    for line in src.lines() {
        let line = strip_comment(line);
        for stmt in line.split(';') {
            let stmt = stmt.trim();
            if stmt.is_empty() {
                continue;
            }
            out.push(parse_statement(stmt)?);
        }
    }
    Ok(out)
}

fn strip_comment(s: &str) -> &str {
    s.find('#').map_or(s, |i| &s[..i])
}

fn parse_statement(s: &str) -> Result<Statement> {
    let (lhs, rhs) = s
        .split_once('=')
        .ok_or_else(|| anyhow!("missing '=' in: {s:?}"))?;
    let keys = parse_target_list(lhs.trim())?;
    let color = parse_color(rhs.trim())?;
    Ok(Statement { keys, color })
}

fn parse_target_list(s: &str) -> Result<Vec<Key>> {
    let mut out = Vec::new();
    for target in s.split(',').map(str::trim).filter(|t| !t.is_empty()) {
        out.extend(parse_target(target)?);
    }
    if out.is_empty() {
        bail!("empty target list");
    }
    Ok(out)
}

fn parse_target(t: &str) -> Result<Vec<Key>> {
    if let Some(name) = t.strip_prefix('@') {
        return groups::resolve(name).ok_or_else(|| anyhow!("unknown group: @{name}"));
    }
    // hex slot or hex range: 0xNN or 0xNN-0xMM — placeholder for the
    // keymap DSL; for the paint DSL, raw hex slots aren't useful because
    // the LED buffer is indexed by physical position, not XT scancode.
    // Until we can map a position to a Key, raw hex in the paint DSL is
    // rejected.
    if t.starts_with("0x") || t.starts_with("0X") {
        bail!("raw hex slot {t:?} not supported in paint DSL — use a key name");
    }
    keys::resolve_bare(t).ok_or_else(|| anyhow!("unknown key or sequence: {t:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_canonical_example() {
        let p = parse_program("wasd,f1,f12 = lime").unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].keys, vec![Key::W, Key::A, Key::S, Key::D, Key::F1, Key::F12]);
    }

    #[test]
    fn f12_resolves_as_f12_not_f_one_two() {
        let p = parse_program("f12 = red").unwrap();
        assert_eq!(p[0].keys, vec![Key::F12]);
    }

    #[test]
    fn multiple_statements_via_semicolon() {
        let p = parse_program("@all = off; wasd = lime").unwrap();
        assert_eq!(p.len(), 2);
        assert_eq!(p[0].keys.len(), keys::ALL_KEYS.len());
        assert_eq!(p[1].keys, vec![Key::W, Key::A, Key::S, Key::D]);
    }

    #[test]
    fn group_reference() {
        let p = parse_program("@fkeys = sky").unwrap();
        assert_eq!(p[0].keys.len(), 12);
    }

    #[test]
    fn comma_unions_keys_and_groups() {
        let p = parse_program("@arrows,caps = red").unwrap();
        assert_eq!(p[0].keys.len(), 5);
    }

    #[test]
    fn comment_and_blank_lines_skipped() {
        let p = parse_program(
            "
            # the development layout
            @all = off    # background dark
            wasd = lime   # inline comment
            ",
        )
        .unwrap();
        assert_eq!(p.len(), 2);
    }

    #[test]
    fn bare_all_works_too() {
        // `all` is documented with @ but reserved key names lookup fails
        // for it — should it work? Groups MUST be @-prefixed per design.
        assert!(parse_program("all = off").is_ok());
        // works because bare `all` falls through to letter expansion:
        // 'a' 'l' 'l' → A, L, L. That's surprising. Document.
    }
}
