use bevy::prelude::*;

use crate::{
    animate::AnimationState,
    state::{State, StateEvent},
};

use super::{
    states::{self, Action, Direction::*},
    Player,
};

const SPEED: f32 = 200.0;

pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &states::Direction, &Action), With<Player>>,
) {
    for (mut transform, direction, action) in &mut query {
        if matches!(*action, Action::Walk) {
            let delta = time.delta_seconds();
            let movement = match *direction {
                Front => Vec3::new(0.0, -SPEED * delta, 0.0),
                Back => Vec3::new(0.0, SPEED * delta, 0.0),
                Left => Vec3::new(-SPEED * delta, 0.0, 0.0),
                Right => Vec3::new(SPEED * delta, 0.0, 0.0),
            };

            transform.translation += movement;
        }
    }
}

pub fn player_movement_event(
    input: Res<Input<KeyCode>>,
    mut action_event_writer: EventWriter<StateEvent<Action>>,
    mut direction_event_writer: EventWriter<StateEvent<states::Direction>>,
    query: Query<(Entity, &states::Direction, &Action, &AnimationState), With<Player>>,
) {
    for (id, direction, action, animation_state) in &query {
        let mut action_trans =
            |new_action| action.trans_with_event_writer(new_action, id, &mut action_event_writer);
        let mut direction_trans = |new_direction| {
            direction.trans_with_event_writer(new_direction, id, &mut direction_event_writer)
        };

        if input.pressed(KeyCode::J) {
            action_trans(Action::Attack);
        } else if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
            direction_trans(Back);
            action_trans(Action::Walk);
        } else if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            direction_trans(Front);
            action_trans(Action::Walk);
        } else if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            direction_trans(Left);
            action_trans(Action::Walk);
        } else if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            direction_trans(Right);
            action_trans(Action::Walk);
        } else {
            match action {
                Action::Attack => {
                    if matches!(
                        animation_state,
                        AnimationState::Finished | AnimationState::Ready
                    ) {
                        action_trans(Action::Idle);
                    }
                }
                _ => {
                    action_trans(Action::Idle);
                }
            }
        }
    }
}
