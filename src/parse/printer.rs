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
    todo!("Write the code to represent any node as a string");
}
