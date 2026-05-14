use crate::consts::{PACKET_SIZE, PAYLOAD_SIZE, REPORT_ID, opcode};

/// Compose a single 64-byte Report-ID-4 frame.
///
/// Layout (from SPEC.md "Frame structure (universal)"):
/// ```text
/// byte 0     0x04             Report ID
/// bytes 1-2  16-bit LE        CHECKSUM = sum of bytes 3..62 truncated to u16
/// byte 3     u8               opcode
/// byte 4     u8               data length
/// bytes 5-6  16-bit LE        offset
/// bytes 7..  payload
/// ```
pub fn frame(opcode: u8, offset: u16, data: &[u8]) -> [u8; PACKET_SIZE] {
    assert!(
        data.len() <= PAYLOAD_SIZE - 4,
        "payload too large for one frame"
    );

    let mut f = [0u8; PACKET_SIZE];
    f[0] = REPORT_ID;
    f[3] = opcode;
    f[4] = data.len() as u8;
    f[5..7].copy_from_slice(&offset.to_le_bytes());
    f[7..7 + data.len()].copy_from_slice(data);

    let sum: u16 = f[3..63]
        .iter()
        .fold(0u16, |acc, b| acc.wrapping_add(*b as u16));
    f[1..3].copy_from_slice(&sum.to_le_bytes());
    f
}

pub fn begin_tx() -> [u8; PACKET_SIZE] {
    frame(opcode::BEGIN_TX, 0, &[])
}

pub fn end_tx() -> [u8; PACKET_SIZE] {
    frame(opcode::END_TX, 0, &[])
}

pub fn led_set_one(position: u8, r: u8, g: u8, b: u8) -> [u8; PACKET_SIZE] {
    let offset = (position as u16) * 3;
    frame(opcode::LED_SET_ONE, offset, &[r, g, b])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_covers_bytes_3_through_62() {
        let f = led_set_one(0x02, 0xfe, 0x11, 0x00); // F1 → (fe,11,00), SPEC C3
        let csum = u16::from_le_bytes([f[1], f[2]]);
        let recomputed: u16 = f[3..63].iter().fold(0u16, |a, b| a.wrapping_add(*b as u16));
        assert_eq!(csum, recomputed);
    }

    #[test]
    fn led_offset_is_position_times_three() {
        let f = led_set_one(0x02, 0, 0, 0);
        let off = u16::from_le_bytes([f[5], f[6]]);
        assert_eq!(off, 0x06, "F1 LED offset per SPEC.md is 0x06");
    }
}
