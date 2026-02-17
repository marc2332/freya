use std::{
    cell::{
        Ref,
        RefMut,
    },
    marker::PhantomData,
    rc::Rc,
};

use freya_core::prelude::*;

use crate::hooks::{
    Radio,
    RadioChannel,
    RadioStation,
};

/// A read-only slice of a portion of the global radio state.
///
/// Components using a slice only re-render when that specific portion changes,
/// as determined by the slice's channel.
///
/// # Example
///
/// ```rust, ignore
/// let count_slice = radio.slice(AppChannel::Count, |state| &state.count);
/// child_component(count_slice)
/// ```
pub struct RadioSlice<Value, SliceValue, Channel>
where
    Channel: RadioChannel<Value>,
    Value: 'static,
    SliceValue: 'static,
{
    channel: Channel,
    station: RadioStation<Value, Channel>,
    selector: Rc<dyn Fn(&Value) -> &SliceValue>,
    _marker: PhantomData<SliceValue>,
}

impl<Value, SliceValue, Channel> Clone for RadioSlice<Value, SliceValue, Channel>
where
    Channel: RadioChannel<Value>,
    SliceValue: 'static,
{
    fn clone(&self) -> Self {
        Self {
            channel: self.channel.clone(),
            station: self.station,
            selector: self.selector.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Value, SliceValue, Channel> PartialEq for RadioSlice<Value, SliceValue, Channel>
where
    Channel: RadioChannel<Value>,
    SliceValue: 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.channel == other.channel
    }
}

impl<Value, SliceValue, Channel> RadioSlice<Value, SliceValue, Channel>
where
    Channel: RadioChannel<Value>,
    SliceValue: 'static,
{
    pub(crate) fn new(
        channel: Channel,
        station: RadioStation<Value, Channel>,
        selector: impl Fn(&Value) -> &SliceValue + 'static,
    ) -> RadioSlice<Value, SliceValue, Channel> {
        RadioSlice {
            channel,
            station,
            selector: Rc::new(selector),
            _marker: PhantomData,
        }
    }

    pub(crate) fn subscribe_if_not(&self) {
        if let Some(rc) = ReactiveContext::try_current() {
            let is_listening = self.station.is_listening(&self.channel, &rc);

            if !is_listening {
                self.station.listen(self.channel.clone(), rc);
            }
        }
    }

    /// Read the slice value and subscribe to changes.
    #[allow(invalid_reference_casting)]
    pub fn read(&'_ self) -> ReadRef<'_, SliceValue> {
        self.subscribe_if_not();
        self.peek()
    }

    /// Read the slice value and subscribe to changes, with 'static lifetime.
    #[allow(invalid_reference_casting)]
    pub fn read_unchecked(&'_ self) -> ReadRef<'static, SliceValue> {
        self.subscribe_if_not();
        self.peek_unchecked()
    }

    /// Read the slice value without subscribing.
    #[allow(invalid_reference_casting)]
    pub fn peek(&'_ self) -> ReadRef<'_, SliceValue> {
        self.peek_unchecked()
    }

    /// Read the slice value without subscribing, with 'static lifetime.
    #[allow(invalid_reference_casting)]
    pub fn peek_unchecked(&'_ self) -> ReadRef<'static, SliceValue> {
        let inner = self.station.peek_unchecked();
        inner.map(|v| {
            Ref::map(v, |v| {
                (self.selector)(unsafe { &mut *(v as *const Value as *mut Value) })
            })
        })
    }
}

/// A mutable slice of a portion of the global radio state.
///
/// Like `RadioSlice`, components only re-render when the specific portion changes.
///
/// # Example
///
/// ```rust, ignore
/// let mut count_slice = radio.slice_mut(AppChannel::Count, |state| &mut state.count);
/// child_component(count_slice)
/// ```
pub struct RadioSliceMut<Value, SliceValue, Channel>
where
    Channel: RadioChannel<Value>,
    Value: 'static,
    SliceValue: 'static,
{
    channel: Channel,
    pub(crate) station: RadioStation<Value, Channel>,
    selector: Rc<dyn Fn(&mut Value) -> &mut SliceValue>,
    _marker: PhantomData<SliceValue>,
}

impl<Value, SliceValue, Channel> Clone for RadioSliceMut<Value, SliceValue, Channel>
where
    Channel: RadioChannel<Value>,
    SliceValue: 'static,
{
    fn clone(&self) -> Self {
        Self {
            channel: self.channel.clone(),
            station: self.station,
            selector: self.selector.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Value, SliceValue, Channel> PartialEq for RadioSliceMut<Value, SliceValue, Channel>
where
    Channel: RadioChannel<Value>,
    SliceValue: 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.channel == other.channel
    }
}

impl<Value, SliceValue, Channel> RadioSliceMut<Value, SliceValue, Channel>
where
    Channel: RadioChannel<Value>,
    SliceValue: 'static,
{
    pub(crate) fn new(
        channel: Channel,
        station: RadioStation<Value, Channel>,
        selector: impl Fn(&mut Value) -> &mut SliceValue + 'static,
    ) -> RadioSliceMut<Value, SliceValue, Channel> {
        RadioSliceMut {
            channel,
            station,
            selector: Rc::new(selector),
            _marker: PhantomData,
        }
    }

    pub(crate) fn subscribe_if_not(&self) {
        if let Some(rc) = ReactiveContext::try_current() {
            let is_listening = self.station.is_listening(&self.channel, &rc);

            if !is_listening {
                self.station.listen(self.channel.clone(), rc);
            }
        }
    }

    /// Read the slice value and subscribe to changes.
    #[allow(invalid_reference_casting)]
    pub fn read(&'_ self) -> ReadRef<'_, SliceValue> {
        self.subscribe_if_not();
        self.peek()
    }

    /// Read the slice value and subscribe to changes, with 'static lifetime.
    #[allow(invalid_reference_casting)]
    pub fn read_unchecked(&'_ self) -> ReadRef<'static, SliceValue> {
        self.subscribe_if_not();
        self.peek_unchecked()
    }

    /// Read the slice value without subscribing.
    #[allow(invalid_reference_casting)]
    pub fn peek(&'_ self) -> ReadRef<'_, SliceValue> {
        self.peek_unchecked()
    }

    /// Read the slice value without subscribing, with 'static lifetime.
    #[allow(invalid_reference_casting)]
    pub fn peek_unchecked(&'_ self) -> ReadRef<'static, SliceValue> {
        let inner = self.station.peek_unchecked();
        inner.map(|v| {
            Ref::map(v, |v| {
                (self.selector)(unsafe { &mut *(v as *const Value as *mut Value) })
            })
        })
    }

    /// Write the slice value, with 'static lifetime.
    pub fn write_unchecked(&'_ self) -> WriteRef<'static, SliceValue> {
        self.notify();
        self.write_unchecked_no_notify()
    }

    /// Write the slice value without notifying.
    pub fn write_unchecked_no_notify(&'_ self) -> WriteRef<'static, SliceValue> {
        let value = self.station.value.write_unchecked();
        let selector = self.selector.clone();
        value.map(|v| RefMut::map(v, |v| selector(v)))
    }

    /// Notify listeners for this slice's channel.
    pub fn notify(&self) {
        let value = self.station.peek_unchecked();
        for channel in self.channel.clone().derive_channel(&value) {
            self.station.notify_listeners(&channel)
        }
        self.station.cleanup();
    }

    /// Write the slice value.
    pub fn write(&'_ mut self) -> WriteRef<'_, SliceValue> {
        self.write_unchecked()
    }
}

impl<Value, Channel> Radio<Value, Channel>
where
    Channel: RadioChannel<Value>,
    Value: 'static,
{
    /// Create a read-only slice of a specific portion of the state.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// let count_slice = radio.slice(AppChannel::Count, |state| &state.count);
    /// ```
    pub fn slice<SliceValue>(
        &self,
        channel: Channel,
        selector: impl Fn(&Value) -> &SliceValue + 'static,
    ) -> RadioSlice<Value, SliceValue, Channel> {
        let station = self.antenna.peek().station;
        RadioSlice::new(channel, station, selector)
    }

    /// Create a read-only slice using the current radio's channel.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// let count_slice = radio.slice_current(|state| &state.count);
    /// ```
    pub fn slice_current<SliceValue>(
        &self,
        selector: impl Fn(&Value) -> &SliceValue + 'static,
    ) -> RadioSlice<Value, SliceValue, Channel> {
        let channel = self.antenna.peek().channel.clone();
        self.slice(channel, selector)
    }

    /// Create a mutable slice of a specific portion of the state.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// let mut count_slice = radio.slice_mut(AppChannel::Count, |state| &mut state.count);
    /// ```
    pub fn slice_mut<SliceValue>(
        &self,
        channel: Channel,
        selector: impl Fn(&mut Value) -> &mut SliceValue + 'static,
    ) -> RadioSliceMut<Value, SliceValue, Channel> {
        let station = self.antenna.peek().station;
        RadioSliceMut::new(channel, station, selector)
    }

    /// Create a mutable slice using the current radio's channel.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// let mut count_slice = radio.slice_mut_current(|state| &mut state.count);
    /// ```
    pub fn slice_mut_current<SliceValue>(
        &self,
        selector: impl Fn(&mut Value) -> &mut SliceValue + 'static,
    ) -> RadioSliceMut<Value, SliceValue, Channel> {
        let channel = self.antenna.peek().channel.clone();
        self.slice_mut(channel, selector)
    }
}
