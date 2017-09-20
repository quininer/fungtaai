#![no_std]

#![feature(i128_type, iterator_for_each)]

extern crate byteorder;

pub mod traits;
mod generator;
mod pool;

use traits::{ KEY_LENGTH, Prf, Hash, Time };
use generator::Generator;
use pool::Pool;


pub const POOLS_NUM: usize = 32;
pub const MIN_POOL_SIZE: usize = 64;
pub const MAX_GENERATE_SIZE: usize = 1 << 20;


#[derive(Debug)]
pub enum Error {
    NotSeededYet
}

/// 9.5 Accumulator
///
/// The accumulator collects real random data from various sources and uses it to
/// reseed the generator.
pub struct Fortuna<P: Prf, H: Hash, T: Time> {
    // TODO https://github.com/rust-lang/rust/issues/44580
    pool: [Pool<H>; POOLS_NUM],
    generator: Generator<P, H>,
    reseed_cnt: u32,
    last_reseed_time: u64,
    clock: T
}

impl<P, H, T> Fortuna<P, H, T>
    where P: Prf, H: Hash, T: Time
{
    /// 9.5.4 Initialization
    ///
    /// Initialization is, as always, a simple function. So far we’ve only talked about
    /// the generator and the accumulator, but the functions we are about to define
    /// are part of the external interface of Fortuna. Their names reflect the fact that
    /// they operate on the whole prng.
    pub fn new(clock: T) -> Self {
        macro_rules! array {
            ( $val:expr ; x8  ) => {
                [$val, $val, $val, $val, $val, $val, $val, $val]
            };
            ( $val:expr ; x16 ) => {
                [
                    $val, $val, $val, $val, $val, $val, $val, $val,
                    $val, $val, $val, $val, $val, $val, $val, $val
                ]
            };
            ( $val:expr ; x32 ) => {
                [
                    $val, $val, $val, $val, $val, $val, $val, $val,
                    $val, $val, $val, $val, $val, $val, $val, $val,
                    $val, $val, $val, $val, $val, $val, $val, $val,
                    $val, $val, $val, $val, $val, $val, $val, $val
                ]
            }
        }

        // Package up the state.
        Fortuna {
            // Set the 32 pools to the empty string.
            pool: array![Pool::default(); x32],
            // And initialize the generator.
            generator: Generator::default(),
            // Set the reseed counter to zero.
            reseed_cnt: 0,
            last_reseed_time: 0,
            clock: clock
        }
    }

    /// 9.5.5 Getting Random Data
    ///
    /// This is not quite a simple wrapper around the generator component of the
    /// prng, because we have to handle the reseeds here.
    pub fn random_data(&mut self, r: &mut [u8]) -> Result<(), Error> {
        let now = self.clock.now();
        if self.pool[0].length >= MIN_POOL_SIZE && now > self.last_reseed_time + 100 {
            // We need to reseed.
            self.reseed_cnt += 1;
            self.last_reseed_time = now;

            // Got the data, now do the reseed.
            let pools = &mut self.pool;
            let reseed_cnt = self.reseed_cnt;
            self.generator.reseed_with(|hasher| {
                // Append the hashes of all the pools we will use
                pools.iter_mut()
                    .enumerate()
                    .take_while(|&(i, _)| reseed_cnt % (1 << i) == 0)
                    .for_each(|(_, pool)| {
                        let mut seed = [0; KEY_LENGTH];
                        pool.output(&mut seed);
                        pool.reset();
                        hasher.update(&seed);
                    })
            });
        }

        if self.reseed_cnt == 0 {
            // Generate error, prng not seeded yet
            Err(Error::NotSeededYet)
        } else {
            // Reseeds (if needed) are done. Let the generator that is part of R do the work.
            r.chunks_mut(MAX_GENERATE_SIZE)
                .for_each(|chunk| self.generator.pseudo_random_data(chunk));
            Ok(())
        }
    }

    /// 9.5.6 Add an Event
    ///
    /// Random sources call this routine when they have another random event. Note
    /// that the random sources are each uniquely identified by a source number.
    /// We will not specify how to allocate the source numbers because the solution
    /// depends on the local situation.
    pub fn add_random_event(&mut self, s: u8, i: usize, e: &[u8]) {
        // Check the parameters first.
        assert!(!e.is_empty() && e.len() <= 32);
        assert!(i <= POOLS_NUM);

        // Add the data to the pool.
        self.pool[i].input(&[s, e.len() as u8]);
        self.pool[i].input(e);
    }
}
