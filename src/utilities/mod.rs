pub fn to_binary(bytes:& Vec<u8>)->String
{   let mut value:String=String::from("");
    for i in 0..bytes.len()
    {
     value=format!("{}{}",value,format!("{:08b}",bytes[i]));
    }
    return value
}

// now converting the binary repsentation back to the bytes
pub fn binary_to_bytes(s:&String)->Vec<u8>
{
    let mut bytes:Vec<u8>=Vec::new();
    for i in 0..(s.len()/8)
    {   let i=i*8;
        bytes.push(u8::from_str_radix(&s[i..(i+8)],2).unwrap());
    }
    return bytes;
}

// now we will take te function to convert the bits into the 11 bit decimal equivalent number\
pub fn binary_to_11bitDecimalNumber(s:&String)->Vec<u16>
{
    println!("{}",s.len());
  let mut numbers:Vec<u16>=Vec::new();
  for i in 0..(s.len()/11)
  {
   let i=i*11;
   numbers.push(u16::from_str_radix(&s[i..(i+11)],2).unwrap());
  }
  return numbers;
}

// ============================================================================================================
// now the utilities function to convert the string of memonics to the vector of string
pub fn to_vec_string(memonics:&str)->Vec<String>
{
    let mut result:Vec<String>=Vec::new();
    for str in String::from(memonics).split_whitespace()
    {
        result.push(String::from(str));
    }
    return result;
}