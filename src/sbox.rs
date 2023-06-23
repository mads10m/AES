use std::vec;

// -------------------- Helper Functions -------------------- //
fn finite_field_mul(mut a: u8, mut b: u8) -> u8 {
    let mut p: u8 = 0;

    let mut counter = 0;
    let mut hi_bit_set;

    while counter < 8 {
        if (b & 1) == 1 {
            p ^= a;
        }

        hi_bit_set = (a & 0x80) as u8;
        a <<= 1;
        if hi_bit_set == 0x80 {
            a ^= 0x1b;
        }

        b >>= 1;
        counter += 1;
    }

    p
}

// S-box and inverse S-box calculations
fn find_multiplicative_inverse(a: u8) -> u8 {
    for b in 0..=255 {
        if finite_field_mul(a, b) == 1 {
            return b;
        }
    }
    return 0x00;
}


fn affine_transformation(byte: u8) -> u8 {
    let ci: u8 = 0x63; // Special byte

    let b0 = (byte >> 0) & 0x01;
    let b1 = (byte >> 1) & 0x01;
    let b2 = (byte >> 2) & 0x01;
    let b3 = (byte >> 3) & 0x01;
    let b4 = (byte >> 4) & 0x01;
    let b5 = (byte >> 5) & 0x01;
    let b6 = (byte >> 6) & 0x01;
    let b7 = (byte >> 7) & 0x01;

    let b0_new = b0 ^ b4 ^ b5 ^ b6 ^ b7 ^ ((ci >> 0) & 0x01);
    let b1_new = b1 ^ b5 ^ b6 ^ b7 ^ b0 ^ ((ci >> 1) & 0x01);
    let b2_new = b2 ^ b6 ^ b7 ^ b0 ^ b1 ^ ((ci >> 2) & 0x01);
    let b3_new = b3 ^ b7 ^ b0 ^ b1 ^ b2 ^ ((ci >> 3) & 0x01);
    let b4_new = b4 ^ b0 ^ b1 ^ b2 ^ b3 ^ ((ci >> 4) & 0x01);
    let b5_new = b5 ^ b1 ^ b2 ^ b3 ^ b4 ^ ((ci >> 5) & 0x01);
    let b6_new = b6 ^ b2 ^ b3 ^ b4 ^ b5 ^ ((ci >> 6) & 0x01);
    let b7_new = b7 ^ b3 ^ b4 ^ b5 ^ b6 ^ ((ci >> 7) & 0x01);

    (b7_new << 7) | (b6_new << 6) | (b5_new << 5) | (b4_new << 4) | (b3_new << 3) | (b2_new << 2) | (b1_new << 1) | b0_new
}

fn inverse_affine_transformation(byte: u8) -> u8 {
    let di: u8 = 0x05; // Spedial byte for inverse S-box

    let s0 = (byte >> 0) & 0x01;
    let s1 = (byte >> 1) & 0x01;
    let s2 = (byte >> 2) & 0x01;
    let s3 = (byte >> 3) & 0x01;
    let s4 = (byte >> 4) & 0x01;
    let s5 = (byte >> 5) & 0x01;
    let s6 = (byte >> 6) & 0x01;
    let s7 = (byte >> 7) & 0x01;

    let b0_new = s2 ^ s5 ^ s7 ^ ((di >> 0) & 0x01);
    let b1_new = s0 ^ s3 ^ s6 ^ ((di >> 1) & 0x01);
    let b2_new = s1 ^ s4 ^ s7 ^ ((di >> 2) & 0x01);
    let b3_new = s0 ^ s2 ^ s5 ^ ((di >> 3) & 0x01);
    let b4_new = s1 ^ s3 ^ s6 ^ ((di >> 4) & 0x01);
    let b5_new = s2 ^ s4 ^ s7 ^ ((di >> 5) & 0x01);
    let b6_new = s0 ^ s3 ^ s5 ^ ((di >> 6) & 0x01);
    let b7_new = s1 ^ s4 ^ s6 ^ ((di >> 7) & 0x01);

    (b7_new << 7) | (b6_new << 6) | (b5_new << 5) | (b4_new << 4) | (b3_new << 3) | (b2_new << 2) | (b1_new << 1) | b0_new
}

// Main
fn main() {
    let mut sbox: Vec<Vec<u8>> = vec![vec![0; 16]; 16];
    let mut inverse_sbox: Vec<Vec<u8>> = vec![vec![0; 16]; 16];

    for y in 0..16 {
        for x in 0..16 {
            let value = (y << 4) | x;

            // s-box
            let inv = find_multiplicative_inverse(value);
            let transformed = affine_transformation(inv);

            // inverse s-box
            let inv_transformed = inverse_affine_transformation(value);
            let inv_inv = find_multiplicative_inverse(inv_transformed);

            sbox[y as usize][x as usize] = transformed;
            inverse_sbox[y as usize][x as usize] = inv_inv;
        }
    }

    println!("S-box:");
    for row in &sbox {
        println!("{:02x?}", row);
    }

    println!("Inverse S-box:");
    for row in &inverse_sbox {
        println!("{:02x?}", row);
    }
}

