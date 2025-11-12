use crate::{
    EmmitableEvent,
    NameOfEvent,
    NodeKey,
    NodesState,
    NodesStatesUpdate,
    PotentialEvent,
    SourceEvent,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ProcessedEvents<
    Key: NodeKey,
    Name: NameOfEvent,
    Emmitable: EmmitableEvent,
    Source: SourceEvent,
> {
    pub emmitable_events: Vec<Emmitable>,
    pub flattened_potential_events: Vec<PotentialEvent<Key, Name, Source>>,
    pub nodes_states_update: NodesStatesUpdate<Key>,
}

impl<Key: NodeKey, Name: NameOfEvent, Emmitable: EmmitableEvent, Source: SourceEvent> Default
    for ProcessedEvents<Key, Name, Emmitable, Source>
{
    fn default() -> Self {
        Self {
            emmitable_events: Vec::default(),
            flattened_potential_events: Vec::default(),
            nodes_states_update: NodesStatesUpdate::default(),
        }
    }
}

pub trait EventsExecutor
where
    Self: std::marker::Sized,
{
    type Name: NameOfEvent;
    type Key: NodeKey;
    type Emmitable: EmmitableEvent<Key = Self::Key, Name = Self::Name>;
    type Source: SourceEvent;

    /// Call the event handler of the given [Self::Emmitable].
    fn emit_event(&mut self, event: Self::Emmitable) -> bool;

    // All events have been emitted
    fn emitted_events(&mut self) {}
}

impl<T: EventsExecutor> private::Sealed for T {}

impl<T: EventsExecutor + private::Sealed> EventsExecutorRunner for T {
    type Name = T::Name;
    type Key = T::Key;
    type Emmitable = T::Emmitable;
    type Source = T::Source;
    fn run(
        mut self,
        nodes_state: &mut NodesState<Self::Key>,
        ProcessedEvents {
            mut emmitable_events,
            flattened_potential_events,
            mut nodes_states_update,
        }: ProcessedEvents<Self::Key, Self::Name, Self::Emmitable, Self::Source>,
    ) {
        let mut processed_events = Vec::<Self::Emmitable>::new();

        #[cfg(debug_assertions)]
        tracing::info!("Processing {} Tree events", emmitable_events.len());

        while !emmitable_events.is_empty() {
            let emmitable_event = emmitable_events.remove(0);

            let default_action_enabled = self.emit_event(emmitable_event.clone());

            if !default_action_enabled {
                // Get the events that this event can cancel
                let cancellable_events = emmitable_event.name().get_cancellable_events();

                // Remove the rest of emmitable events that are cancellable
                emmitable_events.retain(|event| !cancellable_events.contains(&event.name()));

                // Discard the potential events that dont find a matching emmitable event
                // So for instance, a cancelled potential mousemove event wont be discarded if a emmitable mousenter was processed before
                // At the same time, a cancelled potential mousemove event that actually gets discarded will only discard the node state
                // made in this run, but will not change what was already before this run
                // So if the affected node was already being hovered from the last events run, it will continue to be as so
                for potential_event in &flattened_potential_events {
                    let is_cancellable = cancellable_events.contains(&potential_event.name);
                    if is_cancellable {
                        let processed_event = processed_events.iter().find(|event| {
                            potential_event.name == event.source()
                                && potential_event.node_key == event.key()
                        });
                        if processed_event.is_none() {
                            nodes_states_update
                                .discard(&potential_event.name, &potential_event.node_key);
                        }
                    }
                }
            }

            processed_events.push(emmitable_event);
        }

        self.emitted_events();

        nodes_state.apply_update(nodes_states_update);
    }
}

pub trait EventsExecutorRunner: private::Sealed
where
    Self: std::marker::Sized,
{
    type Name: NameOfEvent;
    type Key: NodeKey;
    type Emmitable: EmmitableEvent<Key = Self::Key, Name = Self::Name>;
    type Source: SourceEvent;

    fn run(
        self,
        nodes_state: &mut NodesState<Self::Key>,
        processed_events: ProcessedEvents<Self::Key, Self::Name, Self::Emmitable, Self::Source>,
    );
}

#[doc(hidden)]
mod private {
    pub trait Sealed {}
}
