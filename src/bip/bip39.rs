use crate::bip::memonics;
use pbkdf2::{pbkdf2_hmac};
use sha2::Sha512;
use crate::utilities::to_vec_string;

pub fn gen_seed_pbkdf2(memonics:&str,passphrase:&str)->[u8;64]
{
    let mut result:[u8;64]=[0;64];
    let mem:Vec<String>=to_vec_string(memonics);
    let mut bytes:Vec<u8>=Vec::new();
    for (i,str) in mem.iter().enumerate()
    {
      for byte in String::from(str).bytes()
      {
        bytes.push(byte)
      } 
    }

    let mem:Vec<u8>=bytes.clone();
    // now converting the passphrase to the bytes
    for byte in String::from(passphrase).bytes()
    {
     bytes.push(byte);
    }

    // create an array of the following 512/8=64 bytes
    let mut key:[u8;64]=[0;64];
    pbkdf2_hmac::<Sha512>(&mem,&bytes,2048,& mut result);
  return result;   
}   