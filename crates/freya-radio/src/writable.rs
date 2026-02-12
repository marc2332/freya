use freya_core::lifecycle::writable::{
    IntoWritable,
    Writable,
};

use crate::{
    hooks::RadioChannel,
    slice::RadioSliceMut,
};

pub trait RadioWritable<T: 'static> {
    fn from_slice<Value, Channel>(slice: RadioSliceMut<Value, T, Channel>) -> Self
    where
        Value: 'static,
        Channel: RadioChannel<Value> + 'static;
}

impl<T: 'static> RadioWritable<T> for Writable<T> {
    fn from_slice<Value, Channel>(slice: RadioSliceMut<Value, T, Channel>) -> Self
    where
        Value: 'static,
        Channel: RadioChannel<Value> + 'static,
    {
        Self::new(
            Box::new({
                let slice = slice.clone();
                move || slice.peek_unchecked()
            }),
            Box::new({
                let slice = slice.clone();
                move || slice.write_unchecked_no_notify()
            }),
            Box::new({
                let slice = slice.clone();
                move || slice.subscribe_if_not()
            }),
            Box::new(move || slice.notify()),
        )
    }
}

impl<T: 'static, Value: 'static, Channel: RadioChannel<Value> + 'static> IntoWritable<T>
    for RadioSliceMut<Value, T, Channel>
{
    fn into_writable(self) -> Writable<T> {
        Writable::from_slice(self)
    }
}
