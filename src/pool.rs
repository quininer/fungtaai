use ::traits::{ KEY_LENGTH, Hash };


#[derive(Default)]
pub struct Pool<H: Hash> {
    hasher: H,
    pub length: usize
}

impl<H: Hash> Pool<H> {
    #[inline]
    pub fn input(&mut self, input: &[u8]) {
        self.hasher.update(input);
        self.length += input.len();
    }

    #[inline]
    pub fn output(&mut self, output: &mut [u8; KEY_LENGTH]) {
        self.hasher.result(output);
    }

    #[inline]
    pub fn reset(&mut self) {
        self.hasher = H::default();
    }
}
