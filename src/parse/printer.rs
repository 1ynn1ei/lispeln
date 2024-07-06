use crate::arena::Arena;
use crate::arena::ArenaRef;
use crate::parse::AstNode;

fn ident_string(old_string: &str, ident: usize) -> String {
    let mut string = String::new();
    for i in 0..ident {
        string.push(' ');
        string.push(' ');
    }
    format!("{string}{old_string}")
}

pub fn print_node(node_pool: &Arena<AstNode>, cur_ref: &ArenaRef, ident: usize) -> String {
    let node = node_pool.get(*cur_ref).unwrap();
    match node {
        AstNode::Program { body } => {
            return format!(
                "{}: \n{}",
                ident_string("Program", ident),
                body.iter().fold("".to_string(), |acc, next| {
                    acc + &print_node(node_pool, next, ident + 1) + "\n"
                })
            );
        }
        AstNode::Number { value } => {
            return format!(
                "{}: {}",
                ident_string("Number", ident),
                value.to_string().as_str()
            );
        }
        _ => todo!(),
    }
    todo!("Write the code to represent any node as a string");
}
