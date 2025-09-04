struct Transaction
{   
    // id is the transaction hash that is generated from the content of the hash
    id:String,
    initiator:String,
    beneficiar:String,
    amount:u64,
    timestamp:u128,
    signature:String
}
// a block struct to define what field should a block should contain
struct Block
{
    sequence:u128,
    timestamp:u128,
    current_hash:String,
    prev_hash:String,
    // hash of all transaction's hashes within this block
    merkle_root:String,
    nonce:String,
    transactions:Vec<Transaction>,
    version:u32,
    difficulty:u64
}

impl Transaction
{
    pub fn verify()->bool
    {
        return true;
    }

    // pub fn new(initiator:String,beneficiar:String,amount:u64)->self
    // {
    //   return self {

    //   }
    // }
}