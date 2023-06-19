use std::vec;

#[derive(Debug, PartialEq)]
pub enum KeyType {
    AES128,
    AES192,
    AES256,
}

#[derive(Debug, PartialEq)]
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

//pub fn expand_key(key: &Vec<u8>) -> Vec<u8> {
//let mut expanded_key = key;
//let mut rcon = 1;
//let mut temp = vec![0; 4];
//let mut i = 0;
//while expanded_key.len() < 176 {
//    for j in 0..4 {
//        temp[j] = expanded_key[j + i - 4];
//    }
//    if i % 16 == 0 {
//        temp = sub_word(rot_word(temp));
//        temp[0] ^= rcon;
//        rcon = rcon << 1;
//    }
//    if key_type == KeyType::AES256 && i % 16 == 16 {
//        temp = sub_word(temp);
//    }
//    for j in 0..4 {
//        expanded_key[i] = expanded_key[i - 16] ^ temp[j];
//        i += 1;
//    }
//}
//expanded_key
//}

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
        assert_eq!(key192.key_type, KeyType::AES192);
        assert_eq!(key256.key_type, KeyType::AES256);
    }

    #[test]
    #[should_panic]
    fn test_create_key_invalid_length() {
        let key: Vec<u8> = vec![0x2b, 0x7e, 0x15];
        let _key = create_key(key);
    }

    //#[test]
    //fn test_expand_key() {
    //    let key = vec![
    //        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
    //        0x4f, 0x3c,
    //    ];
    //    let expanded_key = vec![
    //        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
    //        0x4f, 0x3c, 0xa0, 0xfa, 0xfe, 0x17, 0x88, 0x54, 0x2c, 0xb1, 0x23, 0xa3, 0x39, 0x39,
    //        0x2a, 0x6c, 0x76, 0x05, 0xf2, 0xc2, 0x95, 0xf2, 0x7a, 0x96, 0xb9, 0x43, 0x59, 0x35,
    //        0x80, 0x7a, 0x73, 0x59, 0xf6, 0x7f, 0x3d, 0x80, 0x47, 0x7d, 0x47, 0x16, 0xfe, 0x3e,
    //        0x1e, 0x23, 0x7e, 0x44, 0x6d, 0x7a, 0x88, 0x3b, 0xef, 0x44, 0xa5, 0x41, 0xa8, 0x52,
    //        0x5b, 0x7f,
    //    ];
    //    assert_eq!(expand_key(&key), expanded_key);
    //}
}