use std::time::Duration;

pub const VENDOR_ID: u16 = 0x04b4;
pub const PRODUCT_ID: u16 = 0x5004;

pub const REPORT_ID: u8 = 0x04;

// EP3 OUT max packet = 64 B. With the hidapi convention, the buffer we
// hand to `write()` is [report_id, ...payload]; payload is 63 B and the
// total wire frame is 64 B.
pub const PACKET_SIZE: usize = 64;
pub const PAYLOAD_SIZE: usize = PACKET_SIZE - 1;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_millis(250);

pub mod opcode {
    pub const BEGIN_TX: u8 = 0x01;
    pub const END_TX: u8 = 0x02;
    pub const SAVE_REQUEST: u8 = 0x03;
    pub const SAVE_DATA: u8 = 0x04;
    pub const MODE_PARAM: u8 = 0x06;
    pub const KEYMAP_TAIL: u8 = 0x07;
    pub const KEYMAP_CHUNK: u8 = 0x08;
    pub const CONFIG_HEADER: u8 = 0x0a;
    pub const STAGING_CLEAR: u8 = 0x0f;
    pub const LED_SET_ONE: u8 = 0x11;
}
