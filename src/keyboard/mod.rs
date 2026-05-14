pub mod sk2rgb;

use anyhow::Result;

/// Wire-level operations: each method returns the bytes that go on EP3 OUT
/// (including the leading report-ID byte expected by hidapi).
pub trait Messages {
    fn begin_tx(&self) -> Vec<u8>;
    fn end_tx(&self) -> Vec<u8>;

    /// Set keymap slot `slot` to emit XT scancode `scancode`.
    ///
    /// NOTE: until SPEC.md (P0) "decode the caps slot in the keymap table" is
    /// resolved, callers MUST treat this as unsafe to call against arbitrary
    /// slots. The reset-three path is the only safe write surface for v0.
    fn set_key(&self, slot: u8, scancode: u16) -> Vec<u8>;

    /// LED single-key set (op=0x11). `position` is the physical-position index
    /// (NOT the XT scancode). See SPEC.md "Single-LED set" for the offset
    /// derivation (`position * 3` → byte offset into the LED buffer).
    fn led_set(&self, position: u8, r: u8, g: u8, b: u8) -> Vec<u8>;

    /// Save-to-flash / profile commit pair (op=0x03 then op=0x04). SPEC.md
    /// (P0) "Verify save / persistence behavior" — semantics still partially
    /// unconfirmed; treat as best-effort until validated against a real K2.
    fn commit(&self) -> Vec<u8>;
}

/// Higher-level semantic operations layered on top of `Messages`.
pub trait Configuration {
    fn probe(&mut self) -> Result<()>;
    fn reset_three(&mut self) -> Result<()>;
}
