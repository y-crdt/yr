use extendr_api::prelude::*;
use yrs::{Array as YArray, ArrayPrelim, MapPrelim as YMapPrelim, TextPrelim as YTextPrelim};

use crate::type_conversion::{FromExtendr, IntoExtendr};
use crate::{try_read, MapRef, TextRef, Transaction};

#[extendr]
pub struct ArrayRef(yrs::ArrayRef);

impl From<yrs::ArrayRef> for ArrayRef {
    fn from(value: yrs::ArrayRef) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for ArrayRef {
    type Target = yrs::ArrayRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[extendr]
impl ArrayRef {
    pub fn len(&self, transaction: &Transaction) -> Result<u32, Error> {
        try_read!(transaction, t => self.0.len(t))
    }

    pub fn insert_any(
        &self,
        transaction: &mut Transaction,
        index: u32,
        obj: Robj,
    ) -> Result<(), Error> {
        let trans = transaction.try_mut()?;
        let any = yrs::Any::from_extendr(obj)?;
        self.0.insert(trans, index, any);
        Ok(())
    }

    pub fn insert_text(&self, transaction: &mut Transaction, index: u32) -> Result<TextRef, Error> {
        transaction
            .try_mut()
            .map(|trans| TextRef::from(self.0.insert(trans, index, YTextPrelim::default())))
    }

    pub fn insert_array(
        &self,
        transaction: &mut Transaction,
        index: u32,
    ) -> Result<ArrayRef, Error> {
        transaction
            .try_mut()
            .map(|trans| ArrayRef::from(self.0.insert(trans, index, ArrayPrelim::default())))
    }

    pub fn insert_map(&self, transaction: &mut Transaction, index: u32) -> Result<MapRef, Error> {
        transaction
            .try_mut()
            .map(|trans| MapRef::from(self.0.insert(trans, index, YMapPrelim::default())))
    }

    pub fn get(&self, transaction: &mut Transaction, index: u32) -> Result<Robj, Error> {
        try_read!(transaction, t => self.0.get(t, index).extendr()).and_then(|r| r)
    }

    pub fn remove(&self, transaction: &mut Transaction, index: u32) -> Result<(), Error> {
        transaction.try_mut().map(|trans| {
            self.0.remove(trans, index);
        })
    }
}

extendr_module! {
    mod array;
    impl ArrayRef;
}
