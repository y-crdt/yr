use extendr_api::prelude::*;

use yrs::sync::SyncMessage as YSyncMessage;
use yrs::updates::{decoder::Decode as YDecode, encoder::Encode as YEncode};

use crate::type_conversion::IntoExtendr;
use crate::utils;
use crate::StateVector;

utils::extendr_struct!(#[extendr] pub SyncMessage(YSyncMessage));

#[extendr]
impl SyncMessage {
    fn decode_v1(data: &[u8]) -> Result<Self, Error> {
        YSyncMessage::decode_v1(data).extendr().map(From::from)
    }

    fn decode_v2(data: &[u8]) -> Result<Self, Error> {
        YSyncMessage::decode_v2(data).extendr().map(From::from)
    }

    fn new(
        #[extendr(default = "NULL")] sync_step1: Robj,
        #[extendr(default = "NULL")] sync_step2: Robj,
        #[extendr(default = "NULL")] update: Robj,
    ) -> Result<Self, Error> {
        match (sync_step1.is_null(), sync_step2.is_null(), update.is_null()) {
            (false, true, true) => {
                let sv: &StateVector = (&sync_step1).try_into()?;
                Self::from_sync_step1(sv)
            }
            (true, false, true) => Self::from_sync_step2(Raw::try_from(sync_step2)?.as_slice()),
            (true, true, false) => Self::from_update(Raw::try_from(update)?.as_slice()),
            _ => Err(Error::Other(
                "Exactly one of 'sync_step1', 'sync_step2', or 'update' must be provided".into(),
            )),
        }
    }

    fn from_sync_step1(state_vector: &StateVector) -> Result<Self, Error> {
        Ok(Self::from(YSyncMessage::SyncStep1(
            state_vector.as_ref().clone(),
        )))
    }

    fn from_sync_step2(data: &[u8]) -> Result<Self, Error> {
        Ok(Self::from(YSyncMessage::SyncStep2(data.to_vec())))
    }

    fn from_update(data: &[u8]) -> Result<Self, Error> {
        Ok(Self::from(YSyncMessage::Update(data.to_vec())))
    }

    fn equal(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }

    fn not_equal(&self, other: &Self) -> bool {
        self.as_ref().ne(other.as_ref())
    }

    fn encode_v1(&self) -> Vec<u8> {
        self.as_ref().encode_v1()
    }

    fn encode_v2(&self) -> Vec<u8> {
        self.as_ref().encode_v2()
    }

    fn step(&self) -> &str {
        match self.as_ref() {
            YSyncMessage::SyncStep1(_) => "sync_step1",
            YSyncMessage::SyncStep2(_) => "sync_step2",
            YSyncMessage::Update(_) => "update",
        }
    }

    fn is_sync_step1(&self) -> bool {
        matches!(self.as_ref(), YSyncMessage::SyncStep1(_))
    }

    fn is_sync_step2(&self) -> bool {
        matches!(self.as_ref(), YSyncMessage::SyncStep2(_))
    }

    fn is_update(&self) -> bool {
        matches!(self.as_ref(), YSyncMessage::Update(_))
    }

    fn state_vector(&self) -> Result<StateVector, Error> {
        match self.as_ref() {
            YSyncMessage::SyncStep1(sv) => Ok(StateVector::from(sv.clone())),
            _ => Err(Error::Other(format!(
                "Expected step to be 'sync_step1', got {}",
                self.step()
            ))),
        }
    }

    fn data(&self) -> Result<Raw, Error> {
        match self.as_ref() {
            YSyncMessage::SyncStep2(data) | YSyncMessage::Update(data) => Ok(Raw::from_bytes(data)),
            _ => Err(Error::Other(format!(
                "Expected step to be 'sync_step2' or 'update`, got {}",
                self.step()
            ))),
        }
    }
}

extendr_module! {
    mod message;
    impl SyncMessage;
}
