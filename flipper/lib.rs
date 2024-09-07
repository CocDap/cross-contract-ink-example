#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {

    use other_flipper::OtherFlipperRef;
    use ink::codegen::TraitCallBuilder;

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

        #[ink(message)]
        pub fn cross_flip(&mut self){
            let call_builder = self.other_flipper.call_mut();

            call_builder.flip().call_v1().invoke();
        }

        #[ink(message)]
        pub fn cross_get(&self) -> bool{
            let call_builder = self.other_flipper.call();

            call_builder.get().call_v1().invoke()
        }
    }
}
