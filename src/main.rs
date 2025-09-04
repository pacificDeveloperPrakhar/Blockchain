pub mod consesus;
pub mod bip;
pub mod utilities;
pub mod blockchain;
pub mod test;
#[cfg(not(test))]
fn main() {
//  let data:String=String::from(r#"{
//   "from": "Alice",
//   "to": "Bob",
//   "amount": 100,
//   "signature": "3045022100dff...abc"
// }"#);
//   let result:(String,u64)=consesus::proof_of_work::func(&data);
//   println!("{}-{}",result.0,result.1);
// ================================================================================================================
// use blockchain_rust::balances::balance_pallet;
// balance_pallet.lock().balance.insert(String::from("john"),234);
// ===================================================================================================================
use crate::bip::bip39::gen_seed_pbkdf2;
let result=gen_seed_pbkdf2("exhaust hair stable across valley claw episode document focus bundle exact mouse inquiry nasty sense believe fence lobster pull one noise palace stick pudding","");
use hex::encode;
println!("{}",encode(result));
}

