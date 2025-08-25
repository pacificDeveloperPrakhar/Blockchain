use std::any::type_name_of_val;
use hex;
use sha2::{Sha256,Digest};

pub fn func(input: &str) -> (String, u64) {
    let mut nonce: u64 = 0;

    let mut root_value:String=input.to_string();
    loop {
    //  the input value that will be used to be hashed
    let value=format!("{}{}",root_value,nonce);   
     let mut input=Sha256::new();
     input.update(value.as_bytes());
    //  now we will take the input and get the hashed verison of this
     let  input:Vec<u8>=input.finalize().to_vec();
    // now take the bytes of the hashed and get the hex form of it
     let input=hex::encode(input);
     if input.starts_with("0000")
     {
        return (input,nonce);
     }
     nonce =match nonce.checked_add(1)
     {
        Some(x)=>x,
        None=>{
         root_value=String::from(format!("{}{}",root_value,0));
         0
        }
     }
    }
}