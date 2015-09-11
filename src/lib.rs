extern crate pem_parser;
extern crate openssl as openssl_;
extern crate rand as rand_;
pub use self::openssl_ as openssl;
pub use self::rand_ as rand;

use self::openssl::crypto::pkey::{PKey, EncryptionPadding};

mod symm;
pub use symm::*;

// Using a thread-local because PKey doesn't implement Sync.
thread_local! {
    static STEAM_PKEY: PKey = {
        let mut pkey = PKey::new();
        pkey.load_pub(include_str!("../steam.pub").as_bytes());
        pkey
    }
}

pub struct SessionKeys {
  pub plain: Vec<u8>,
  pub encrypted: Vec<u8>
}

/// Generate a session key and a copy encrypted with Steam's public key.
pub fn generate_session_key() -> SessionKeys {
  let session_key: [u8; 32] = rand::random(); // XXX: make sure this RNG is cryptographically secure

  let encrypted_session_key: Vec<u8> = STEAM_PKEY.with(|pk| {
    pk.encrypt_with_padding(
      &session_key,
      EncryptionPadding::PKCS1v15
    )
  });

  SessionKeys {
    plain: session_key.to_vec(),
    encrypted: encrypted_session_key
  }
}
