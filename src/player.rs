use crate::animate::{AnimationNode, AnimationTimer};
use bevy::prelude::*;

use self::{
    control::player_movement,
    states::{player_animate_indices_update, Action},
};

mod control;
mod states;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Player control plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, (player_animate_indices_update, player_movement));

        #[cfg(feature = "debug")]
        app.register_type::<states::Direction>()
            .register_type::<Action>();
    }
}

#[derive(Component)]
pub struct Player;

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/player.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 6, 10, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let direction = states::Direction::Front;
    let action = Action::Idle;
    let (first, last) = states::State(&direction, &action).indices();
    let animation_node = AnimationNode::new(first, last, true);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        direction,
        action,
        animation_node,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Once)),
        Player,
        Name::new("Player"),
    ));
}
