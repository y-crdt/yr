use extendr_api::prelude::*;
use yrs::types::text::TextEvent as YTextEvent;
use yrs::{GetString as YGetString, Text as YText};

use crate::event::ExtendrObservable;
use crate::type_conversion::IntoExtendr;
use crate::utils::{self, lifetime, ExtendrRef};
use crate::{try_read, ExtendrTransaction, Transaction};

utils::extendr_struct!(#[extendr] pub TextRef(yrs::TextRef));

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
        let text = self.0.clone(); // Cheap ptr copy
        let chunk = chunk.to_string();
        transaction.with_write_mut(move |trans| text.insert(trans, index, &chunk))
    }

    pub fn push(&self, transaction: &mut Transaction, chunk: &str) -> Result<(), Error> {
        let text = self.0.clone(); // Cheap ptr copy
        let chunk = chunk.to_string();
        transaction.with_write_mut(move |trans| text.push(trans, &chunk))
    }

    pub fn remove_range(
        &self,
        transaction: &mut Transaction,
        index: u32,
        len: u32,
    ) -> Result<(), Error> {
        let text = self.0.clone(); // Cheap ptr copy
        transaction.with_write_mut(move |trans| text.remove_range(trans, index, len))
    }

    pub fn get_string(&self, transaction: &Transaction) -> Result<String, Error> {
        try_read!(transaction, t => self.0.get_string(t))
    }

    pub fn observe(&self, f: Function, key: &Robj) -> Result<(), Error> {
        ExtendrObservable::<TextEvent>::observe(self, f, key)
    }

    pub fn unobserve(&self, key: &Robj) -> Result<(), Error> {
        ExtendrObservable::<TextEvent>::unobserve(self, key)
    }
}

utils::extendr_struct!(#[extendr] pub TextEvent(lifetime::CheckedRef<YTextEvent>));

#[extendr]
impl TextEvent {
    fn target(&self) -> Result<TextRef, Error> {
        // Cloning is shallow BranchPtr copy pinting to same data.
        self.try_map(|event| event.target().clone().into())
    }

    fn delta(&self, transaction: &Transaction) -> Result<Robj, Error> {
        self.try_map(|event| {
            transaction
                .try_write()
                .map(|trans| event.delta(trans).extendr())
        })
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
    }

    fn path(&self) -> Result<Robj, Error> {
        self.try_map(|event| event.path().extendr()).and_then(|r| r) // TODO(MSRV 1.89) .flatten()
    }
}

extendr_module! {
    mod text;
    impl TextRef;
    impl TextEvent;
}
