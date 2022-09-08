use crate::renderer::ScratchTarget;
use bevy::{prelude::*, time::FixedTimestep};
use indexmap::IndexMap;
use scratch_edu_parser::project::{
    sprite::{
        op_codes::{self, Motion},
        Block, OpCode,
    },
    Target,
};

const REFRESH_TIMESTEP: f64 = 1.0;
/// 30.0;

#[derive(Debug, Component)]
pub struct ScratchInterpreter {
    current_blocks: Vec<String>,
}

impl ScratchInterpreter {
    pub fn new(blocks: &IndexMap<String, Block>) -> Self {
        Self {
            current_blocks: blocks
                .iter()
                .filter_map(|(uuid, block)| {
                    if block.opcode == OpCode::Event(op_codes::Event::WhenFlagClicked) {
                        Some(uuid.to_string())
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }

    pub fn interpret_once(&mut self, sprite: &mut Target) {
        let mut completed = Vec::new();
        for block_uuid in self.current_blocks.iter_mut() {
            let block = sprite.blocks.get(block_uuid).unwrap();
            println!("{:?}", block);
            if let Some(next_uuid) = &block.next {
                *block_uuid = next_uuid.clone();
            } else {
                completed.push(block_uuid.clone());
            }
            match block.opcode {
                OpCode::Event(_) => (), // starting events have no side effects
                OpCode::Motion(Motion::GoToXY) => {
                    let x = block
                        .inputs
                        .get("X")
                        .and_then(|i| i.data.first())
                        .and_then(|s| s.as_shadow())
                        .and_then(|s| s.as_number())
                        .unwrap();
                    let y = block
                        .inputs
                        .get("Y")
                        .and_then(|i| i.data.first())
                        .and_then(|s| s.as_shadow())
                        .and_then(|s| s.as_number())
                        .unwrap();
                    sprite.x = Some(*x);
                    sprite.y = Some(*y);
                }
                _ => unimplemented!(),
            }
        }
        self.current_blocks.retain(|uuid| !completed.contains(uuid));
    }

    pub fn is_complete(&self) -> bool {
        self.current_blocks.is_empty()
    }
}

fn interpreter_system(
    mut commands: Commands,
    mut interpreters: Query<(&mut ScratchInterpreter, &mut ScratchTarget, Entity)>,
) {
    for (mut interpreter, mut target, entity) in &mut interpreters {
        // cleanup finished interpreters
        if interpreter.is_complete() {
            commands.entity(entity).remove::<ScratchInterpreter>();
        }
        interpreter.interpret_once(&mut target.0);
    }
}

pub struct InterpreterPlugin;

impl Plugin for InterpreterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(REFRESH_TIMESTEP))
                .with_system(interpreter_system),
        );
    }
}
