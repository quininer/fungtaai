use ::traits::Hash;


#[derive(Default)]
pub struct Pool<H: Hash> {
    hasher: H,
    pub length: usize
}

impl<H: Hash> Pool<H> {
    pub fn input(&mut self, input: &[u8]) {
        self.hasher.update(input);
        self.length += input.len();
    }

    pub fn output(&mut self, output: &mut [u8]) {
        self.hasher.result(output);
    }

    pub fn reset(&mut self) {
        self.hasher = H::default();
    }
}
