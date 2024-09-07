#![cfg_attr(not(feature = "std"), no_std, no_main)]


pub use self::other_flipper::{
    OtherFlipper,
    OtherFlipperRef,
};

#[ink::trait_definition]
pub trait OtherFlip {
    // định nghĩa abstract method flip 
    #[ink(message)]
    fn flip(&mut self);

    // định nghĩa abstract method get 
    #[ink(message)]
    fn get(&self) -> bool;
}


#[ink::contract]
mod other_flipper {
    use super::OtherFlip;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct OtherFlipper {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl OtherFlipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
    }

    impl OtherFlip for OtherFlipper {
        #[ink(message)]
        fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        fn get(&self) -> bool {
            self.value
        }

    }

}
