extern crate aesni;
extern crate sha2;
extern crate digest;
extern crate fungtaai;

use std::time::{ Instant, Duration };
use std::thread;
use aesni::{ check_aesni, Aes256 };
use sha2::Sha256;
use fungtaai::Fortuna;
use fungtaai::traits::{ KEY_LENGTH, RESULT_LENGTH, BLOCK_LENGTH, Prf, Hash, Timer };


struct InstantTimer(pub Instant);

impl Timer for InstantTimer {
    fn elapsed_ms(&self) -> u64 {
        let dur = self.0.elapsed();
        dur.as_secs() * 1000 + u64::from(dur.subsec_nanos()) / 1_000_000
    }

    fn reset(&mut self) {
        self.0 = Instant::now();
    }
}

struct Aes256Prf(Aes256);

impl Prf for Aes256Prf {
    fn new(key: &[u8; KEY_LENGTH]) -> Self {
        if check_aesni() {
            Aes256Prf(Aes256::new(key))
        } else {
            panic!("no aesni")
        }
    }

    fn prf(&self, data: &mut [u8; BLOCK_LENGTH]) {
        self.0.encrypt(data);
    }
}

#[derive(Default)]
struct Sha256d(Sha256);

impl Hash for Sha256d {
    fn update(&mut self, input: &[u8]) {
        use digest::Input;

        self.0.process(input);
    }

    fn result(self, output: &mut [u8; RESULT_LENGTH]) {
        use digest::{ Input, FixedOutput };

        let mut sha256d = Sha256::default();
        sha256d.process(self.0.fixed_result().as_slice());
        output.copy_from_slice(sha256d.fixed_result().as_slice());
    }
}


#[test]
fn test_vector() {
    // from https://github.com/DaGenix/rust-crypto/blob/master/src/fortuna.rs#L397

    let mut fungtaai: Fortuna<Aes256Prf, Sha256d, _> = Fortuna::new(InstantTimer(Instant::now()));

    let mut output = [0; 100];
    // Expected output from experiments with pycryto
    // Note that this does not match the results for the Go implementation
    // as described at http://www.seehuhn.de/pages/fortuna ... I believe
    // this is because the author there is reusing some Fortuna state from
    // the previous test. These results agree with pycrypto on a fresh slate
    fungtaai.add_random_event(0, 0, &[0; 32]);
    fungtaai.add_random_event(0, 0, &[0; 32]);
    for i in 0..32 {
        fungtaai.add_random_event(1, i, &[1, 2]);
    }

    // from Crypto.Random.Fortuna import FortunaAccumulator
    // x = FortunaAccumulator.FortunaAccumulator()
    // x.add_random_event(0, 0, "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
    // x.add_random_event(0, 0, "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
    // x.add_random_event(1, 0, "\1\2")
    // x.add_random_event(1, 1, "\1\2")
    // print list(bytearray(x.random_data(100)))
    let expected = [ 21,  42, 103, 180, 211,  46, 177, 231, 172, 210,
                    109, 198,  34,  40, 245, 199,  76, 114, 105, 185,
                    186, 112, 183, 213,  19,  72, 186,  26, 182, 211,
                    254,  88,  67, 142, 246, 102,  80,  93, 144, 152,
                    123, 191, 168,  26,  21, 194,  69, 214, 249,  80,
                    182, 165, 203,  69, 134, 140,  11, 208,  50, 175,
                    180, 210, 110, 119,   3,  75,   1,   8,   5, 142,
                    226, 168, 179, 246,  82,  42, 223, 239, 201,  23,
                     28,  30, 195, 195,   9, 154,  31, 172, 209, 232,
                    238, 111,  75, 251, 196,  43, 217, 241,  93, 237];
    fungtaai.random_data(&mut output).unwrap();
    assert_eq!(&expected[..], &output[..]);

    // Immediately (less than 100ms)
    fungtaai.add_random_event(0, 0, &[0; 32]);
    fungtaai.add_random_event(0, 0, &[0; 32]);

    // x.add_random_event(0, 0, "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
    // x.add_random_event(0, 0, "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
    // print list(bytearray(x.random_data(100)))
    let expected = [101, 123, 175, 157, 142, 202, 211,  47, 149, 214,
                    135, 249, 148,  19,  50, 116, 169, 188, 240, 218,
                     91,  62,  35,  44, 142, 108,  95,  20,  37, 185,
                     19, 121, 128, 231, 213,  23,  94, 147,  14,  41,
                    199, 253, 246,  14, 230, 152,  11,  17, 118, 254,
                     96, 251, 171, 115,  66,  21, 196, 164,  82,   6,
                    139, 238, 135,  22, 179,   6,   6, 252, 115,  87,
                     19, 167,  56, 192, 140,  93, 132,  78,  22,  16,
                    114,  68, 123, 200,  37, 183, 163, 224, 201, 155,
                    233,  71, 111,  26,   8, 114, 232, 181,  13,  51];
    fungtaai.random_data(&mut output).unwrap();
    assert_eq!(&expected[..], &output[..]);

    // Simulate more than 100 ms passing
    thread::sleep(Duration::from_millis(200));
    // time.sleep(0.2)
    // print list(bytearray(x.random_data(100)))
    let expected = [ 62, 147, 205, 228,  22,   3, 225, 217, 211, 202,
                     49, 148, 236, 125, 132,  43,  25, 177, 172,  93,
                     98, 177, 112, 160,  76, 101,  60,  98, 225,   9,
                    223, 120, 161,  98, 173, 178,  71,  15,  90, 153,
                     64, 179, 143,  22,  43, 165,  87, 147, 177, 128,
                     21, 105, 214, 197, 224, 187,  22, 139,  16, 153,
                    251,  48, 244,  87,  10, 104, 119, 179,  27, 255,
                     67, 148, 192,  52, 147, 216,  79, 204, 106, 112,
                    238,   0, 239,  99, 159,  96, 184,  90,  54, 122,
                    184, 241, 221, 151, 169,  29, 197,  45,  80,   6];
    fungtaai.random_data(&mut output).unwrap();
    assert_eq!(&expected[..], &output[..]);
}
