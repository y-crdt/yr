use extendr_api::prelude::*;
use yrs::updates::{decoder::Decode as YDecode, encoder::Encode as YEncode};

use crate::type_conversion::IntoExtendr;
use crate::StateVector;

#[extendr]
#[derive(Default)]
pub struct Update(yrs::Update);

impl From<yrs::Update> for Update {
    fn from(value: yrs::Update) -> Self {
        Self(value)
    }
}

#[extendr]
impl Update {
    pub fn decode_v1(data: &[u8]) -> Result<Self, Error> {
        Ok(Self(yrs::Update::decode_v1(data).extendr()?))
    }

    pub fn decode_v2(data: &[u8]) -> Result<Self, Error> {
        Ok(Self(yrs::Update::decode_v2(data).extendr()?))
    }

    pub fn new() -> Self {
        Self(yrs::Update::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn extends(&self, state_vector: &StateVector) -> bool {
        self.0.extends(state_vector)
    }

    pub fn encode_v1(&self) -> Vec<u8> {
        self.0.encode_v1()
    }

    pub fn encode_v2(&self) -> Vec<u8> {
        self.0.encode_v2()
    }

    pub fn state_vector(&self) -> StateVector {
        self.0.state_vector().into()
    }

    pub fn state_vector_lower(&self) -> StateVector {
        self.0.state_vector_lower().into()
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

extendr_module! {
    mod update;
    impl Update;
}
