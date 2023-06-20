use std::vec;

#[derive(Debug, PartialEq, Clone)]
pub enum KeyType {
    AES128,
    AES192,
    AES256,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Key {
    key: Vec<u8>,
    key_type: KeyType,
}

pub fn create_key(key: Vec<u8>) -> Key {
    let key_type = match key.len() {
        16 => KeyType::AES128,
        24 => KeyType::AES192,
        32 => KeyType::AES256,
        _ => panic!("Invalid key length"),
    };

    return Key {
        key: key,
        key_type: key_type,
    };
}

fn sbox(byte: u8) -> u8 {
    // TODO: calculate sbox
    #[rustfmt::skip]
    let sbox: [[u8; 16]; 16] = [
        [0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76],
        [0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0],
        [0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15],
        [0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75],
        [0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84],
        [0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf],
        [0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8],
        [0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2],
        [0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73],
        [0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb],
        [0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79],
        [0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08],
        [0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a],
        [0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e],
        [0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf],
        [0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16]
    ];

    let x = (byte >> 4) as usize;
    let y = (byte & 0x0f) as usize;

    return sbox[x][y];
}

fn inv_sbox(byte: u8) -> u8 {
    #[rustfmt::skip]
    let inv_sbox: [[u8; 16]; 16] = [
        [0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb],
        [0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb],
        [0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e],
        [0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25],
        [0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92],
        [0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84],
        [0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06],
        [0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b],
        [0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73],
        [0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e],
        [0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b],
        [0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4],
        [0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f],
        [0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef],
        [0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61],
        [0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d],
    ];

    let x = (byte >> 4) as usize;
    let y = (byte & 0x0f) as usize;

    return inv_sbox[x][y];
}

pub fn cipher(input: Vec<u8>, key: Key) -> Vec<u8> {
    // check input length
    // Todo: this check can be moved to the caller
    //if input.len() != 16 {
    //    return Err("input length must be 16 bytes");
    //}

    let nb = 4;
    let nr = match key.key_type {
        KeyType::AES128 => 10,
        KeyType::AES192 => 12,
        KeyType::AES256 => 14,
    };

    // TODO: this can be made more efficient by just using the input
    let mut state = input.clone();

    let w = key_expansion(key);

    state = add_round_key(state, w[0..nb].to_vec());

    for round in 1..nr {
        state = sub_bytes(state);
        state = shift_rows(state);
        state = mix_columns(state);
        state = add_round_key(state, w[(round * nb)..((round + 1) * nb)].to_vec());
    }

    state = sub_bytes(state);
    state = shift_rows(state);
    state = add_round_key(state, w[nr * nb..((nr + 1) * nb)].to_vec());

    return state;
}

fn inv_cipher(input: Vec<u8>, key: Key) -> Vec<u8> {
    let nb = 4;
    let nr = match key.key_type {
        KeyType::AES128 => 10,
        KeyType::AES192 => 12,
        KeyType::AES256 => 14,
    };

    let mut state = input.clone();

    let w = key_expansion(key);

    state = add_round_key(state, w[nr * nb..((nr + 1) * nb)].to_vec());

    for round in (1..nr).rev() {
        state = inv_shift_rows(state);
        state = inv_sub_bytes(state);
        state = add_round_key(state, w[(round * nb)..((round + 1) * nb)].to_vec());
        state = inv_mix_columns(state);
    }

    state = inv_shift_rows(state);
    state = inv_sub_bytes(state);
    state = add_round_key(state, w[0..nb].to_vec());

    return state;
}

fn add_round_key(state: Vec<u8>, w: Vec<u32>) -> Vec<u8> {
    let nb = 4;

    // TODO: this can be made more efficient by just using the input
    let mut result = vec![0; 4 * nb];

    let mut key_schedule_index = 0;
    for i in 0..4 * nb {
        key_schedule_index = match i {
            4 | 8 | 12 => key_schedule_index + 1,
            _ => key_schedule_index,
        };
        // NOTE: you don't need to and with 0xff because the cast to u8 will do that
        result[i] = state[i] ^ (w[key_schedule_index] >> (24 - (i % 4) * 8)) as u8;
    }

    return result;
}

fn sub_bytes(state: Vec<u8>) -> Vec<u8> {
    let mut result = vec![0; state.len()];

    for i in 0..state.len() {
        result[i] = sbox(state[i]);
    }

    return result;
}

fn inv_sub_bytes(state: Vec<u8>) -> Vec<u8> {
    let mut result = vec![0; state.len()];

    for i in 0..state.len() {
        result[i] = inv_sbox(state[i]);
    }

    return result;
}

fn shift_rows(state: Vec<u8>) -> Vec<u8> {
    let nb = 4;

    let mut result = vec![0; 4 * nb];

    let look_up_position: [usize; 16] = [0, 13, 10, 7, 4, 1, 14, 11, 8, 5, 2, 15, 12, 9, 6, 3];

    for i in 0..(4 * nb) {
        result[look_up_position[i]] = state[i];
    }

    return result;

    // TODO: test if this is faster than an array lookup
    //for i in 0..(4 * nb) {
    //    let index = i % 4;
    //    result[(i + ((4 - index) * 4)) % 16] = state[i];
    //}
    //
    //return result;
}

fn inv_shift_rows(state: Vec<u8>) -> Vec<u8> {
    let nb = 4;

    let mut result = vec![0; 4 * nb];

    let look_up_position: [usize; 16] = [0, 5, 10, 15, 4, 9, 14, 3, 8, 13, 2, 7, 12, 1, 6, 11];

    for i in 0..(4 * nb) {
        result[look_up_position[i]] = state[i];
    }

    return result;

    // TODO: test if this is faster than an array lookup
    //for i in 0..(4 * nb) {
    //    let index = i % 4;
    //    result[(i + ((index) * 4)) % 16] = state[i];
    //}
    //
    //return result;
}

fn mix_columns(state: Vec<u8>) -> Vec<u8> {
    let nb = 4;

    let mut result = vec![0; 4 * nb];

    for i in 0..nb {
        let s0 = state[i * 4];
        let s1 = state[i * 4 + 1];
        let s2 = state[i * 4 + 2];
        let s3 = state[i * 4 + 3];

        result[i * 4] = finite_field_mul(0x02, s0) ^ finite_field_mul(0x03, s1) ^ s2 ^ s3;
        result[i * 4 + 1] = s0 ^ finite_field_mul(0x02, s1) ^ finite_field_mul(0x03, s2) ^ s3;
        result[i * 4 + 2] = s0 ^ s1 ^ finite_field_mul(0x02, s2) ^ finite_field_mul(0x03, s3);
        result[i * 4 + 3] = finite_field_mul(0x03, s0) ^ s1 ^ s2 ^ finite_field_mul(0x02, s3);
    }

    return result;
}

fn inv_mix_columns(state: Vec<u8>) -> Vec<u8> {
    let nb = 4;

    let mut result = vec![0; 4 * nb];

    for i in 0..nb {
        let s0 = state[i * 4];
        let s1 = state[i * 4 + 1];
        let s2 = state[i * 4 + 2];
        let s3 = state[i * 4 + 3];

        result[i * 4] = finite_field_mul(0x0e, s0)
            ^ finite_field_mul(0x0b, s1)
            ^ finite_field_mul(0x0d, s2)
            ^ finite_field_mul(0x09, s3);
        result[i * 4 + 1] = finite_field_mul(0x09, s0)
            ^ finite_field_mul(0x0e, s1)
            ^ finite_field_mul(0x0b, s2)
            ^ finite_field_mul(0x0d, s3);
        result[i * 4 + 2] = finite_field_mul(0x0d, s0)
            ^ finite_field_mul(0x09, s1)
            ^ finite_field_mul(0x0e, s2)
            ^ finite_field_mul(0x0b, s3);
        result[i * 4 + 3] = finite_field_mul(0x0b, s0)
            ^ finite_field_mul(0x0d, s1)
            ^ finite_field_mul(0x09, s2)
            ^ finite_field_mul(0x0e, s3);
    }

    return result;
}

// TODO: look more into this
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

    return p;
}

fn sub_word(word: u32) -> u32 {
    let mut result: u32 = 0;

    // TODO: this can be made into an for loop

    let a0 = sbox(((word & 0xff000000) >> 24) as u8);
    let a1 = sbox(((word & 0x00ff0000) >> 16) as u8);
    let a2 = sbox(((word & 0x0000ff00) >> 8) as u8);
    let a3 = sbox((word & 0x000000ff) as u8);

    result |= ((a0 as u32) << 24) | ((a1 as u32) << 16) | ((a2 as u32) << 8) as u32 | a3 as u32;

    return result;
}

fn rot_word(word: u32) -> u32 {
    return (word >> 24) | (word << 8);
}

fn rcon(i: usize) -> u32 {
    // TODO: calculate rcon
    let rcon: [u32; 31] = [
        0x00000000, 0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000,
        0x40000000, 0x80000000, 0x1b000000, 0x36000000, 0x6c000000, 0xd8000000, 0xab000000,
        0x4d000000, 0x9a000000, 0x2f000000, 0x5e000000, 0xbc000000, 0x63000000, 0xc6000000,
        0x97000000, 0x35000000, 0x6a000000, 0xd4000000, 0xb3000000, 0x7d000000, 0xfa000000,
        0xef000000, 0xc5000000, 0x91000000,
    ];

    return rcon[i];
}

pub fn key_expansion(key: Key) -> Vec<u32> {
    let nb = 4;
    let nk = match key.key_type {
        KeyType::AES128 => 4,
        KeyType::AES192 => 6,
        KeyType::AES256 => 8,
    };
    let nr = match key.key_type {
        KeyType::AES128 => 10,
        KeyType::AES192 => 12,
        KeyType::AES256 => 14,
    };

    let mut w = vec![0; nb * (nr + 1)];

    for i in 0..nk {
        w[i] = (key.key[4 * i] as u32) << 24
            | (key.key[4 * i + 1] as u32) << 16
            | (key.key[4 * i + 2] as u32) << 8
            | (key.key[4 * i + 3] as u32);
    }

    for i in nk..nb * (nr + 1) {
        let mut temp = w[i - 1];

        if i % nk == 0 {
            temp = sub_word(rot_word(temp)) ^ (rcon(i / nk));
        } else if (nk > 6) && (i % nk == 4) {
            temp = sub_word(temp);
        }

        w[i] = (w[i - nk]) ^ (temp);
    }

    return w;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_key() {
        let key128: Vec<u8> = vec![
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];

        let key192: Vec<u8> = vec![
            0x8e, 0x73, 0xb0, 0xf7, 0xda, 0x0e, 0x64, 0x52, 0xc8, 0x10, 0xf3, 0x2b, 0x80, 0x90,
            0x79, 0xe5, 0x62, 0xf8, 0xea, 0xd2, 0x52, 0x2c, 0x6b, 0x7b,
        ];

        let key256: Vec<u8> = vec![
            0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d,
            0x77, 0x81, 0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3,
            0x09, 0x14, 0xdf, 0xf4,
        ];

        let key128 = create_key(key128);
        let key192 = create_key(key192);
        let key256 = create_key(key256);

        assert_eq!(key128.key_type, KeyType::AES128);
        assert_eq!(key128.key.len(), 16);

        assert_eq!(key192.key_type, KeyType::AES192);
        assert_eq!(key192.key.len(), 24);

        assert_eq!(key256.key_type, KeyType::AES256);
        assert_eq!(key256.key.len(), 32);
    }

    #[test]
    #[should_panic]
    fn test_create_key_invalid_length() {
        let key: Vec<u8> = vec![0x2b, 0x7e, 0x15];
        let _key = create_key(key);
    }

    #[test]
    fn test_sbox() {
        assert_eq!(sbox(0x53), 0xed);
        assert_eq!(sbox(0xff), 0x16);
        assert_eq!(sbox(0xf0), 0x8c);
        assert_eq!(sbox(0x0f), 0x76);
        assert_eq!(sbox(0x00), 0x63);
    }

    #[test]
    fn test_inv_sbox() {
        assert_eq!(inv_sbox(0x53), 0x50);
        assert_eq!(inv_sbox(0xff), 0x7d);
        assert_eq!(inv_sbox(0xf0), 0x17);
        assert_eq!(inv_sbox(0x0f), 0xfb);
        assert_eq!(inv_sbox(0x00), 0x52);
    }

    #[test]
    fn test_sub_word() {
        assert_eq!(sub_word(0x5355fcb0), 0xedfcb0e7);
        assert_eq!(sub_word(0x00000000), 0x63636363);
        assert_eq!(sub_word(0xffffffff), 0x16161616);
    }

    // TODO
    //#[test]
    //fn test_key_expansion() {
    //    let key128: Vec<u8> = vec![
    //        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
    //        0x4f, 0x3c,
    //    ];

    //    let key192: Vec<u8> = vec![
    //        0x8e, 0x73, 0xb0, 0xf7, 0xda, 0x0e, 0x64, 0x52, 0xc8, 0x10, 0xf3, 0x2b, 0x80, 0x90,
    //        0x79, 0xe5, 0x62, 0xf8, 0xea, 0xd2, 0x52, 0x2c, 0x6b, 0x7b,
    //    ];

    //    let key256: Vec<u8> = vec![
    //        0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d,
    //        0x77, 0x81, 0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3,
    //        0x09, 0x14, 0xdf, 0xf4,
    //    ];

    //    let key128 = create_key(key128);
    //    let key192 = create_key(key192);
    //    let key256 = create_key(key256);

    //    let key128_expanded = key_expansion(key128);
    //}

    #[test]
    fn test_cipher_encrypt() {
        let key128 = create_key(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ]);

        let key192 = create_key(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        ]);

        let key256 = create_key(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ]);

        let correct_output_128 = vec![
            0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4,
            0xc5, 0x5a,
        ];

        let correct_output_192 = vec![
            0xdd, 0xa9, 0x7c, 0xa4, 0x86, 0x4c, 0xdf, 0xe0, 0x6e, 0xaf, 0x70, 0xa0, 0xec, 0x0d,
            0x71, 0x91,
        ];

        let correct_output_256 = vec![
            0x8e, 0xa2, 0xb7, 0xca, 0x51, 0x67, 0x45, 0xbf, 0xea, 0xfc, 0x49, 0x90, 0x4b, 0x49,
            0x60, 0x89,
        ];

        let input = vec![
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];

        assert_eq!(cipher(input.clone(), key128), correct_output_128);
        assert_eq!(cipher(input.clone(), key192), correct_output_192);
        assert_eq!(cipher(input, key256), correct_output_256);
    }

    #[test]
    fn test_cipher_decrypt() {
        let key128 = create_key(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ]);

        let key192 = create_key(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        ]);

        let key256 = create_key(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ]);

        let input128 = vec![
            0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4,
            0xc5, 0x5a,
        ];
        let input192 = vec![
            0xdd, 0xa9, 0x7c, 0xa4, 0x86, 0x4c, 0xdf, 0xe0, 0x6e, 0xaf, 0x70, 0xa0, 0xec, 0x0d,
            0x71, 0x91,
        ];

        let input256 = vec![
            0x8e, 0xa2, 0xb7, 0xca, 0x51, 0x67, 0x45, 0xbf, 0xea, 0xfc, 0x49, 0x90, 0x4b, 0x49,
            0x60, 0x89,
        ];

        let output = vec![
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];

        assert_eq!(inv_cipher(input128, key128), output);
        assert_eq!(inv_cipher(input192, key192), output);
        assert_eq!(inv_cipher(input256, key256), output);
    }

    #[test]
    fn test_cipher() {
        let key128 = create_key(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ]);

        let key192 = create_key(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        ]);

        let key256 = create_key(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ]);

        let input = vec![
            0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0xff, 0xff,
            0xff, 0xff,
        ];

        assert_eq!(
            inv_cipher(cipher(input.clone(), key128.clone()), key128),
            input
        );

        assert_eq!(
            inv_cipher(cipher(input.clone(), key192.clone()), key192),
            input
        );

        assert_eq!(
            inv_cipher(cipher(input.clone(), key256.clone()), key256),
            input
        );
    }

    #[test]
    fn test_shift_rows() {
        let state = vec![
            0x00, 0x10, 0x20, 0x30, 0x01, 0x11, 0x21, 0x31, 0x02, 0x12, 0x22, 0x32, 0x03, 0x13,
            0x23, 0x33,
        ];

        let new_state = vec![
            0x00, 0x11, 0x22, 0x33, 0x01, 0x12, 0x23, 0x30, 0x02, 0x13, 0x20, 0x31, 0x03, 0x10,
            0x21, 0x32,
        ];

        assert_eq!(shift_rows(state), new_state);
    }

    #[test]
    fn test_inv_shift_rows() {
        // inverse of test_shift_rows
        let state = vec![
            0x00, 0x11, 0x22, 0x33, 0x01, 0x12, 0x23, 0x30, 0x02, 0x13, 0x20, 0x31, 0x03, 0x10,
            0x21, 0x32,
        ];

        let new_state = vec![
            0x00, 0x10, 0x20, 0x30, 0x01, 0x11, 0x21, 0x31, 0x02, 0x12, 0x22, 0x32, 0x03, 0x13,
            0x23, 0x33,
        ];

        assert_eq!(inv_shift_rows(state), new_state);

        let state = vec![
            0x00, 0x10, 0x20, 0x30, 0x01, 0x11, 0x21, 0x31, 0x02, 0x12, 0x22, 0x32, 0x03, 0x13,
            0x23, 0x33,
        ];

        let new_state = vec![
            0x00, 0x13, 0x22, 0x31, 0x01, 0x10, 0x23, 0x32, 0x02, 0x11, 0x20, 0x33, 0x03, 0x12,
            0x21, 0x30,
        ];

        assert_eq!(inv_shift_rows(state), new_state);
    }
}
