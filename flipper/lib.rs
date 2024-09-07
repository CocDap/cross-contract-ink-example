#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {

    use other_flipper::OtherFlipperRef;

    #[ink(storage)]
    pub struct Flipper {
        other_flipper: OtherFlipperRef
    }

    impl Flipper {
        /// Initializes the contract by instantiating the code at the given code hash via
        /// the original `instantiate` host function.
        #[ink(constructor)]
        pub fn new_v1(other_flipper_code_hash: Hash) -> Self {
            let other_flipper = OtherFlipperRef::new(true)
                .instantiate_v1()
                .code_hash(other_flipper_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self { other_flipper }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            todo!()
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            todo!()
        }
    }
}
