#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {

    use other_flipper::OtherFlipperRef;
    use ink::codegen::TraitCallBuilder;
    // import trait 
    use other_flipper::OtherFlip;


    #[ink(storage)]
    pub struct Flipper {
        other_flipper: ink::contract_ref!(OtherFlip),
    }

    impl Flipper {

        #[ink(constructor)]
        pub fn new_v1(other_flipper_contract: AccountId) -> Self {


            Self { other_flipper: other_flipper_contract.into() }
        }

        #[ink(message)]
        pub fn cross_flip(&mut self){
            todo!()
        }

        #[ink(message)]
        pub fn cross_get(&self) -> bool{
            todo!()
        }
    }
}
