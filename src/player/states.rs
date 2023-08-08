use bevy::prelude::*;

use crate::animate::AnimationNode;

use super::Player;

#[derive(Component, Clone, Copy, Reflect)]
pub enum Direction {
    Front,
    Back,
    Left,
    Right,
}

#[derive(Component, Clone, Copy, Reflect)]
pub enum Action {
    Idle,
    Walk,
    Attack,
    Dead,
}

pub struct State<'a>(pub &'a Direction, pub &'a Action);

impl<'a> State<'a> {
    pub fn indices(&self) -> (usize, usize) {
        use crate::player::states::Direction::*;
        use Action::*;

        match (self.0, self.1) {
            (Front, Idle) => (0, 5),
            (Back, Idle) => (12, 17),
            (_, Idle) => (6, 11),
            (Front, Walk) => (18, 23),
            (Back, Walk) => (30, 35),
            (_, Walk) => (24, 29),
            (Front, Attack) => (36, 39),
            (Back, Attack) => (48, 51),
            (_, Attack) => (42, 45),
            (_, Dead) => (54, 56),
        }
    }
}

pub fn player_animate_indices_update(
    mut query: Query<
        (
            &Direction,
            &Action,
            &mut AnimationNode,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    for (direction, action, mut node, mut sprite) in &mut query {
        node.set_indices(State(direction, action).indices());
        node.set_repeate(matches!(action, Action::Idle | Action::Walk));
        sprite.flip_x = matches!(direction, Direction::Left);
    }
}
