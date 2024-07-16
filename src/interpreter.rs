use crate::arena::Arena;
use crate::arena::ArenaRef;
use crate::parse::AstNode;
pub enum InterpreterError {
    UnableToInterpretNumber,
}

pub struct ProgramState {}

impl Default for ProgramState {
    fn default() -> Self {
        Self {}
    }
}

pub fn run(state: &mut ProgramState, root: ArenaRef, node_pool: &mut Arena<AstNode>) {
    let cur_node = node_pool.get(root);
    if let Some(node) = cur_node {
        println!("{:?}", node);
        match node {
            AstNode::Program { body } => {
                todo!("implement the code path handling evaluating all children of program");
            }
            AstNode::Number { value } => {
                if let Ok(v) = evaluate_number(value) {
                    println!("{}", v);
                }
            }
            AstNode::Boolean { value } => {}
            AstNode::String { value } => {}
            _ => todo!(),
        }
    }
}

fn evaluate_number(candidate: &str) -> Result<i32, InterpreterError> {
    match candidate.parse::<i32>() {
        Ok(value) => Ok(value),
        Err(_) => Err(InterpreterError::UnableToInterpretNumber),
    }
}
