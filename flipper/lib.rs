#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {

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
            self.other_flipper.flip();
        }

        #[ink(message)]
        pub fn cross_get(&self) -> bool{
            self.other_flipper.get()
        }
    }
}
