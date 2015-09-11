use openssl::crypto::symm::{self, Crypter, Mode, Type};
use rand::random;

#[cfg(test)]
mod tests;

/// Encrypt or decrypt an Initialization Vector with AES 256 ECB.
fn crypt_iv(plain_iv: &[u8], key: &[u8], mode: Mode) -> Vec<u8> {
  let iv_crypter = Crypter::new(Type::AES_256_ECB);
  iv_crypter.init(mode, key, "".as_bytes());
  iv_crypter.pad(false); // disable padding
  let mut buffer = iv_crypter.update(plain_iv);
  buffer.extend(iv_crypter.finalize().into_iter());
  buffer
}

fn crypt_message(message: &[u8], key: &[u8], plain_iv: &[u8], mode: Mode) -> Vec<u8> {
  let message_crypter = Crypter::new(Type::AES_256_CBC);
  message_crypter.init(mode, key, plain_iv);
  let mut buffer = message_crypter.update(message);
  buffer.extend(message_crypter.finalize().into_iter());
  buffer
}

fn symmetric_encrypt_with_iv(message: &[u8], key: &[u8], plain_iv: &[u8]) -> Vec<u8> {
  let encrypted_iv = crypt_iv(plain_iv, key, Mode::Encrypt);
  let encrypted_message = crypt_message(message, key, plain_iv, Mode::Encrypt);

  let mut output = encrypted_iv;
  output.extend(encrypted_message.into_iter());
  output
}

/// Generate a random IV and encrypt `input` with it and `key`.
pub fn symmetric_encrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
  let iv: [u8; 16] = random(); // initialization vector

  symmetric_encrypt_with_iv(input, key, &iv)
}

/// Decrypt the IV stored in the first 16 bytes of `input`
/// and use it to decrypt the remaining bytes.
pub fn symmetric_decrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
  let encrypted_iv = &input[0..16];
  let plain_iv = crypt_iv(encrypted_iv, key, Mode::Decrypt);

  let encrypted_message = &input[16..];
  let decrypted_message = crypt_message(encrypted_message, key, &plain_iv, Mode::Decrypt);

  decrypted_message
}
