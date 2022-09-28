
/// ABI related struct can be defined here
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
#[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub struct CalleeDefinedData{
    pub n: u128,
    pub s: ink::prelude::string::String,
}

pub fn get() -> u16{
    8
}