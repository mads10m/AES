use std::vec;

use crate::aes::{cipher, inv_cipher, Key};

mod aes;

fn main() {
    // ecrypt
    let input = "very secret!!!!!".bytes().collect::<Vec<u8>>();
    let key = vec![
        0xCA, 0x97, 0x81, 0x12, 0xCA, 0x1B, 0xBD, 0xCA, 0xFA, 0xC2, 0x31, 0xB3, 0x9A, 0x23, 0xDC,
        0x4D,
    ];
    let key = Key::new(key);

    let out = cipher(input, key);
    //println!("encrypt: {:x?}", out);

    // decrypt
    let input = out;
    let key = vec![
        0xCA, 0x97, 0x81, 0x12, 0xCA, 0x1B, 0xBD, 0xCA, 0xFA, 0xC2, 0x31, 0xB3, 0x9A, 0x23, 0xDC,
        0x4D,
    ];

    let key = Key::new(key);
    let out = inv_cipher(input, key);
    println!("decrypt: {:x?}", out);
    println!(
        "decrypt text: {}",
        out.iter().map(|x| *x as char).collect::<String>()
    );
}
