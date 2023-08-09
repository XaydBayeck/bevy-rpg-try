use bevy::prelude::*;

use crate::state::{State, StateEvent, StatePlugin};

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn name(&self) -> &str {
        "Stateful Animate Play Plugin"
    }

    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(StatePlugin::<AnimationState>::default())
            .add_systems(Update, animate_sprite);

        #[cfg(feature = "debug")]
        app.register_type::<AnimationState>()
            .register_type::<AnimationNode>()
            .register_type::<AnimationTimer>();
    }
}

#[derive(Component, Reflect, Clone, Copy, PartialEq)]
pub enum AnimationState {
    Ready,
    Playing,
    Puase,
    Finished,
}
//
// impl AnimationState {
//     pub fn is_ready(&self) -> bool {
//         matches!(self, Self::Ready)
//     }
//     pub fn is_playing(&self) -> bool {
//         matches!(self, Self::Playing)
//     }
//     pub fn is_puase(&self) -> bool {
//         matches!(self, Self::Puase)
//     }
//     pub fn is_finished(&self) -> bool {
//         matches!(self, Self::Finished)
//     }
// }

impl State for AnimationState {}

impl Default for AnimationState {
    fn default() -> Self {
        Self::Ready
    }
}

#[derive(Component, Reflect)]
pub struct AnimationNode {
    index: usize,
    first: usize,
    last: usize,
    repeat: bool,
}

impl AnimationNode {
    pub fn new(first: usize, last: usize, repeat: bool) -> Self {
        Self {
            index: first,
            first,
            last,
            repeat,
        }
    }

    pub fn set_repeate(&mut self, turn: bool) {
        self.repeat = turn;
    }

    pub fn set_indices(&mut self, (first, last): (usize, usize)) {
        self.first = first;
        self.last = last;
    }

    pub fn next(&mut self) -> Option<usize> {
        if self.index > self.last {
            None
        } else if self.index <= self.first {
            self.index = self.first + 1;
            Some(self.first)
        } else {
            let result = Some(self.index);
            self.index += 1;
            result
        }
    }

    pub fn reset(&mut self) {
        self.index = self.first;
    }
}

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct AnimationTimer(pub Timer);

fn animate_sprite(
    time: Res<Time>,
    mut animate_state_events_writer: EventWriter<StateEvent<AnimationState>>,
    mut query: Query<(
        Entity,
        &AnimationState,
        &mut AnimationNode,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (id, state, mut node, mut timer, mut sprite) in &mut query {
        let mut trans_state = |new_state| {
            state.trans_with_event_writer(new_state, id, &mut animate_state_events_writer)
        };

        match state {
            AnimationState::Playing => {
                if timer.tick(time.delta()).finished() {
                    if let Some(index) = node.next() {
                        sprite.index = index;
                        timer.reset();
                        timer.unpause();
                    } else {
                        trans_state(AnimationState::Finished);
                    }
                };
            }
            AnimationState::Finished => {
                node.reset();
                trans_state(AnimationState::Ready);
            }
            AnimationState::Ready => {
                if node.repeat {
                    trans_state(AnimationState::Playing);
                }
            }
            _ => (),
        }
    }
}
