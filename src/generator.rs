use core::marker::PhantomData;
use byteorder::{ ByteOrder, LittleEndian };
use ::traits::{ KEY_LENGTH, BLOCK_LENGTH, RESULT_LENGTH, Prf, Hash };
use ::MAX_GENERATE_SIZE;


/// 9.4 The Generator
///
/// The generator is the part that converts a fixed-size state to arbitrarily long
/// outputs. We’ll use an AES-like block cipher for the generator; feel free to
/// choose AES (Rijndael), Serpent, or Twofish for this function. The internal state
/// of the generator consists of a 256-bit block cipher key and a 128-bit counter.
pub struct Generator<P: Prf, H: Hash> {
    key: [u8; KEY_LENGTH],
    ctr: u128,
    _phantom: PhantomData<(P, H)>
}

impl<P, H> Default for Generator<P, H>
    where P: Prf, H: Hash
{
    /// 9.4.1 Initialization
    ///
    /// This is rather simple. We set the key and the counter to zero to indicate that
    /// the generator has not been seeded yet.
    fn default() -> Self {
        const_assert_eq!(KEY_LENGTH, RESULT_LENGTH);

        // Package up the state.
        Generator {
            // Set the key K and counter C to zero.
            key: [0; KEY_LENGTH], ctr: 0,
            _phantom: PhantomData
        }
    }
}

impl<P, H> Generator<P, H>
    where P: Prf, H: Hash
{
    /// 9.4.2 Reseed
    ///
    /// The reseed operation updates the state with an arbitrary input string. At this
    /// level we do not care what this input string contains. To ensure a thorough
    /// mixing of the input with the existing key, we use a hash function.
    pub fn reseed_with<F>(&mut self, f: F)
        where F: FnOnce(&mut H)
    {
        // Compute the new key using a hash function.
        let mut hasher = H::default();
        hasher.update(&self.key);
        f(&mut hasher);
        hasher.result(&mut self.key);

        // Increment the counter to make it nonzero and mark the generator as seeded.
        self.ctr = self.ctr.wrapping_add(1);
    }

    /// 9.4.3 Generate Blocks
    ///
    /// This function generates a number of blocks of random output. This is an
    /// internal function used only by the generator. Any entity outside the prng
    /// should not be able to call this function.
    fn generate_blocks(ctr: &mut u128, prf: &P, r: &mut [u8]) {
        // Append the necessary blocks.
        for chunk in r.chunks_mut(BLOCK_LENGTH) {
            let mut part = [0; BLOCK_LENGTH];
            LittleEndian::write_u128(&mut part, *ctr);
            prf.prf(&mut part);

            // TODO https://github.com/rust-lang/rust/issues/44100
            let len = chunk.len();
            chunk.copy_from_slice(&part[..len]);
            *ctr = ctr.wrapping_add(1);
        }
    }

    /// 9.4.4 Generate Random Data
    ///
    /// This function generates random data at the request of the user of the generator.
    /// It allows for output of up to 2^20 bytes and ensures that the generator forgets
    /// any information about the result it generated.
    pub fn pseudo_random_data(&mut self, r: &mut [u8]) {
        // Limit the output length to reduce the statistical deviation from perfectly random
        // outputs. Also ensure that the length is not negative.
        assert!(r.len() <= MAX_GENERATE_SIZE);
        assert_ne!(self.ctr, 0);

        let prf = P::new(&self.key);

        // Compute the output.
        Self::generate_blocks(&mut self.ctr, &prf, r);

        // Switch to a new key to avoid later compromises of this output.
        Self::generate_blocks(&mut self.ctr, &prf, &mut self.key);
    }
}
