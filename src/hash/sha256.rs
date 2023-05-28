use ethnum::U256;

use super::sha256_constants::{
    Indexes::{A_, B_, C_, D_, E_, F_, G_, H_},
    H, K,
};
use crate::crypto_utils::bits::{rotate, split_into_bytes, Direction};
use crate::ternary;

#[cfg(debug_assertions)]
use std::fmt::Binary;

pub struct Input {
    input: Vec<u8>,
    initial_len: u64,
}

impl Input {
    pub fn new(src: &str) -> Self {
        Self {
            input: src.as_bytes().to_vec().to_owned(),
            initial_len: (src.len() * 8) as u64,
        }
    }

    pub fn sha256(&self) -> U256 {
        let pre_process = self.init();

        process(pre_process)
    }

    fn init(&self) -> Vec<u8> {
        let len = self.input.len();
        let mut res = self.input.to_vec();

        let rest = len + (64 - len % 64);
        res.resize(ternary!(rest - len > 8, rest, rest + 64), 0);
        res[len] = 1 << 7;

        let initial_len_bytes = split_into_bytes(self.initial_len);

        let res_len = res.len();
        res[res_len - initial_len_bytes.len()..].copy_from_slice(initial_len_bytes.as_slice());

        return res;
    }
}

fn process(pre_process: Vec<u8>) -> U256 {
    let u = pre_process.chunks(64).fold(H, |v, batch| round(batch, v));

    u.iter().enumerate().fold(U256::from(0u8), |acc, (i, v)| {
        acc | (U256::from(*v) << ((7 - i) * 32))
    })
}

fn round(batch: &[u8], prev_h: [u32; 8]) -> [u32; 8] {
    let mut new_batch: Vec<u32> = batch
        .chunks(4)
        .map(|v| {
            ((v[0] as u32) << 24) | ((v[1] as u32) << 16) | ((v[2] as u32) << 8) | (v[3] as u32)
        })
        .collect();
    new_batch.resize(64, 0);

    for i in 0..48 {
        let s0: u64 = (rotate::<u32>(new_batch[i + 1], 7, Direction::Right)
            ^ rotate::<u32>(new_batch[i + 1], 18, Direction::Right)
            ^ (new_batch[i + 1] >> 3)) as u64;

        let s1: u64 = (rotate::<u32>(new_batch[i + 14], 17, Direction::Right)
            ^ rotate::<u32>(new_batch[i + 14], 19, Direction::Right)
            ^ (new_batch[i + 14] >> 10)) as u64;

        new_batch[i + 16] =
            ((new_batch[i] as u64 + s0 + new_batch[i + 9] as u64 + s1) % 2_u64.pow(32)) as u32;
    }

    let mut curr_h = prev_h;
    for i in 0..64 {
        let s1: u64 = (rotate::<u32>(curr_h[E_ as usize], 6, Direction::Right)
            ^ rotate::<u32>(curr_h[E_ as usize], 11, Direction::Right)
            ^ rotate::<u32>(curr_h[E_ as usize], 25, Direction::Right))
            as u64;

        let ch: u64 = ((curr_h[E_ as usize] & curr_h[F_ as usize])
            ^ (!curr_h[E_ as usize] & curr_h[G_ as usize])) as u64;

        let temp1: u64 = (curr_h[H_ as usize] as u64 + s1 + ch + K[i] as u64 + new_batch[i] as u64)
            % 2_u64.pow(32);

        let s0: u64 = (rotate::<u32>(curr_h[A_ as usize], 2, Direction::Right)
            ^ rotate::<u32>(curr_h[A_ as usize], 13, Direction::Right)
            ^ rotate::<u32>(curr_h[A_ as usize], 22, Direction::Right))
            as u64;

        let maj: u64 = ((curr_h[A_ as usize] & curr_h[B_ as usize])
            ^ (curr_h[A_ as usize] & curr_h[C_ as usize])
            ^ (curr_h[B_ as usize] & curr_h[C_ as usize])) as u64;

        let temp2: u64 = (s0 + maj) as u64 % 2_u64.pow(32);

        curr_h[H_ as usize] = curr_h[G_ as usize];
        curr_h[G_ as usize] = curr_h[F_ as usize];
        curr_h[F_ as usize] = curr_h[E_ as usize];
        curr_h[E_ as usize] = ((curr_h[D_ as usize] as u64 + temp1) as u64 % 2_u64.pow(32)) as u32;
        curr_h[D_ as usize] = curr_h[C_ as usize];
        curr_h[C_ as usize] = curr_h[B_ as usize];
        curr_h[B_ as usize] = curr_h[A_ as usize];
        curr_h[A_ as usize] = ((temp1 + temp2) as u64 % 2_u64.pow(32)) as u32;
    }

    curr_h[0] = ((curr_h[A_ as usize] as u64 + prev_h[0] as u64) % 2_u64.pow(32)) as u32;
    curr_h[1] = ((curr_h[B_ as usize] as u64 + prev_h[1] as u64) % 2_u64.pow(32)) as u32;
    curr_h[2] = ((curr_h[C_ as usize] as u64 + prev_h[2] as u64) % 2_u64.pow(32)) as u32;
    curr_h[3] = ((curr_h[D_ as usize] as u64 + prev_h[3] as u64) % 2_u64.pow(32)) as u32;
    curr_h[4] = ((curr_h[E_ as usize] as u64 + prev_h[4] as u64) % 2_u64.pow(32)) as u32;
    curr_h[5] = ((curr_h[F_ as usize] as u64 + prev_h[5] as u64) % 2_u64.pow(32)) as u32;
    curr_h[6] = ((curr_h[G_ as usize] as u64 + prev_h[6] as u64) % 2_u64.pow(32)) as u32;
    curr_h[7] = ((curr_h[H_ as usize] as u64 + prev_h[7] as u64) % 2_u64.pow(32)) as u32;

    return curr_h;
}

#[cfg(debug_assertions)]
fn debug8<T: Binary>(x: &Vec<T>) {
    x.iter().for_each(|f| print!("{:08b} ", f));
    println!("\nSize: {}", x.len());
}

#[cfg(debug_assertions)]
fn debug32<T: Binary>(x: &Vec<T>) {
    x.iter().for_each(|f| print!("{:032b} ", f));
    println!("\nSize: {}", x.len());
}
