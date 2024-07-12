use crate::arena::Arena;
use crate::arena::ArenaRef;
use crate::parse::AstNode;

pub struct ProgramState {}

impl Default for ProgramState {
    fn default() -> Self {
        Self {}
    }
}

pub fn run(state: &mut ProgramState, root: ArenaRef, node_pool: &mut Arena<AstNode>) {
    todo!()
}
