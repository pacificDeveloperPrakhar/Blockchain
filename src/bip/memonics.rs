use sha2::{Sha256, Digest};
use rand::Rng;
use std::fs::File;
use std::io::Read;
use hex;

pub fn gen_memonics()->Vec<String> {
    // create the RNG object
    let mut rng = rand::thread_rng();

    // entropy for 24-word mnemonic (256 bits = 32 bytes)
    let mut entropy: [u8; 32] = [0; 32];

    // fill with random values
    for i in 0..entropy.len() {
        entropy[i] = rng.random::<u8>();
    }
    let mut hashed=Sha256::new();
    hashed.update(&entropy);
    // conver the hashed to vector of bytes
    let hashed=hashed.finalize().to_vec();
    // now after getting the hashed entropy we will take entropy in bytes /32
    let entropy_bin=crate::utilities::to_binary(&entropy.to_vec());
    let hashed_entropy_bin=crate::utilities::to_binary(&hashed);
    let first_bits=&hashed_entropy_bin[0..((entropy.len()*8)/32)];
    let result=entropy_bin+first_bits;
    // now lets convert back this result back to the 11 bit decimal number
    let memonics_number_mapping:Vec<u16>=crate::utilities::binary_to_11bitDecimalNumber(&result);
    //content will be stored in this string buffer
    let mut content:String=String::new();
    // now read the english words memonics text file 
    let mut file=match File::open("/home/prakhar/Desktop/workspace_2/blockchain/src/bip/memonics_english.txt")
    {
        Ok(file)=>file,
        Err(_)=>panic!("unable to open the file memonics_english.txt"),
    };
    
    match file.read_to_string(& mut content)
    {
        Ok(_)=>{},
        Err(e)=>panic!("unable to read the files content")
    };

    let substring:Vec<&str>=content.split("\n").collect();
    // i am converting the string or the mnemonic word to the string
    let mut memonics:Vec<String>=Vec::new();
    for i in 0..memonics_number_mapping.len()
    {
        memonics.push(substring[i].to_string());
    }
    // now return the memonics
    return memonics;
}
