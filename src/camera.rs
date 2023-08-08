use bevy::{prelude::*, render::camera};
use bevy_ecs_ldtk::{LdtkLevel, LevelSelection};

use crate::player::Player;

const ASPECT_RATION: f32 = 16. / 9.;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_fit_inside_current_level);
    }
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (&mut OrthographicProjection, &mut Transform),
        (With<MainCamera>, Without<Player>),
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<(&Transform, &Handle<LdtkLevel>), (Without<MainCamera>, Without<Player>)>,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        if let Ok((mut orthographic_projection, mut camera_transform)) =
            camera_query.get_single_mut()
        {
            for (level_transform, level_handle) in &level_query {
                if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                    let level = &ldtk_level.level;
                    if level_selection.is_match(&0, level) {
                        let level_ratio = level.px_wid as f32 / level.px_hei as f32;
                        orthographic_projection.viewport_origin = Vec2::ZERO;
                        if level_ratio > ASPECT_RATION {
                            let height = (level.px_hei as f32 / 9.).round() * 9.;
                            let width = height * ASPECT_RATION;
                            orthographic_projection.scaling_mode =
                                camera::ScalingMode::Fixed { width, height };
                            camera_transform.translation.x =
                                (player_translation.x - level_transform.translation.x - width / 2.)
                                    .clamp(0., level.px_wid as f32 - width);
                            camera_transform.translation.y = 0.;
                        } else {
                            let width = (level.px_wid as f32 / 16.).round() * 16.;
                            let height = width / ASPECT_RATION;
                            orthographic_projection.scaling_mode =
                                camera::ScalingMode::Fixed { width, height };
                            camera_transform.translation.y = (player_translation.y
                                - level_transform.translation.y
                                - height / 2.)
                                .clamp(0., level.px_hei as f32 - height);
                            camera_transform.translation.x = 0.;
                        }

                        camera_transform.translation.x += level_transform.translation.x;
                        camera_transform.translation.y += level_transform.translation.y;
                    }
                }
            }
        };
    }
}
