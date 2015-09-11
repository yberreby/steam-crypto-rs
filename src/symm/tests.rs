#![cfg_attr(not(test), allow(unused_imports, dead_code))]
extern crate rustc_serialize;

use super::symmetric_encrypt_with_iv;
use self::rustc_serialize::hex::FromHex;

const PLAIN_IV_HEX: &'static str = "7b2d7972d6eb4142edc26206b224089a";
const KEY_HEX: &'static str = "d10f33200d2372699513478275400073715478ac3794339c8b526433479767de";
const EXPECTED_HEX: &'static str = "c7a7c9f2a5ce77ea3b09ce2b1bb4cd9e502aa78131ef38f4b4fcccc05fb4f734b356e31c9e9a43a86c54a6b6b29b2b2be24358d9bac107d6b5f7a6609a900f80";

#[test]
fn test_symmetric_encrypt_with_iv() {
  let iv = PLAIN_IV_HEX.from_hex().unwrap();
  let key = KEY_HEX.from_hex().unwrap();
  let plaintext = "The quick brown fox jumped over the lazy dog.";

  let expected = EXPECTED_HEX.from_hex().unwrap();
  let encrypted = symmetric_encrypt_with_iv(plaintext.as_bytes(), &key, &iv);

  assert_eq!(encrypted, expected);
}
