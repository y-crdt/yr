use extendr_api::prelude::*;
use yrs::updates::{decoder::Decode as YDecode, encoder::Encode as YEncode};

use crate::type_conversion::IntoExtendr;
use crate::utils;

utils::extendr_struct!(#[extendr] pub StateVector(yrs::StateVector));

#[extendr]
impl StateVector {
    fn decode_v1(data: &[u8]) -> Result<Self, Error> {
        Ok(Self(yrs::StateVector::decode_v1(data).extendr()?))
    }

    fn decode_v2(data: &[u8]) -> Result<Self, Error> {
        Ok(Self(yrs::StateVector::decode_v2(data).extendr()?))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn contains_client(&self, client_id: yrs::block::ClientID) -> bool {
        self.0.contains_client(&client_id)
    }

    fn equal(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn not_equal(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }

    fn less_than(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }

    fn less_than_equal(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }

    fn greater_than(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }

    fn greater_than_equal(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }

    fn encode_v1(&self) -> Vec<u8> {
        self.0.encode_v1()
    }

    fn encode_v2(&self) -> Vec<u8> {
        self.0.encode_v2()
    }
}

extendr_module! {
    mod state;
    impl StateVector;
}
