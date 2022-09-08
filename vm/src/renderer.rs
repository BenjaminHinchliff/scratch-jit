use crate::{interpreter::ScratchInterpreter, ScratchProject};
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};
use scratch_edu_parser::project;

fn setup_scene(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            left: -240.0,
            right: 240.0,
            top: 180.0,
            bottom: -180.0,
            scaling_mode: ScalingMode::Auto {
                min_width: 480.0,
                min_height: 360.0,
            },
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::WHITE),
        },
        ..default()
    });
}

fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

#[derive(Debug, Component)]
pub struct ScratchTarget(pub project::Target);

#[derive(Debug, Component)]
pub struct ScratchSprite;

fn load_scratch(
    mut commands: Commands,
    scratch: Res<ScratchProject>,
    asset_server: Res<AssetServer>,
) {
    for (i, target) in scratch.project.targets.iter().enumerate() {
        let costume = &target.costumes[target.current_costume];
        let mut target_ent = commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load(
                scratch
                    .assets
                    .path()
                    .join(&target.costumes[target.current_costume].md5ext),
            ),
            transform: Transform {
                translation: Vec3::new(target.x.unwrap_or(0.0), target.y.unwrap_or(0.0), i as f32),
                rotation: Quat::from_axis_angle(
                    Vec3::Z,
                    deg_to_rad(target.direction.map(|t| t - 90.0).unwrap_or(0.0)),
                ),
                scale: Vec2::splat(
                    1.0 / costume.bitmap_resolution as f32
                        * (target.size.map(|t| t / 100.0).unwrap_or(1.0)),
                )
                .extend(1.0),
            },
            ..default()
        });
        target_ent
            .insert(ScratchTarget(target.clone()))
            .insert(ScratchInterpreter::new(&target.blocks));
        if !target.is_stage {
            target_ent.insert(ScratchSprite);
        }
    }
}

fn update_sprite_transforms(
    mut sprites: Query<(&mut Transform, &ScratchTarget), With<ScratchSprite>>,
) {
    for (mut transform, ScratchTarget(sprite)) in &mut sprites {
        transform.translation.x = sprite.x.unwrap();
        transform.translation.y = sprite.y.unwrap();
    }
}

pub struct Renderer;

impl Plugin for Renderer {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_scene)
            .add_startup_system(load_scratch)
            .add_system(update_sprite_transforms);
    }
}
