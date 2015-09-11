extern crate openssl;
extern crate rustc_serialize;
use self::rustc_serialize::base64::FromBase64;
use self::openssl::crypto::pkey::PKey;

pub const STEAM_PUBLIC_KEY_BASE64_PAYLOAD_STR: &'static str = include_str!("../steam_payload.pub");


fn main() {
  let keydata: Vec<u8> = STEAM_PUBLIC_KEY_BASE64_PAYLOAD_STR.from_base64().unwrap();
  let data: Vec<u8> = "ltkU21V29klPGMMJIfj7o6XVOGhWkt7imm1QdCu+5kwQL0m7n4lVw5jLlbDOjltcWKVdCT2zHWQMj1EzaOOi8BtyZ3rkUaHtSNblkCAutAeJiP8aMD8r4Ymdvv98ClJDPxCEnH8lGPeuBiTaXOWpPKW9c3k+Cd6hpIkLx4O52Fs=".from_base64().unwrap();

  let mut pkey = PKey::new();
  pkey.load_pub(&keydata);
  let encrypted = pkey.encrypt(&data); // NO PADDING

  println!("{:?}", encrypted);
}
