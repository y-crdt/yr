use extendr_api::prelude::*;
use yrs::updates::decoder::Decode as YDecode;
use yrs::{ReadTxn as YReadTxn, Transact as YTransact};

use crate::type_conversion::IntoExtendr;
use crate::{Doc, StateVector};

macro_rules! try_read {
    ($txn:expr, $t:ident => $body:expr) => {
        $txn.try_dyn().map(|txn| match txn {
            crate::transaction::DynTransaction::Write($t) => $body,
            crate::transaction::DynTransaction::Read($t) => $body,
        })
    };
}

pub(crate) use try_read;

// Perhaps we could have two different bindings of Transaction and TransactionMut
// with the same API and use a macro to bind YTransact trait methods.
#[allow(clippy::large_enum_variant)]
pub enum DynTransaction<'doc> {
    Read(yrs::Transaction<'doc>),
    Write(yrs::TransactionMut<'doc>),
}

#[extendr]
pub struct Transaction {
    // Transaction auto commits on Drop, and keeps a lock
    // We need to be able to explicitly drop the lock.
    transaction: Option<DynTransaction<'static>>,
    // Keep Document alive while the transaction is alive
    #[allow(dead_code)]
    owner: Robj,
}

impl Transaction {
    pub(crate) fn try_dyn(&self) -> Result<&DynTransaction<'static>, Error> {
        match &self.transaction {
            Some(trans) => Ok(trans),
            None => Err(Error::Other("Transaction was dropped".into())),
        }
    }

    pub(crate) fn try_mut(&mut self) -> Result<&mut yrs::TransactionMut<'static>, Error> {
        use DynTransaction::{Read, Write};
        match &mut self.transaction {
            Some(Write(trans)) => Ok(trans),
            Some(Read(_)) => Err(Error::Other("Transaction is readonly".into())),
            None => Err(Error::Other("Transaction was dropped".into())),
        }
    }
}

#[extendr]
impl Transaction {
    pub fn new(doc: ExternalPtr<Doc>, #[extendr(default = "FALSE")] mutable: bool) -> Self {
        let transaction = if mutable {
            DynTransaction::Write(doc.transact_mut())
        } else {
            DynTransaction::Read(doc.transact())
        };

        // Safety: Doc live in R memory and is kept alive in the owner field of this struct
        let transaction = unsafe {
            std::mem::transmute::<DynTransaction<'_>, DynTransaction<'static>>(transaction)
        };
        Transaction {
            owner: doc.into(),
            transaction: Some(transaction),
        }
    }

    pub fn commit(&mut self) -> Result<(), Error> {
        self.try_mut().map(|trans| trans.commit())
    }

    // Ambiguous with Drop trait, but we keep the name until we have a better approach
    // on the R side (with_transaction blocked by not being able to get Robj from inside Doc)
    #[allow(clippy::should_implement_trait)]
    pub fn drop(&mut self) {
        self.transaction = None;
    }

    pub fn state_vector(&self) -> Result<StateVector, Error> {
        try_read!(self, t => t.state_vector().into())
    }

    pub fn encode_diff_v1(&self, state_vector: &StateVector) -> Result<Vec<u8>, Error> {
        try_read!(self, t => t.encode_diff_v1(state_vector))
    }

    pub fn encode_diff_v2(&self, state_vector: &StateVector) -> Result<Vec<u8>, Error> {
        try_read!(self, t => t.encode_diff_v2(state_vector))
    }

    pub fn apply_update_v1(&mut self, data: &[u8]) -> Result<(), Error> {
        let trans = self.try_mut()?;
        let update = yrs::Update::decode_v1(data).extendr()?;
        trans.apply_update(update).extendr()
    }

    pub fn apply_update_v2(&mut self, data: &[u8]) -> Result<(), Error> {
        let trans = self.try_mut()?;
        let update = yrs::Update::decode_v2(data).extendr()?;
        trans.apply_update(update).extendr()
    }
}

extendr_module! {
    mod transaction;
    impl Transaction;
}
