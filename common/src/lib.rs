use crypto::digest::Digest;
use crypto::md5::Md5;

pub mod db;
pub mod res;
pub mod entity;
pub mod dto;

// 使用md5加密
pub fn encrypt_password(password: String) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(password.as_str());
    hasher.result_str()
}
