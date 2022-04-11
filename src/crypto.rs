use magic_crypt::MagicCryptTrait;

pub fn encrypt(text: &str, passwd: &str) -> String {
    let mc = magic_crypt::new_magic_crypt!(passwd, 256);
    let encrypted_base64 = mc.encrypt_str_to_base64(text);
    encrypted_base64
}

pub fn decrypt(encrypted_base64: &str, passwd: &str) -> Option<String> {
    let mc = magic_crypt::new_magic_crypt!(passwd, 256);
    let decrypted_result = mc.decrypt_base64_to_string(encrypted_base64);
    match decrypted_result {
        Ok(decrypted) => Some(decrypted),
        Err(_) => None,
    }
}
