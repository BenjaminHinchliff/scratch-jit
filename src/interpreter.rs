use bevy::{prelude::*, time::FixedTimestep};

use crate::renderer::ScratchSprite;

const REFRESH_TIMESTEP: f64 = 1.0 / 30.0;

fn circle_test(time: Res<Time>, mut position: Query<&mut Transform, With<ScratchSprite>>) {
    let theta = time.seconds_since_startup() as f32;
    for mut transform in &mut position {
        transform.translation.x = theta.cos() * 100.0;
        transform.translation.y = theta.sin() * 100.0;
    }
}

pub struct InterpreterPlugin;

impl Plugin for InterpreterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(REFRESH_TIMESTEP))
                .with_system(circle_test),
        );
    }
}
