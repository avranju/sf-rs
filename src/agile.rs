use std::marker::PhantomData;

use windows::core::{AgileReference, ComInterface, IUnknown};

use crate::error::Error;

/// A wrapper around an `IUnknown` that is agile.
///
/// This is needed because `AgileReference` only seems to work on interfaces that
/// have been registered on the system. SF interfaces are not registered, so we
/// need to wrap them in an `IUnknown` and then wrap that in an `AgileReference`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgileRef<T>(AgileReference<IUnknown>, PhantomData<T>);

unsafe impl<T: ComInterface> Send for AgileRef<T> {}
unsafe impl<T: ComInterface> Sync for AgileRef<T> {}

impl<T: ComInterface> AgileRef<T> {
    pub fn new(unk: IUnknown) -> Result<Self, Error> {
        Ok(Self(AgileReference::new(&unk)?, Default::default()))
    }

    pub fn resolve(&self) -> Result<T, Error> {
        Ok(self.0.resolve()?.cast()?)
    }
}
