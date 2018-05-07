use core::time::Duration;

pub const KEY_LENGTH: usize = 32;
pub const BLOCK_LENGTH: usize = 16;
pub const RESULT_LENGTH: usize = KEY_LENGTH;

pub trait Prf {
    // TODO
    //  https://github.com/rust-lang/rust/issues/44168
    // const KEY_LENGTH: usize;
    // const BLOCK_LENGTH: usize;

    fn new(key: &[u8; KEY_LENGTH]) -> Self;
    fn prf(&self, data: &mut [u8; BLOCK_LENGTH]);
}

pub trait Hash: Default {
    // NOTE RESULT_LENGTH == KEY_LENGTH
    // const RESULT_LENGTH: usize;

    fn update(&mut self, input: &[u8]);
    fn result(self, output: &mut [u8; RESULT_LENGTH]);
}

pub trait Timer {
    /// must be milliseconds time accuracy
    fn elapsed(&self) -> Duration;

    fn reset(&mut self);
}
