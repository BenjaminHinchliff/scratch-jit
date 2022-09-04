mod interpreter;
mod renderer;
mod unpack;

use std::fs;

use bevy::{prelude::*, window::PresentMode};
use clap::Parser;
use interpreter::InterpreterPlugin;
use renderer::Renderer;
use scratch_edu_parser::Project;
use temp_dir::TempDir;

pub struct ScratchProject {
    pub project: Project,
    pub assets: TempDir,
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    project: String,
}

fn main() {
    let cli = Cli::parse();

    let assets = unpack::unpack_project(cli.project).unwrap();
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
        .add_plugin(Renderer)
        .add_plugin(InterpreterPlugin)
        .run();
}
