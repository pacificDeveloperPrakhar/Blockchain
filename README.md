
# Blockchain

##### A blockchain implementation using rust ,with bip32 using edsa25519 rather than the sekp256k1 just like the solana

way of creating the memonics the 

![alt text](./public/memonics1.png)

### a slight overview of how do the Mutex Guard works

* first we have created the lazy_static wrapper around the pallet this ensures that the mutex static is stored inside of the static binding of the program

* now it also ensures that the pallet struct is initialized during the time of runtime ,the lazy_static just reserves the memory in the static for the to be initialized in the runtime

* when the lock is called the mutex object is created and as long as the mutex guard object remain in the scope
the lock will be remained as soon as the mute guard is dropped off from the memory the lock is dropped


![alt textt](./public/mutex_guard_excalidraw.png)

#### WARNING:We are using the no td feaure of the lazy static meaning the spin lock version of mutex we are using is the cpu extensive and is often used for the bare metal ,Use std version over the no std version



# Pallet

pallet are these modular functionlaity that are 
like plug in for the blockchain runtime.

* they are sort of use in substrate that is a framework used to create custom blockchain for

* It’s not a blockchain itself → Substrate is a blockchain development framework.

* Polkadot is built using Substrate. Many other chains (Kusama, Acala, Moonbeam, etc.) are also Substrate-based.


## MuexGuard working

* std::sync::Mutex vs spin::Mutex

### std::sync::Mutex 

* Part of the standard library (std).

* Uses OS-level synchronization primitives (like futexes on Linux, critical sections on Windows).

* Requires an operating system → not usable in #![no_std] environments.

* spin::Mutex (from the spin crate
)

* Implemented with spinlocks: the thread busy-waits in a loop until the lock is free.

* Does not require the OS or syscalls → works in #![no_std] environments like kernels, bare-metal, or embedded.

* But: CPU keeps spinning while waiting, so it’s not efficient in multitasking environments (OS with many threads).