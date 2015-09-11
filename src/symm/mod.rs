use openssl::crypto::symm::{self, Crypter, Mode, Type};
use rand::random;

mod tests;

// how to test: use a non-random source of data and compare the result to symmetricEncrypt

/// Symmetric encrypt
/// Generate an IV
pub fn symmetric_encrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
  let iv: [u8; 16] = random(); // initialization vector

  symmetric_encrypt_with_iv(input, key, &iv)
}

fn encrypt_iv(plain_iv: &[u8], key: &[u8]) -> Vec<u8> {
  let iv_crypter = Crypter::new(Type::AES_256_ECB);
  iv_crypter.init(Mode::Encrypt, key, "".as_bytes());
  iv_crypter.pad(false); // disable padding
  let mut buffer = iv_crypter.update(plain_iv);
  buffer.extend(iv_crypter.finalize().into_iter());
  buffer
}

fn encrypt_message(message: &[u8], key: &[u8], plain_iv: &[u8]) -> Vec<u8> {
  symm::encrypt(Type::AES_256_CBC, key, plain_iv, message)
}

fn symmetric_encrypt_with_iv(message: &[u8], key: &[u8], plain_iv: &[u8]) -> Vec<u8> {
  let encrypted_iv = encrypt_iv(plain_iv, key);
  let encrypted_message = encrypt_message(message, key, plain_iv);

  let mut output = encrypted_iv;
  output.extend(encrypted_message.into_iter());
  output

  //let aes_encrypted_iv = symm::encrypt(symm::Type::AES_256_ECB, key, "".as_bytes(), iv);


  /*
  var iv = crypto.randomBytes(16);
  var aesIv = crypto.createCipheriv('aes-256-ecb', key, '');
  aesIv.setAutoPadding(false);
  aesIv.end(iv);

  var aesData = crypto.createCipheriv('aes-256-cbc', key, iv);
  aesData.end(input);

  return Buffer.concat([aesIv.read(), aesData.read()]);
  */

  //unimplemented!()

  // let encrypted_message = {
  //   let message_crypter = Crypter::new(Type::AES_256_CBC);
  //   iv_crypter.init(Mode::Encrypt, key, plain_iv);
  //   let mut buffer = iv_crypter.update(plain_iv);
  //   buffer.extend(iv_crypter.finalize().into_iter());
  //   buffer
  // };
}



pub fn symmetric_decrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
  unimplemented!()
}
