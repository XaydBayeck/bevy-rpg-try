use bevy::prelude::*;

use super::{
    states::{self, Action, Direction::*},
    Player,
};

const SPEED: f32 = 500.0;

pub fn player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut states::Direction, &mut Action), With<Player>>,
) {
    for (mut transform, mut direction, mut action) in &mut query {
        if input.pressed(KeyCode::J) {
            *action = Action::Attack;
        } else if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
            *direction = Back;
            *action = Action::Walk;
        } else if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            *direction = Front;
            *action = Action::Walk;
        } else if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            *direction = Left;
            *action = Action::Walk;
        } else if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            *direction = Right;
            *action = Action::Walk;
        } else {
            *action = Action::Idle;
        }

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
