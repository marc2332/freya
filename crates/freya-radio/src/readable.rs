use freya_core::lifecycle::{
    readable::{
        IntoReadable,
        Readable,
    },
    state::ReadableRef,
};

use crate::{
    hooks::RadioChannel,
    slice::{
        RadioSlice,
        RadioSliceMut,
    },
};

pub trait RadioReadable<T: 'static> {
    fn from_slice<Value, Channel>(slice: RadioSlice<Value, T, Channel>) -> Self
    where
        Value: 'static,
        Channel: RadioChannel<Value> + 'static;
}

impl<T: 'static> RadioReadable<T> for Readable<T> {
    fn from_slice<Value, Channel>(slice: RadioSlice<Value, T, Channel>) -> Self
    where
        Value: 'static,
        Channel: RadioChannel<Value> + 'static,
    {
        Self::new(
            Box::new({
                let slice = slice.clone();
                move || ReadableRef::Ref(slice.read_unchecked())
            }),
            Box::new(move || ReadableRef::Ref(slice.read_unchecked())),
        )
    }
}

impl<T: 'static, Value: 'static, Channel: RadioChannel<Value> + 'static> IntoReadable<T>
    for RadioSlice<Value, T, Channel>
{
    fn into_readable(self) -> Readable<T> {
        Readable::from_slice(self)
    }
}

impl<T: 'static, Value: 'static, Channel: RadioChannel<Value> + 'static> IntoReadable<T>
    for RadioSliceMut<Value, T, Channel>
{
    fn into_readable(self) -> Readable<T> {
        Readable::new(
            Box::new({
                let this = self.clone();
                move || ReadableRef::Ref(this.read_unchecked())
            }),
            Box::new(move || ReadableRef::Ref(self.peek_unchecked())),
        )
    }
}
