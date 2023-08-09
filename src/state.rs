use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Default)]
pub struct StatePlugin<S: State>(PhantomData<S>);

impl<S: State> Plugin for StatePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_event::<StateEvent<S>>()
            .add_systems(Update, state_update::<S>);
    }
}

pub trait State: Component + Clone + PartialEq {
    fn trans_to(&self, new_state: Self) -> (Event<Self>, Event<Self>) {
        (Event::Enter(new_state), Event::Leaving(self.clone()))
    }

    fn trans_with_event_writer(
        &self,
        new_state: Self,
        id: Entity,
        writer: &mut EventWriter<StateEvent<Self>>,
    ) {
        if *self != new_state {
            let (enter_event, leaving_event) = self.trans_to(new_state);
            writer.send(StateEvent::new(id, enter_event));
            writer.send(StateEvent::new(id, leaving_event));
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event<S> {
    Enter(S),
    Leaving(S),
}

#[derive(Event)]
pub struct StateEvent<S> {
    id: Entity,
    event: Event<S>,
}

impl<S> StateEvent<S> {
    pub fn new(id: Entity, event: Event<S>) -> Self {
        Self { id, event }
    }
}

fn state_update<S>(mut state_events: EventReader<StateEvent<S>>, mut query: Query<&mut S>)
where
    S: State,
{
    for StateEvent { id, event } in state_events.iter() {
        if let Event::Enter(state) = event {
            if let Ok(mut old_state) = query.get_mut(*id) {
                *old_state = state.clone();
            }
        }
    }
}
