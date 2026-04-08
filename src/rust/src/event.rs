/// Wrap a yrs callback to be safely used in R.
///
/// This needs to be a macro because some types call it multiple times with
/// different method names.
///
/// # Reference management
///
/// The Transaction and Event reference lifetime are erased and a pointer is
/// wrapped into an [`Robj`][`extendr_api::Robj`]. THis is required because
/// lifetimes cannot exist in an [`Robj`][`extendr_api::Robj`].
/// To avoid dangling pointers, they have been wrapped in an [`Option`] (this
/// is part of their type) and explicitly cleared after the callback ends on
/// the Rust side. Any further use outside of the callback on the R side will
/// raise an error.
///
/// See [`crate::utils::ExtendrRef`], for a safe reference wrapper used here.
macro_rules! observe_with {
    ($self:expr, $method:ident, $Event:ty, $f:expr, $key:expr) => {{
        if $f.formals().map(|g| g.len()).unwrap_or(0) != 2 {
            return Err(extendr_api::Error::Other(
                "Callback expect exactly two parameters: transaction and event".into(),
            ));
        }

        $self.$method(
            $crate::Origin::new($key)?,
            move |trans: &yrs::TransactionMut<'_>, event: &_| {
                let event = <$Event>::guard(event);
                // Converting to Robj first as the converter will set the class symbol attribute,
                // otherwise it will only be seen as an `externalptr` from R.
                let mut trans: Robj = $crate::Transaction::from_ref(trans).into();
                let result = $f.call(pairlist!(trans.clone(), event.get().clone().into_robj()));
                // PANIC: we just created the object with a Transaction
                TryInto::<&mut $crate::Transaction>::try_into(&mut trans)
                    .unwrap()
                    .unlock();
                // TODO Either take an on_error, or store it somewhere
                result.unwrap();
            },
        )
    }};
}

pub(crate) use observe_with;

macro_rules! unobserve_with {
    ($self:expr, $method:ident, $key:expr) => {{
        $self.$method($crate::Origin::new($key)?)
    }};
}

pub(crate) use unobserve_with;
