use std::{
    cell::RefCell,
    collections::HashMap,
    hash::Hash,
    ops::{
        Deref,
        DerefMut,
    },
    rc::Rc,
};

use freya_core::{
    integration::FxHashSet,
    prelude::*,
};

#[cfg(feature = "tracing")]
pub trait RadioChannel<T>: 'static + PartialEq + Eq + Clone + Hash + std::fmt::Debug + Ord {
    fn derive_channel(self, _radio: &T) -> Vec<Self> {
        vec![self]
    }
}

/// Defines a channel for radio communication.
/// Channels are used to subscribe to specific changes in the global state.
/// Each channel must implement this trait to be used with [`RadioStation`] and [`Radio`].
///
/// Channels allow fine-grained control over which components re-render when the state changes.
/// Components only re-render when a channel they are subscribed to is notified.
///
/// # Example
///
/// ```rust, no_run
/// # use freya::radio::*;
///
/// # struct Data;
///
/// #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
/// pub enum DataChannel {
///     ListCreation,
///     SpecificListItemUpdate(usize),
/// }
///
/// impl RadioChannel<Data> for DataChannel {}
/// ```
#[cfg(not(feature = "tracing"))]
pub trait RadioChannel<T>: 'static + PartialEq + Eq + Clone + Hash {
    /// Derive additional channels based on the current state value.
    /// This allows a single write operation to notify multiple channels.
    ///
    /// By default, returns a vector containing only `self`.
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// # use freya::radio::*;
    ///
    /// # struct Data;
    ///
    /// #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
    /// pub enum DataChannel {
    ///     All,
    ///     Specific(usize),
    /// }
    ///
    /// impl RadioChannel<Data> for DataChannel {
    ///     fn derive_channel(self, _data: &Data) -> Vec<Self> {
    ///         match self {
    ///             DataChannel::All => vec![DataChannel::All],
    ///             DataChannel::Specific(id) => vec![DataChannel::All, DataChannel::Specific(id)],
    ///         }
    ///     }
    /// }
    /// ```
    fn derive_channel(self, _radio: &T) -> Vec<Self> {
        vec![self]
    }
}

/// The central hub for global state management in Freya applications.
/// A `RadioStation` holds the global state value and manages subscriptions to different channels.
/// Components can subscribe to specific channels to receive notifications when the state changes.
///
/// RadioStations can be shared across multiple windows or components using [`use_share_radio`].
///
/// # Examples
///
/// ## Basic usage
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use freya::radio::*;
///
/// #[derive(Default)]
/// struct AppState {
///     count: i32,
/// }
///
/// #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
/// enum AppChannel {
///     Count,
/// }
///
/// impl RadioChannel<AppState> for AppChannel {}
///
/// fn app() -> impl IntoElement {
///     // Create a radio station (scoped to this component tree)
///     use_init_radio_station::<AppState, AppChannel>(AppState::default);
///
///     let mut radio = use_radio(AppChannel::Count);
///
///     rect()
///         .child(label().text(format!("Count: {}", radio.read().count)))
///         .child(
///             Button::new()
///                 .on_press(move |_| radio.write().count += 1)
///                 .child("Increment"),
///         )
/// }
/// ```
///
/// ## Global radio station for multi-window apps
///
/// ```rust, ignore
/// # use freya::prelude::*;
/// # use freya::radio::*;
///
/// let radio_station = RadioStation::create_global(AppState::default);
///
/// launch(
///     LaunchConfig::new()
///         .with_window(WindowConfig::new(Window1 { radio_station }))
///         .with_window(WindowConfig::new(Window2 { radio_station })),
/// );
/// ```
pub struct RadioStation<Value, Channel>
where
    Channel: RadioChannel<Value>,
    Value: 'static,
{
    pub(crate) value: State<Value>,
    listeners: State<HashMap<Channel, Rc<RefCell<FxHashSet<ReactiveContext>>>>>,
}

impl<Value, Channel> Clone for RadioStation<Value, Channel>
where
    Channel: RadioChannel<Value>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Value, Channel> Copy for RadioStation<Value, Channel> where Channel: RadioChannel<Value> {}

impl<Value, Channel> RadioStation<Value, Channel>
where
    Channel: RadioChannel<Value>,
{
    pub(crate) fn create(init_value: Value) -> Self {
        RadioStation {
            value: State::create(init_value),
            listeners: State::create(HashMap::default()),
        }
    }

    /// Create a global `RadioStation` that lives for the entire application lifetime.
    /// This is useful for sharing state across multiple windows.
    ///
    /// You would usually want to call this in your `main` function, not anywhere else.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// # use freya::prelude::*;
    /// # use freya::radio::*;
    ///
    /// let radio_station = RadioStation::create_global(AppState::default);
    ///
    /// launch(
    ///     LaunchConfig::new()
    ///         .with_window(WindowConfig::new(Window1 { radio_station }))
    ///         .with_window(WindowConfig::new(Window2 { radio_station })),
    /// );
    /// ```
    pub fn create_global(init_value: Value) -> Self {
        RadioStation {
            value: State::create_global(init_value),
            listeners: State::create_global(HashMap::default()),
        }
    }

    pub(crate) fn is_listening(
        &self,
        channel: &Channel,
        reactive_context: &ReactiveContext,
    ) -> bool {
        let listeners = self.listeners.peek();
        listeners
            .get(channel)
            .map(|contexts| contexts.borrow().contains(reactive_context))
            .unwrap_or_default()
    }

    pub(crate) fn listen(&self, channel: Channel, mut reactive_context: ReactiveContext) {
        let mut listeners = self.listeners.write_unchecked();
        let listeners = listeners.entry(channel).or_default();
        reactive_context.subscribe(listeners);
    }

    pub(crate) fn notify_listeners(&self, channel: &Channel) {
        let listeners = self.listeners.write_unchecked();

        #[cfg(feature = "tracing")]
        tracing::info!("Notifying {channel:?}");

        for (listener_channel, listeners) in listeners.iter() {
            if listener_channel == channel {
                for reactive_context in listeners.borrow().iter() {
                    reactive_context.notify();
                }
            }
        }
    }

    /// Read the current state value and subscribe to all channel changes.
    /// Any component calling this will re-render when any channel is notified.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// # use freya::radio::*;
    /// let value = radio_station.read();
    /// ```
    pub fn read(&'_ self) -> ReadRef<'_, Value> {
        self.value.read()
    }

    pub fn peek_unchecked(&self) -> ReadRef<'static, Value> {
        self.value.peek()
    }

    pub fn write_unchecked(&self) -> WriteRef<'static, Value> {
        self.value.write_unchecked()
    }

    /// Read the current state value without subscribing to changes.
    /// Components using this will not re-render when the state changes.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// # use freya::radio::*;
    /// let value = radio_station.peek();
    /// ```
    pub fn peek(&'_ self) -> ReadRef<'_, Value> {
        self.value.peek()
    }

    pub(crate) fn cleanup(&self) {
        let mut listeners = self.listeners.write_unchecked();

        // Clean up those channels with no reactive contexts
        listeners.retain(|_, listeners| !listeners.borrow().is_empty());

        #[cfg(feature = "tracing")]
        {
            use itertools::Itertools;
            use tracing::{
                Level,
                info,
                span,
            };

            let mut channels_subscribers = HashMap::<&Channel, usize>::new();

            for (channel, listeners) in listeners.iter() {
                *channels_subscribers.entry(&channel).or_default() = listeners.borrow().len();
            }

            let span = span!(Level::DEBUG, "Radio Station Metrics");
            let _enter = span.enter();

            for (channel, count) in channels_subscribers.iter().sorted() {
                info!(" {count} subscribers for {channel:?}")
            }
        }
    }

    /// Modify the state using a specific channel.
    /// This will notify all subscribers to that channel (and any derived channels).
    ///
    /// Returns a [`RadioGuard`] that allows direct mutation of the state.
    /// The guard automatically notifies listeners when dropped.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// # use freya::radio::*;
    /// radio_station.write_channel(MyChannel::Update).count += 1;
    /// ```
    pub fn write_channel(&mut self, channel: Channel) -> RadioGuard<Value, Channel> {
        let value = self.value.write_unchecked();
        RadioGuard {
            channels: channel.clone().derive_channel(&*value),
            station: *self,
            value,
        }
    }
}

pub struct RadioAntenna<Value, Channel>
where
    Channel: RadioChannel<Value>,
    Value: 'static,
{
    pub(crate) channel: Channel,
    pub(crate) station: RadioStation<Value, Channel>,
}

impl<Value, Channel> RadioAntenna<Value, Channel>
where
    Channel: RadioChannel<Value>,
{
    pub(crate) fn new(
        channel: Channel,
        station: RadioStation<Value, Channel>,
    ) -> RadioAntenna<Value, Channel> {
        RadioAntenna { channel, station }
    }
}
impl<Value, Channel> Clone for RadioAntenna<Value, Channel>
where
    Channel: RadioChannel<Value>,
{
    fn clone(&self) -> Self {
        Self {
            channel: self.channel.clone(),
            station: self.station,
        }
    }
}

pub struct RadioGuard<Value, Channel>
where
    Channel: RadioChannel<Value>,
    Value: 'static,
{
    pub(crate) station: RadioStation<Value, Channel>,
    pub(crate) channels: Vec<Channel>,
    pub(crate) value: WriteRef<'static, Value>,
}

impl<Value, Channel> Drop for RadioGuard<Value, Channel>
where
    Channel: RadioChannel<Value>,
{
    fn drop(&mut self) {
        for channel in &mut self.channels {
            self.station.notify_listeners(channel)
        }
        if !self.channels.is_empty() {
            self.station.cleanup();
        }
    }
}

impl<Value, Channel> Deref for RadioGuard<Value, Channel>
where
    Channel: RadioChannel<Value>,
{
    type Target = WriteRef<'static, Value>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<Value, Channel> DerefMut for RadioGuard<Value, Channel>
where
    Channel: RadioChannel<Value>,
{
    fn deref_mut(&mut self) -> &mut WriteRef<'static, Value> {
        &mut self.value
    }
}

/// A reactive handle to the global state for a specific channel.
/// `Radio` provides methods to read and write the global state, and automatically subscribes
/// the current component to re-render when the associated channel is notified.
///
/// Each `Radio` instance is tied to a specific channel, allowing fine-grained control
/// over which components update when the state changes.
///
/// # Examples
///
/// ## Basic usage
///
/// ```rust, ignore
/// # use freya::prelude::*;
/// # use freya::radio::*;
///
/// #[derive(PartialEq)]
/// struct MyComponent {}
///
/// impl Component for MyComponent {
///     fn render(&self) -> impl IntoElement {
///         let mut radio = use_radio(MyChannel::Count);
///
///         rect()
///             .child(label().text(format!("Count: {}", radio.read().count)))
///             .child(
///                 Button::new()
///                     .on_press(move |_| radio.write().count += 1)
///                     .child("Increment"),
///             )
///     }
/// }
/// ```
///
/// ## Using reducers
///
/// ```rust, ignore
/// # use freya::prelude::*;
/// # use freya::radio::*;
///
/// #[derive(Clone)]
/// struct CounterState {
///     count: i32,
/// }
///
/// impl DataReducer for CounterState {
///     type Channel = CounterChannel;
///     type Action = CounterAction;
///
///     fn reduce(&mut self, action: CounterAction) -> ChannelSelection<CounterChannel> {
///         match action {
///             CounterAction::Increment => self.count += 1,
///             CounterAction::Decrement => self.count -= 1,
///         }
///         ChannelSelection::Current
///     }
/// }
///
/// #[derive(PartialEq)]
/// struct CounterComponent {}
///
/// impl Component for CounterComponent {
///     fn render(&self) -> impl IntoElement {
///         let mut radio = use_radio(CounterChannel::Count);
///
///         rect()
///             .child(
///                 Button::new()
///                     .on_press(move |_| radio.apply(CounterAction::Increment))
///                     .child("+"),
///             )
///             .child(label().text(format!("{}", radio.read().count)))
///             .child(
///                 Button::new()
///                     .on_press(move |_| radio.apply(CounterAction::Decrement))
///                     .child("-"),
///             )
///     }
/// }
/// ```
pub struct Radio<Value, Channel>
where
    Channel: RadioChannel<Value>,
    Value: 'static,
{
    pub(crate) antenna: State<RadioAntenna<Value, Channel>>,
}

impl<Value, Channel> Clone for Radio<Value, Channel>
where
    Channel: RadioChannel<Value>,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<Value, Channel> Copy for Radio<Value, Channel> where Channel: RadioChannel<Value> {}

impl<Value, Channel> PartialEq for Radio<Value, Channel>
where
    Channel: RadioChannel<Value>,
{
    fn eq(&self, other: &Self) -> bool {
        self.antenna == other.antenna
    }
}

impl<Value, Channel> Radio<Value, Channel>
where
    Channel: RadioChannel<Value>,
{
    pub(crate) fn new(antenna: State<RadioAntenna<Value, Channel>>) -> Radio<Value, Channel> {
        Radio { antenna }
    }

    pub(crate) fn subscribe_if_not(&self) {
        if let Some(rc) = ReactiveContext::try_current() {
            let antenna = &self.antenna.write_unchecked();
            let channel = antenna.channel.clone();
            let is_listening = antenna.station.is_listening(&channel, &rc);

            // Subscribe the reader reactive context to the channel if it wasn't already
            if !is_listening {
                antenna.station.listen(channel, rc);
            }
        }
    }

    /// Read the current state value and subscribe the current component to changes
    /// on this radio's channel. The component will re-render when this channel is notified.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// # use freya::radio::*;
    /// let count = radio.read().count;
    /// ```
    pub fn read(&'_ self) -> ReadRef<'_, Value> {
        self.subscribe_if_not();
        self.antenna.peek().station.value.peek()
    }

    /// Read the current state value inside a callback.
    ///
    /// Example:
    ///
    /// ```rust, ignore
    /// # use freya::radio::*;
    /// radio.with(|value| {
    ///     // Do something with `value`
    /// });
    /// ```
    pub fn with(&self, cb: impl FnOnce(ReadRef<Value>)) {
        self.subscribe_if_not();
        let value = self.antenna.peek().station.value;
        let borrow = value.read();
        cb(borrow);
    }

    /// Get a mutable reference to the state for writing.
    /// Changes will notify subscribers to this radio's channel.
    ///
    /// Returns a [`RadioGuard`] that allows direct mutation of the state.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// # use freya::radio::*;
    /// radio.write().count += 1;
    /// ```
    pub fn write(&mut self) -> RadioGuard<Value, Channel> {
        let value = self.antenna.peek().station.value.write_unchecked();
        let channel = self.antenna.peek().channel.clone();
        RadioGuard {
            channels: channel.derive_channel(&*value),
            station: self.antenna.read().station,
            value,
        }
    }

    /// Get a mutable reference to the current state value, inside a callback.
    ///
    /// Example:
    ///
    /// ```rust, ignore
    /// # use freya::radio::*;
    /// radio.write_with(|value| {
    ///     // Modify `value`
    /// });
    /// ```
    pub fn write_with(&mut self, cb: impl FnOnce(RadioGuard<Value, Channel>)) {
        let guard = self.write();
        cb(guard);
    }

    /// Modify the state using a custom Channel.
    ///
    /// ## Example:
    /// ```rust, ignore
    /// # use freya::radio::*;
    /// radio.write(Channel::Whatever).value = 1;
    /// ```
    pub fn write_channel(&mut self, channel: Channel) -> RadioGuard<Value, Channel> {
        let value = self.antenna.peek().station.value.write_unchecked();
        RadioGuard {
            channels: channel.derive_channel(&*value),
            station: self.antenna.read().station,
            value,
        }
    }

    /// Get a mutable reference to the current state value, inside a callback.
    ///
    /// Example:
    ///
    /// ```rust, ignore
    /// # use freya::radio::*;
    /// radio.write_channel_with(Channel::Whatever, |value| {
    ///     // Modify `value`
    /// });
    /// ```
    pub fn write_channel_with(
        &mut self,
        channel: Channel,
        cb: impl FnOnce(RadioGuard<Value, Channel>),
    ) {
        let guard = self.write_channel(channel);
        cb(guard);
    }

    /// Get a mutable reference to the current state value, inside a callback that returns the channel to be used.
    ///
    /// Example:
    ///
    /// ```rust, ignore
    /// # use freya::radio::*;
    /// radio.write_with_channel_selection(|value| {
    ///     // Modify `value`
    ///     if value.cool {
    ///         ChannelSelection::Select(Channel::Whatever)
    ///     } else {
    ///         ChannelSelection::Silence
    ///     }
    /// });
    /// ```
    pub fn write_with_channel_selection(
        &mut self,
        cb: impl FnOnce(&mut Value) -> ChannelSelection<Channel>,
    ) -> ChannelSelection<Channel> {
        let value = self.antenna.peek().station.value.write_unchecked();
        let mut guard = RadioGuard {
            channels: Vec::default(),
            station: self.antenna.read().station,
            value,
        };
        let channel_selection = cb(&mut guard.value);
        let channel = match channel_selection.clone() {
            ChannelSelection::Current => Some(self.antenna.peek().channel.clone()),
            ChannelSelection::Silence => None,
            ChannelSelection::Select(c) => Some(c),
        };
        if let Some(channel) = channel {
            for channel in channel.derive_channel(&guard.value) {
                self.antenna.peek().station.notify_listeners(&channel)
            }
            self.antenna.peek().station.cleanup();
        }

        channel_selection
    }

    /// Modify the state silently, no component will be notified.
    ///
    /// This is not recommended, the only intended usage for this is inside [RadioAsyncReducer].
    pub fn write_silently(&mut self) -> RadioGuard<Value, Channel> {
        let value = self.antenna.peek().station.value.write_unchecked();
        RadioGuard {
            channels: Vec::default(),
            station: self.antenna.read().station,
            value,
        }
    }
}

impl<Channel> Copy for ChannelSelection<Channel> where Channel: Copy {}

#[derive(Clone)]
pub enum ChannelSelection<Channel> {
    /// Notify the channel associated with the used [Radio].
    Current,
    /// Notify a given `Channel`.
    Select(Channel),
    /// No subscriber will be notified.
    Silence,
}

impl<Channel> ChannelSelection<Channel> {
    /// Change to [ChannelSelection::Current]
    pub fn current(&mut self) {
        *self = Self::Current
    }

    /// Change to [ChannelSelection::Select]
    pub fn select(&mut self, channel: Channel) {
        *self = Self::Select(channel)
    }

    /// Change to [ChannelSelection::Silence]
    pub fn silence(&mut self) {
        *self = Self::Silence
    }

    /// Check if it is of type [ChannelSelection::Current]
    pub fn is_current(&self) -> bool {
        matches!(self, Self::Current)
    }

    /// Check if it is of type [ChannelSelection::Select] and return the channel.
    pub fn is_select(&self) -> Option<&Channel> {
        match self {
            Self::Select(channel) => Some(channel),
            _ => None,
        }
    }

    /// Check if it is of type [ChannelSelection::Silence]
    pub fn is_silence(&self) -> bool {
        matches!(self, Self::Silence)
    }
}

/// Provide an existing [`RadioStation`] to descendant components.
/// This is useful for sharing the same global state across different parts of the component tree
/// or across multiple windows.
pub fn use_share_radio<Value, Channel>(radio: impl FnOnce() -> RadioStation<Value, Channel>)
where
    Channel: RadioChannel<Value>,
    Value: 'static,
{
    use_provide_context(radio);
}

/// Subscribe to the global state for a specific channel.
/// Returns a [`Radio`] handle that allows reading and writing the state.
/// The current component will re-render whenever the specified channel is notified.
///
/// This hook must be called within a component that has access to a [`RadioStation`]
/// (either through [`use_init_radio_station`] or [`use_share_radio`]).
///
/// # Example
///
/// ```rust, ignore
/// # use freya::prelude::*;
/// # use freya::radio::*;
///
/// fn app() -> impl IntoElement {
///     use_init_radio_station::<AppState, AppChannel>(AppState::default);
///
///     rect().child(Counter {})
/// }
///
/// #[derive(PartialEq)]
/// struct Counter {}
///
/// impl Component for Counter {
///     fn render(&self) -> impl IntoElement {
///         let mut radio = use_radio(AppChannel::Count);
///
///         rect()
///             .child(label().text(format!("Count: {}", radio.read().count)))
///             .child(
///                 Button::new()
///                     .on_press(move |_| radio.write().count += 1)
///                     .child("+"),
///             )
///     }
/// }
/// ```
pub fn use_radio<Value, Channel>(channel: Channel) -> Radio<Value, Channel>
where
    Channel: RadioChannel<Value>,
    Value: 'static,
{
    let station = use_consume::<RadioStation<Value, Channel>>();

    let mut radio = use_hook(|| {
        let antenna = RadioAntenna::new(channel.clone(), station);
        Radio::new(State::create(antenna))
    });

    if radio.antenna.peek().channel != channel {
        radio.antenna.write().channel = channel;
    }

    radio
}

/// Initialize a new radio station in the current component tree.
/// This provides the global state to all descendant components.
///
/// Returns the [`RadioStation`] instance for direct access if needed.
///
/// # Example
///
/// ```rust, ignore
/// # use freya::prelude::*;
/// # use freya::radio::*;
///
/// fn app() -> impl IntoElement {
///     use_init_radio_station::<AppState, AppChannel>(AppState::default);
///
///     rect().child(MyComponent {})
/// }
/// ```
pub fn use_init_radio_station<Value, Channel>(
    init_value: impl FnOnce() -> Value,
) -> RadioStation<Value, Channel>
where
    Channel: RadioChannel<Value>,
    Value: 'static,
{
    use_provide_context(|| RadioStation::create(init_value()))
}

pub fn use_radio_station<Value, Channel>() -> RadioStation<Value, Channel>
where
    Channel: RadioChannel<Value>,
    Value: 'static,
{
    use_consume::<RadioStation<Value, Channel>>()
}

/// Trait for implementing a reducer pattern on your state.
/// Reducers allow you to define actions that modify the state in a controlled way.
///
/// Implement this trait on your state type to enable the [`RadioReducer`] functionality.
///
/// # Example
///
/// ```rust, ignore
/// # use freya::radio::*;
///
/// #[derive(Clone)]
/// struct Counter {
///     count: i32,
/// }
///
/// #[derive(Clone)]
/// enum CounterAction {
///     Increment,
///     Decrement,
///     Set(i32),
/// }
///
/// impl DataReducer for Counter {
///     type Channel = CounterChannel;
///     type Action = CounterAction;
///
///     fn reduce(&mut self, action: Self::Action) -> ChannelSelection<Self::Channel> {
///         match action {
///             CounterAction::Increment => self.count += 1,
///             CounterAction::Decrement => self.count -= 1,
///             CounterAction::Set(value) => self.count = value,
///         }
///         ChannelSelection::Current
///     }
/// }
/// ```
pub trait DataReducer {
    type Channel;
    type Action;

    fn reduce(&mut self, action: Self::Action) -> ChannelSelection<Self::Channel>;
}

pub trait RadioReducer {
    type Action;
    type Channel;

    fn apply(&mut self, action: Self::Action) -> ChannelSelection<Self::Channel>;
}

impl<Data: DataReducer<Channel = Channel, Action = Action>, Channel: RadioChannel<Data>, Action>
    RadioReducer for Radio<Data, Channel>
{
    type Action = Action;
    type Channel = Channel;

    fn apply(&mut self, action: Action) -> ChannelSelection<Channel> {
        self.write_with_channel_selection(|data| data.reduce(action))
    }
}

pub trait DataAsyncReducer {
    type Channel;
    type Action;

    #[allow(async_fn_in_trait)]
    async fn async_reduce(
        _radio: &mut Radio<Self, Self::Channel>,
        _action: Self::Action,
    ) -> ChannelSelection<Self::Channel>
    where
        Self::Channel: RadioChannel<Self>,
        Self: Sized;
}

pub trait RadioAsyncReducer {
    type Action;

    fn async_apply(&mut self, _action: Self::Action)
    where
        Self::Action: 'static;
}

impl<
    Data: DataAsyncReducer<Channel = Channel, Action = Action>,
    Channel: RadioChannel<Data>,
    Action,
> RadioAsyncReducer for Radio<Data, Channel>
{
    type Action = Action;

    fn async_apply(&mut self, action: Self::Action)
    where
        Self::Action: 'static,
    {
        let mut radio = *self;
        spawn(async move {
            let channel = Data::async_reduce(&mut radio, action).await;
            radio.write_with_channel_selection(|_| channel);
        });
    }
}
