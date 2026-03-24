pub mod array;
pub mod doc;
pub mod map;
pub mod state;
pub mod text;
pub mod transaction;
pub mod type_conversion;
pub mod update;

pub use crate::type_conversion::{FromExtendr, IntoExtendr};
pub use crate::{
    array::ArrayRef, doc::Doc, map::MapRef, state::StateVector, text::TextRef,
    transaction::Transaction,
};
pub(crate) use transaction::try_read;

// Register function with R.
// See corresponding C code in `entrypoint.c`.
extendr_api::extendr_module! {
    mod yar;
    use transaction;
    use array;
    use map;
    use state;
    use text;
    use update;
    use doc;
}
