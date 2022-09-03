mod unpack;

use std::{env, fs};

use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode,
    window::PresentMode,
};
use scratch_edu_parser::Project;
use temp_dir::TempDir;

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

fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

fn load_scratch(
    mut commands: Commands,
    scratch: Res<ScratchProject>,
    asset_server: Res<AssetServer>,
) {
    for (i, target) in scratch.project.targets.iter().enumerate() {
        let costume = &target.costumes[target.current_costume];
        commands.spawn_bundle(SpriteBundle {
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
    }
}

fn main() {
    let project_path = env::args().nth(1).expect("No project path given");
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
        .run();
}
