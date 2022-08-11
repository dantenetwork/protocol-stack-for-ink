#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod signatureCrseco {

    use payload::message_protocol::{ MessagePayload, MessageItem, MsgDetail};
    use payload::message_define::{ISentMessage, IReceivedMessage};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct SignatureCrseco {
        
    }

    impl SignatureCrseco {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {  }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn signatureVerify(&self, msg: ink_prelude::string::String, signature: [u8; 65], acct: AccountId)-> bool {
            let mut msg_hash = <ink_env::hash::Sha2x256 as ink_env::hash::HashOutput>::Type::default();
            ink_env::hash_encoded::<ink_env::hash::Sha2x256, _>(&msg, &mut msg_hash);

            let mut compressed_pubkey = [0; 33];
            ink_env::ecdsa_recover(&signature, &msg_hash, &mut compressed_pubkey);

            let mut addr_hash = <ink_env::hash::Blake2x256 as ink_env::hash::HashOutput>::Type::default();
            ink_env::hash_encoded::<ink_env::hash::Blake2x256, _>(&compressed_pubkey, &mut addr_hash);

            AccountId::from(addr_hash) == acct
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            // let signatureCrseco = SignatureCrseco::default();

            const signature: [u8; 65] = [
                208,164,41,181,100,96,15,94,203,97,186,168,16,154,160,93,0,147,235,73,206,32,45,162,1,133,122,170,97,187,176,173,213,157,254,83,69,40,158,12,38,251,12,110,145,138,2,24,192,91,225,141,109,12,243,52,204,238,127,61,157,5,36,243,27
            ];
            const message_hash: [u8; 32] = [
                193,62,69,99,16,27,254,90,125,207,10,134,7,138,102,18,141,117,120,3,57,189,74,12,74,79,203,138,245,221,239,138
            ];

            let msg = "Hello Nika2";
            // let mut msg_code: ink_prelude::vec::Vec<u8> = ink_prelude::vec::Vec::<u8>::new();
            // scale::Encode::encode_to(msg, &mut msg_code);

            let mut msg_hash = <ink_env::hash::Sha2x256 as ink_env::hash::HashOutput>::Type::default();
            ink_env::hash_encoded::<ink_env::hash::Sha2x256, _>(&msg, &mut msg_hash);

            assert_eq!(message_hash, msg_hash);

            const EXPECTED_COMPRESSED_PUBLIC_KEY: [u8; 33] = [
                2,70,89,9,29,24,64,247,97,172,227,31,157,179,226,227,78,252,100,14,16,156,60,47,110,87,58,16,78,230,150,20,114
            ];
            let mut output = [0; 33];
            ink_env::ecdsa_recover(&signature, &message_hash, &mut output);
            assert_eq!(output, EXPECTED_COMPRESSED_PUBLIC_KEY);
        }

        // #[ink::test]
        // fn test_pubkey() {
        //     const EXPECTED_COMPRESSED_PUBLIC_KEY: [u8; 33]= [
        //         2,  70,  89,  9,  29,  24,  64, 247,
        //         97, 172, 227, 31, 157, 179, 226, 227,
        //         78, 252, 100, 14,  16, 156,  60,  47,
        //         110,  87,  58, 16,  78, 230, 150,  20,
        //         114
        //     ];
        //     let mut output = <ink_env::hash::Blake2x256 as ink_env::hash::HashOutput>::Type::default();
        //     ink_env::hash_encoded::<ink_env::hash::Blake2x256, _>(&EXPECTED_COMPRESSED_PUBLIC_KEY, &mut output);

        //     let acct: ink_env::AccountId = AccountId::from(output);

        //     assert_eq!(acct, AccountId::from([0;32]));
        // }
    }
}
