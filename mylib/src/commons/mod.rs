use openssl::hash::{Hasher, MessageDigest};

pub fn get_hash(buffer: &[u8]) -> String {
    let mut h = Hasher::new(MessageDigest::md5()).unwrap();
    h.update(buffer).unwrap();
    hex::encode(h.finish().unwrap())
}