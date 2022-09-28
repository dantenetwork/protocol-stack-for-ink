#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub trait RawData {
    fn into_raw_data(&self) -> ink_prelude::vec::Vec<u8>;
}

impl RawData for ink_prelude::vec::Vec<u8> {
    fn into_raw_data(&self) -> ink_prelude::vec::Vec<u8> {
        self.clone()
    }
}

#[ink::contract]
mod signatureCrseco {

    use payload::message_protocol::{ MessagePayload, MessageItem, MsgDetail, InMsgType};
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
            ink_env::hash_bytes::<ink_env::hash::Sha2x256>(&msg.as_bytes(), &mut msg_hash);

            let mut compressed_pubkey = [0; 33];
            ink_env::ecdsa_recover(&signature, &msg_hash, &mut compressed_pubkey);

            let mut addr_hash = <ink_env::hash::Blake2x256 as ink_env::hash::HashOutput>::Type::default();
            ink_env::hash_encoded::<ink_env::hash::Blake2x256, _>(&compressed_pubkey, &mut addr_hash);

            AccountId::from(addr_hash) == acct
        }

        #[ink(message)]
        pub fn get_raw_data(&self) -> ink_prelude::vec::Vec<u8> {
            let mut raw_buffer = ink_prelude::vec![];

            let mut int32_vec = ink_prelude::vec![99 as i32, 88, 77];
            for ele in int32_vec.iter_mut() {
                raw_buffer.append(&mut ink_prelude::vec::Vec::from(ele.to_be_bytes()));
            }

            let some_str = ink_prelude::string::String::from("Hello Nika");
            raw_buffer.append(&mut ink_prelude::vec::Vec::from(some_str.as_bytes()));

            raw_buffer
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
        fn test_sha2256_secp256k1() {
            // let signatureCrseco = SignatureCrseco::default();

            const signature: [u8; 65] = [
                119, 239,  67, 254,  77,  20, 200, 139, 106,  52, 180,
  113,   5,  87,  53, 109, 195, 208,  44, 145,  57, 206,
   32,  49, 154,  97, 194,  75, 128, 180, 187,  77, 103,
  117, 252, 208,  68, 198, 154,  45, 159, 113,   5,  83,
  206,  99,  41, 210, 144, 235,  48, 199,  57, 192,  38,
  105, 190,  24, 173, 145, 200, 110, 136,  86,  27
            ];
            const message_hash: [u8; 32] = [
                238, 229, 119, 112, 248,  69, 107, 141,
   74,  45, 169, 173,   2, 132,  54, 236,
  106,  98,  71, 118,  53, 193,  37, 113,
  246,  83, 204,  25,  86,  45,  95, 211
            ];

            let msg = "hello nika";
            // let mut msg_code: ink_prelude::vec::Vec<u8> = ink_prelude::vec::Vec::<u8>::new();
            // scale::Encode::encode_to(msg, &mut msg_code);

            let mut msg_hash = <ink_env::hash::Sha2x256 as ink_env::hash::HashOutput>::Type::default();
            ink_env::hash_bytes::<ink_env::hash::Sha2x256>(&msg.as_bytes(), &mut msg_hash);

            assert_eq!(message_hash, msg_hash);

            const EXPECTED_COMPRESSED_PUBLIC_KEY: [u8; 33] = [
                2,144,101,32,18,128,96,228,162,202,76,18,107,219,5,157,35,133,125,153,254,81,97,69,51,241,57,23,173,207,216,227,161
            ];
            let mut output = [0; 33];
            ink_env::ecdsa_recover(&signature, &message_hash, &mut output);
            assert_eq!(output, EXPECTED_COMPRESSED_PUBLIC_KEY);
        }

        #[ink::test]
        fn test_keccak256_secp256k1() {
            // let signatureCrseco = SignatureCrseco::default();

            const signature: [u8; 65] = [
                227,  45, 217, 140, 164, 120,  53, 166, 163, 222,   2,
  249, 128, 197,  65,  49, 198,  43, 172, 194,  44, 240,
  100, 128,  86, 188, 246,  45, 199, 179, 185, 206, 111,
  124, 164,   5, 246,  79, 165,  46, 129, 236, 241,  16,
  145,  96, 252, 187,  77, 110,  14, 120, 183,  34, 245,
  190, 141, 185, 171,  13,  95, 138, 209,  70,  27
            ];
            const message_hash: [u8; 32] = [
                243,208,217,198,193,171,36,240,216,203,71,75,177,226,136,29,157,199,168,47,109,57,194,60,34,70,73,249,39,51,45,112
            ];

            let msg = "hello nika";
            // let mut msg_code: ink_prelude::vec::Vec<u8> = ink_prelude::vec::Vec::<u8>::new();
            // scale::Encode::encode_to(msg, &mut msg_code);

            let mut msg_hash = <ink_env::hash::Keccak256 as ink_env::hash::HashOutput>::Type::default();
            ink_env::hash_bytes::<ink_env::hash::Keccak256>(&msg.as_bytes(), &mut msg_hash);

            assert_eq!(message_hash, msg_hash);

            const EXPECTED_COMPRESSED_PUBLIC_KEY: [u8; 33] = [
                2,144,101,32,18,128,96,228,162,202,76,18,107,219,5,157,35,133,125,153,254,81,97,69,51,241,57,23,173,207,216,227,161
            ];
            let mut output = [0; 33];
            ink_env::ecdsa_recover(&signature, &msg_hash, &mut output);
            assert_eq!(output, EXPECTED_COMPRESSED_PUBLIC_KEY);
        }

        #[ink::test]
        fn test_raw_data() {
            let mut raw_buffer = ink_prelude::vec![];

            let mut int32_vec = ink_prelude::vec![99 as i32, 88, 77];
            for ele in int32_vec.iter_mut() {
                raw_buffer.append(&mut ink_prelude::vec::Vec::from(ele.to_be_bytes()));
                *ele += 1;
            }

            let some_str = ink_prelude::string::String::from("Hello Nika");
            raw_buffer.append(&mut ink_prelude::vec::Vec::from(some_str.as_bytes()));

            assert_eq!(int32_vec[0], 100);
            assert_eq!(raw_buffer.len(), 12 + some_str.len());
        }

        #[ink::test]
        fn test_raw_string() {
            let mut raw_utf8 = [0u8;256];
            let mut i: u8 = 0;
            for mut ele in raw_utf8.iter_mut() {
                *ele = i;
                if (i < 255u8) {
                    i += 1;
                }
            }

            let mut raw_str = ink_prelude::string::String::new();

            unsafe {
                raw_str = ink_prelude::string::String::from_utf8_unchecked(raw_utf8.to_vec());
            }
            
            assert_eq!(raw_str.as_bytes(), raw_utf8);
        }

        #[ink::test]
        fn test_i_number() {
            let i_num: i8 = -99;

            let mut raw_data = ink_prelude::vec![];

            raw_data.push(i_num as u8);

            assert_eq!(raw_data[0] as i8, -99);
        }

        #[ink::test]
        fn test_crypto_payload() {
            let address_here = payload::message_protocol::InkAddressData {
                ink_address: ink_prelude::vec![1, 2, 3],
                address_type: 0
            };

            let raw1 = address_here.clone().into_raw_data();
            let raw2 = address_here.into_raw_data();

            assert_eq!(raw1, raw2);
        }

    }
}
