mod unpack;

use std::fs;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode,
    window::PresentMode,
};
use scratch_edu_parser::Project;
use temp_dir::TempDir;

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

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

struct ScratchProject {
    project: Project,
    assets: TempDir,
}

fn load_scratch(
    mut commands: Commands,
    scratch: Res<ScratchProject>,
    asset_server: Res<AssetServer>,
) {
    for (i, target) in scratch.project.targets.iter().enumerate() {
        let costume = &target.costumes[target.current_costume];
        let scale_factor = 1.0 / costume.bitmap_resolution as f32;
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load(
                scratch
                    .assets
                    .path()
                    .join(&target.costumes[target.current_costume].md5ext),
            ),
            transform: Transform::from_xyz(
                target.x.unwrap_or(0.0),
                target.y.unwrap_or(0.0),
                i as f32,
            )
            .with_scale(Vec2::splat(scale_factor).extend(1.0)),
            ..default()
        });
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}

fn main() {
    let project_path = std::env::args().skip(1).next().expect("No project path given");
    let assets = unpack::unpack_project(project_path).unwrap();
    let project = fs::read_to_string(assets.path().join("project.json")).unwrap();
    let project: Project = serde_json::from_str(&project).unwrap();

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Scratch VM".to_string(),
            width: 480.0,
            height: 360.0,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .insert_resource(ScratchProject { project, assets })
        .add_startup_system(setup_scene)
        .add_startup_system(load_scratch)
        .add_system(sprite_movement)
        .run();
}
