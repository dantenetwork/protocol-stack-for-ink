#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod vv {
    // use ink::env::hash::Sha2x256;


    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Vv {
        /// Stores a single `bool` value on the storage.
        value: bool,
        random_v: u32,
        // milliseconds 
        timestamp: Timestamp,
    }

    impl Vv {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { 
                value: init_value,
                random_v: 73,
                timestamp: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_rand(&self) -> [u8; 32] {
            let t_stamp = ink::env::block_timestamp::<ink::env::DefaultEnvironment>();
            let mut output = <ink::env::hash::Keccak256 as ink::env::hash::HashOutput>::Type::default();

            ink::env::hash_encoded::<ink::env::hash::Keccak256, _>(&t_stamp, &mut output);
            output
        }

        #[ink(message)]
        pub fn rand_set(&mut self) {
            let rand_bytes = self.get_rand();
            let four_bytes: [u8; 4] = rand_bytes[0..4].try_into().unwrap();
            
            self.random_v = u32::from_be_bytes(four_bytes);
        }

        #[ink(message)]
        pub fn get_rand_v(&self) -> u32 {
            self.random_v
        }

        #[ink(message)]
        pub fn time_update(&mut self) {
            self.timestamp = ink::env::block_timestamp::<ink::env::DefaultEnvironment>();
        }

        #[ink(message)]
        pub fn get_time(&self) -> Timestamp {
            self.timestamp
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let vv = Vv::default();
            assert_eq!(vv.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut vv = Vv::new(false);
            assert_eq!(vv.get(), false);
            vv.flip();
            assert_eq!(vv.get(), true);
        }
    }
}
