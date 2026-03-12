use extendr_api::prelude::*;
use yrs::{GetString as YGetString, Text as YText, Transact as YTransact};

#[extendr]
struct Transaction {
    // Transaction auto commits on Drop, and keeps a lock
    // We need to be able to explicitly drop the lock.
    transaction: Option<yrs::TransactionMut<'static>>,
    // Keep Document alive while the transaction is alive
    #[allow(dead_code)]
    owner: Robj,
}

#[extendr]
impl Transaction {
    fn new(doc: ExternalPtr<Doc>) -> Self {
        // Safety: Doc live in R memory and is kept alive in the owner field of this struct
        let transaction: yrs::TransactionMut<'static> =
            unsafe { std::mem::transmute(doc.doc.transact_mut()) };
        Transaction {
            owner: doc.into(),
            transaction: Some(transaction),
        }
    }

    fn commit(&mut self) -> Result<()> {
        match &mut self.transaction {
            Some(trans) => {
                let _: () = trans.commit();
                Ok(())
            }
            None => Err(Error::Other("Transaction was dropped".into())),
        }
    }

    fn drop(&mut self) {
        self.transaction = None;
    }
}

#[extendr]
struct TextRef(yrs::TextRef);

#[extendr]
impl TextRef {
    fn insert(&self, transaction: &mut Transaction, index: u32, chunk: &str) {
        if let Some(trans) = &mut transaction.transaction {
            self.0.insert(trans, index, chunk)
        }
    }

    fn get_string(&self, transaction: &Transaction) -> Result<String> {
        match &transaction.transaction {
            Some(trans) => Ok(self.0.get_string(trans)),
            None => Err(Error::Other("Transaction was dropped".into())),
        }
    }
}

#[extendr]
struct Doc {
    doc: yrs::Doc,
}

#[extendr]
impl Doc {
    fn new() -> Self {
        Self {
            doc: yrs::Doc::new(),
        }
    }

    fn client_id(&self) -> u64 {
        self.doc.client_id()
    }

    fn guid(&self) -> Strings {
        (*self.doc.guid()).into()
    }

    fn get_or_insert_text(&self, name: &str) -> TextRef {
        TextRef(self.doc.get_or_insert_text(name))
    }
}

// Register function with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod yar;
    impl Transaction;
    impl TextRef;
    impl Doc;
}
