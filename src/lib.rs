#![no_std]

#![feature(i128_type)]

extern crate byteorder;

pub mod traits;
mod generator;
mod pool;

use core::marker::PhantomData;
use traits::{ Prf, Hash, Time };
use generator::Generator;
use pool::Pool;


pub const POOLS_NUM: usize = 32;

/// 9.5 Accumulator
///
/// The accumulator collects real random data from various sources and uses it to
/// reseed the generator.
pub struct Fortuna<P: Prf, H: Hash, T: Time> {
    // TODO https://github.com/rust-lang/rust/issues/44580
    pool: [Pool<H>; POOLS_NUM],
    generator: Generator<P, H>,
    reseed_cnt: u32,
    _phantom: PhantomData<T>
}

impl<P, H, T> Default for Fortuna<P, H, T>
    where P: Prf, H: Hash, T: Time
{
    /// 9.5.4 Initialization
    ///
    /// Initialization is, as always, a simple function. So far we’ve only talked about
    /// the generator and the accumulator, but the functions we are about to define
    /// are part of the external interface of Fortuna. Their names reflect the fact that
    /// they operate on the whole prng.
    fn default() -> Self {
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
            _phantom: PhantomData
        }
    }
}

impl<P, H, T> Fortuna<P, H, T>
    where P: Prf, H: Hash, T: Time
{
    /// 9.5.5 Getting Random Data
    ///
    /// This is not quite a simple wrapper around the generator component of the
    /// prng, because we have to handle the reseeds here.
    pub fn random_data(&mut self, r: &mut [u8]) {
        unimplemented!()
    }
}
