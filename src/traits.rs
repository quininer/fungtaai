pub const KEY_LENGTH: usize = 32;

pub trait Prf {
    // TODO
    //  https://github.com/rust-lang/rust/issues/44247
    //  https://github.com/rust-lang/rust/issues/44168
    // const KEY_LENGTH: usize;
    const BLOCK_LENGTH: usize;

    fn new(key: &[u8; KEY_LENGTH]) -> Self;
    fn prf(&self, data: &mut [u8]);
}

pub trait Hash: Default {
    fn update(&mut self, input: &[u8]);
    fn result(&mut self, output: &mut [u8]);
}

pub trait Time {
    fn now(&self) -> u32;
}
