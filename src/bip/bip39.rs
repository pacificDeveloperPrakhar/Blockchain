use crate::bip::memonics;
use pbkdf2::{pbkdf2_hmac};
use sha2::{Sha512,Sha256};
use crate::utilities::to_vec_string;
use unicode_normalization::UnicodeNormalization;
// seed = PBKDF2(
//   password = mnemonic (UTF-8 NFKD),
//   salt     = "mnemonic" + passphrase (UTF-8 NFKD),
//   rounds   = 2048,
//   HMAC-SHA512,
//   output   = 64 bytes
// )


pub fn gen_seed_pbkdf2(mnemonics:&str,passphrase:&str)->[u8;64]
{
    let mut result:[u8;64]=[0;64];
  // there is something called nfkd normalization that we are suppposed to perform on the both mnemonics and
  // the passphrase
    // Perform NFKD normalization
    let normalized_mnemonics: String = mnemonics.nfkd().collect();
    let normalized_passphrase: String = passphrase.nfkd().collect();
    
    // let salt="mnemonic+passphrase";
    let salt = format!("mnemonic{}", normalized_passphrase);

    // create an array of the following 512/8=64 bytes
    let mut key:[u8;64]=[0;64];
    pbkdf2_hmac::<Sha512>(normalized_mnemonics.as_bytes(),salt.as_bytes(),2048,& mut result);
  return result;   
}   