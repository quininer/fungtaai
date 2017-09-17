pub const KEY_LEN: usize = 32;

pub trait Prf {
    const BLOCK_LENGTH: usize;

    fn new(key: &[u8; KEY_LEN]) -> Self;
    fn prf(&self, data: &mut [u8]);
}

pub trait Hash: Default {
    fn input(&mut self, input: &[u8]);
    fn output(&mut self, output: &mut [u8]);
}
