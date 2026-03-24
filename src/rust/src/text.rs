use extendr_api::prelude::*;
use yrs::{GetString as YGetString, Text as YText};

use crate::{try_read, Transaction};

#[extendr]
pub struct TextRef(yrs::TextRef);

impl From<yrs::TextRef> for TextRef {
    fn from(value: yrs::TextRef) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for TextRef {
    type Target = yrs::TextRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[extendr]
impl TextRef {
    pub fn len(&self, transaction: &Transaction) -> Result<u32, Error> {
        try_read!(transaction, t => self.0.len(t))
    }

    pub fn insert(
        &self,
        transaction: &mut Transaction,
        index: u32,
        chunk: &str,
    ) -> Result<(), Error> {
        transaction
            .try_mut()
            .map(|trans| self.0.insert(trans, index, chunk))
    }

    pub fn push(&self, transaction: &mut Transaction, chunk: &str) -> Result<(), Error> {
        transaction.try_mut().map(|trans| self.0.push(trans, chunk))
    }

    pub fn remove_range(
        &self,
        transaction: &mut Transaction,
        index: u32,
        len: u32,
    ) -> Result<(), Error> {
        transaction
            .try_mut()
            .map(|trans| self.0.remove_range(trans, index, len))
    }

    pub fn get_string(&self, transaction: &Transaction) -> Result<String, Error> {
        try_read!(transaction, t => self.0.get_string(t))
    }
}

extendr_module! {
    mod text;
    impl TextRef;
}
