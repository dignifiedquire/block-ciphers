#![no_std]
#![feature(test)]
extern crate aes_soft;
extern crate test;

use aes_soft::{Aes128, BlockCipher};

#[bench]
pub fn aes128_encrypt(bh: &mut test::Bencher) {
    let cipher = Aes128::new(&Default::default());
    let mut input = Default::default();

    bh.iter(|| {
        cipher.encrypt_block(&mut input);
        test::black_box(&input);
    });
    bh.bytes = input.len() as u64;
}

#[bench]
pub fn aes128_decrypt(bh: &mut test::Bencher) {
    let cipher = Aes128::new(&Default::default());
    let mut input = Default::default();

    bh.iter(|| {
        cipher.decrypt_block(&mut input);
        test::black_box(&input);
    });
    bh.bytes = input.len() as u64;
}

#[bench]
pub fn aes128_encrypt8(bh: &mut test::Bencher) {
    let cipher = Aes128::new(&Default::default());
    let mut input = Default::default();

    bh.iter(|| {
        cipher.encrypt_blocks(&mut input);
        test::black_box(&input);
    });
    bh.bytes = (input[0].len() * input.len()) as u64;
}

#[bench]
pub fn aes128_decrypt8(bh: &mut test::Bencher) {
    let cipher = Aes128::new(&Default::default());
    let mut input = Default::default();

    bh.iter(|| {
        cipher.decrypt_blocks(&mut input);
        test::black_box(&input);
    });
    bh.bytes = (input[0].len() * input.len()) as u64;
}
/*
#[bench]
pub fn ctr_aes128(bh: &mut test::Bencher) {
    let mut cipher = aes::CtrAes128::new(&[0; 16], &[0; 16]);
    let mut input = [0u8; 10000];


    bh.iter(|| {
        cipher.xor(&mut input);
        test::black_box(&input);
    });
    bh.bytes = input.len() as u64;
}
*/
