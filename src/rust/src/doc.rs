use extendr_api::prelude::*;

use crate::{ArrayRef, MapRef, TextRef};

#[extendr]
pub struct Doc(yrs::Doc);

impl From<yrs::Doc> for Doc {
    fn from(value: yrs::Doc) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for Doc {
    type Target = yrs::Doc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[extendr]
impl Doc {
    fn new() -> Self {
        Self(yrs::Doc::new())
    }

    fn client_id(&self) -> u64 {
        self.0.client_id()
    }

    fn guid(&self) -> Strings {
        (*self.0.guid()).into()
    }

    fn get_or_insert_text(&self, name: &str) -> TextRef {
        self.0.get_or_insert_text(name).into()
    }

    fn get_or_insert_map(&self, name: &str) -> MapRef {
        self.0.get_or_insert_map(name).into()
    }

    fn get_or_insert_array(&self, name: &str) -> ArrayRef {
        self.0.get_or_insert_array(name).into()
    }
}

extendr_module! {
    mod doc;
    impl Doc;
}
