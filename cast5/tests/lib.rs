#![no_std]
extern crate block_cipher_trait;
#[macro_use]
extern crate generic_array;
extern crate cast5;

use block_cipher_trait::BlockCipher;
use cast5::Cast5;
use generic_array::GenericArray;

#[test]
fn single_plaintext_key_ciphertext_sets() {
    // Test based on RFC 2144 Appendix B.1
    // https://tools.ietf.org/html/rfc2144#appendix-B.1
    // 128-bit case

    let key = arr![u8; 0x01, 0x23, 0x45, 0x67, 0x12, 0x34, 0x56, 0x78, 0x23, 0x45, 0x67, 0x89, 0x34, 0x56, 0x78, 0x9A];
    let plain = arr![u8; 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let cipher_expected =
        arr![u8; 0x23, 0x8B, 0x4F, 0xE5, 0x84, 0x7E, 0x44, 0xB2];

    for _ in 1..50 {
        let cast5 = Cast5::new(&key);

        let mut cipher = plain.clone();
        cast5.encrypt_block(&mut cipher);
        assert_eq!(&cipher[..], &cipher_expected[..]);

        let mut decrypted = cipher.clone();
        cast5.decrypt_block(&mut decrypted);

        assert_eq!(&plain[..], &decrypted[..]);
    }
}

#[test]
fn full_maintance_test() {
    // Test based on RFC 2144 Appendix B.2
    // https://tools.ietf.org/html/rfc2144#appendix-B.1

    let mut a = arr![u8; 0x01, 0x23, 0x45, 0x67, 0x12, 0x34, 0x56, 0x78, 0x23, 0x45, 0x67, 0x89, 0x34, 0x56, 0x78, 0x9A];
    let mut b = arr![u8;0x01, 0x23, 0x45, 0x67, 0x12, 0x34, 0x56, 0x78, 0x23, 0x45, 0x67, 0x89, 0x34, 0x56, 0x78, 0x9A];

    let verify_a = arr![u8;0xEE, 0xA9, 0xD0, 0xA2, 0x49, 0xFD, 0x3B, 0xA6, 0xB3, 0x43, 0x6F, 0xB8, 0x9D, 0x6D, 0xCA, 0x92];
    let verify_b = arr![u8; 0xB2, 0xC9, 0x5E, 0xB0, 0x0C, 0x31, 0xAD, 0x71, 0x80, 0xAC, 0x05, 0xB8, 0xE8, 0x3D, 0x69, 0x6E];

    let count = 1_000_000;

    let (al, ar) = a.split_at_mut(8);
    let (bl, br) = b.split_at_mut(8);

    let mut al = GenericArray::from_mut_slice(al);
    let mut ar = GenericArray::from_mut_slice(ar);

    let mut bl = GenericArray::from_mut_slice(bl);
    let mut br = GenericArray::from_mut_slice(br);

    for _ in 0..count {
        let mut k = bl.to_vec();
        k.extend(br.to_vec());
        let c = Cast5::new(&GenericArray::from_slice(&k));
        c.encrypt_block(&mut al);
        c.encrypt_block(&mut ar);

        let mut k = al.to_vec();
        k.extend(ar.to_vec());
        let c = Cast5::new(&GenericArray::from_slice(&k));
        c.encrypt_block(&mut bl);
        c.encrypt_block(&mut br);
    }

    assert_eq!(&al[..], &verify_a[..8]);
    assert_eq!(&ar[..], &verify_a[8..]);

    assert_eq!(&bl[..], &verify_b[..8]);
    assert_eq!(&br[..], &verify_b[8..]);
}