use extendr_api::prelude::*;
use yrs::updates::{decoder::Decode as YDecode, encoder::Encode as YEncode};

use crate::type_conversion::IntoExtendr;

#[extendr]
pub struct StateVector(yrs::StateVector);

impl From<yrs::StateVector> for StateVector {
    fn from(value: yrs::StateVector) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for StateVector {
    type Target = yrs::StateVector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
