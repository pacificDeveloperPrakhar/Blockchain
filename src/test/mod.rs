#[cfg(test)]
#[test]
fn balance_pallet_working() {
    use blockchain_rust::balances::balance_pallet;
//    .lock() return the MutexGuard<'_,pallet>
   let mut mutex_guard=balance_pallet.lock();
//    now as long as this mutex_guard is not dropped this lock will remain 
// as soon as the mutex_guard will be dropped off the memory the lock will be dropped
mutex_guard.insert(String::from("jason"),&234);
let value=mutex_guard.balance.get("jaoson");
    assert_eq!(value, Some(&234));
}
