use std::collections::BTreeMap;
use spin::Mutex;
use lazy_static::lazy_static;
pub struct Pallet
{
   pub balance:BTreeMap<String,u64>
}
// creating a static binary binding balnce pallet that will be used to operate the ledger balances
lazy_static!
{
    pub static ref balance_pallet:Mutex<Pallet>=
    {
     let mut pallet=Pallet
     {
        balance:BTreeMap::new()
     };

     return Mutex::new(pallet);
    };
}

impl Pallet
{
    pub fn init()
    {
      // this will be called at the start of the process so that the pallet is accessed for the first time
    //   and it then intialize the pallet

    }

    pub fn current_balance(&self)
    {

    }
}