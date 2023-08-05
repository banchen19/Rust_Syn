use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use crypto::{digest::Digest, sha2::Sha256};
use rand_core::{OsRng, RngCore};

use crate::{var_config::def_Config::DatabaseError, CONFIG_VAR};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

// md5加密——将文本转化为[u8; 16]
pub fn generate_md5_key(text: String) -> [u8; 16] {
    // 将输入文本转换为字节数组
    let name_key = md5::compute(text);
    let iv: String = format!("{:x}", name_key);
    let input_slice = &iv.as_bytes()[..16]; // Ensure the slice is exactly 16 bytes
    let mut result = [0u8; 16];
    result.copy_from_slice(input_slice);
    result
    //[100, 56, 97, 54, 51, 57, 55, 53, 100, 97, 100, 102, 54, 102, 52, 55]
}

/// 加密
pub fn encrypt(plain: &[u8])-> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let config = CONFIG_VAR
        .lock()
        .unwrap()
        .as_ref()
        .expect("CONFIG_VAR not initialized")
        .clone();
    let iv: [u8; 16] = generate_md5_key(config.clone().server_name);

    let key: &[u8; 16] = &generate_md5_key(config.server_name);
    // 创建明文缓冲区并复制明文到其中
    let mut plain_buf: Vec<u8> = Vec::with_capacity(plain.len());
    plain_buf.extend_from_slice(plain);

    // 创建密文缓冲区
    let mut ct_buf: Vec<u8> = vec![0u8; plain.len() + 16]; // 初始容量为明文长度 + 16（iv长度）

    // 进行加密
    let ct = match Aes128CbcEnc::new(key.into(), &iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(&plain_buf, &mut ct_buf)
        {
            Ok(pt) => pt,
        Err(_) => return Err(Box::new(DecryptionError)),
        };

    // 截取加密结果，并转换为 Vec<u8>
    let ct_result = ct[..plain.len()].to_vec();

    Ok(ct_result)
}

#[derive(Debug)]
struct DecryptionError;

impl std::fmt::Display for DecryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decryption error")
    }
}

impl std::error::Error for DecryptionError {}

/// 解密
pub fn decrypt(cipher: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let config = CONFIG_VAR
        .lock()
        .unwrap()
        .as_ref()
        .expect("CONFIG_VAR not initialized")
        .clone();

    let iv: [u8; 16] = generate_md5_key(config.clone().server_name);
    let key: &[u8; 16] = &generate_md5_key(config.server_name);

    // 创建明文缓冲区并复制明文到其中
    let mut plain_buf: Vec<u8> = Vec::with_capacity(cipher.len());
    plain_buf.extend_from_slice(cipher);

    // 创建密文缓冲区
    let mut ct_buf: Vec<u8> = vec![0u8; cipher.len() + 16];

    // 进行解密并处理错误
    let pt = match Aes128CbcDec::new(key.into(), &iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(cipher, &mut ct_buf)
    {
        Ok(pt) => pt,
        Err(_) => return Err(Box::new(DecryptionError)),
    };

    Ok(pt.to_vec())
}
