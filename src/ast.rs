use super::enums::{Op, Token};
use std::process::Command;
use super::lexer;

fn find_last_occ(op: &Op, tokens: &Vec<Token>) -> Option<usize> {
    for n in (0..tokens.len()).rev() {
        match &tokens[n] {
            Token::Operator(o) => {
                if o == op {
                    return Some(n);
                }
            }
            Token::CommandOrArgument(_) => continue,
        }
    }
    return None;
}

#[derive(Debug)]
pub enum Ast {
    Node(Box<Option<Ast>>, Box<Option<Ast>>, Op),
    Leaf(Command),
}

pub fn make_ast(tokens: &Vec<Token>) -> Result<Box<Option<Ast>>, String> {
    let operators = [
        Op::Semicolon,
        Op::Background,
        Op::And,
        Op::Or,
        Op::Pipe,
        Op::RedirectLeft,
        Op::RedirectRight,
    ];

    for op in operators.iter() {
        let idx = find_last_occ(&op, tokens);
        match idx {
            Some(n) => {
                println!("{}", n);
                if n == tokens.len() - 1 {
                    let left_tree = make_ast(&(tokens[..n]).to_vec());
                    match left_tree {
                        Ok(tree) => return Ok(Box::new(Some(Ast::Node(tree, Box::new(None), *op)))),
                        x => return x,
                    }
                } else if n == 0 {
                    return Err(format!(
                        "Unexpected Token found {:?} at the start of an expression",
                        *op
                    ));
                } else {
                    let left_tree = make_ast(&(tokens[..n]).to_vec());
                    let right_tree = make_ast(&(tokens[n + 1..]).to_vec());
                    match (left_tree, right_tree) {
                        (Ok(l_tree), Ok(r_tree)) => return Ok(Box::new(Some(Ast::Node(l_tree, r_tree, *op)))),
                        (Err(x), _) => return Err(x),
                        (_, Err(x)) => return Err(x),
                    }
                }
            }
            None => continue,
        }
    }

    if tokens.len() == 1 {
        match &tokens[0] {
            Token::CommandOrArgument(x) => {
                if x.starts_with('(') && x.ends_with(')') {
                    let mut new_str = &(x.as_str()[1..x.len()-1]).to_string();
                    let mut new_tokens: Vec<Token> = Vec::new();
                    lexer::tokenize(&mut new_str, &mut new_tokens);
                    return make_ast(&new_tokens);
                }
            },
            _ => (),
        }
    }

    //there are no operators left
    //TODO: Parenths
    let mut iter = tokens.iter();
    let mut comm; //= Command::new("echo");
    match iter.next() {
        Some(Token::CommandOrArgument(command)) => {
            comm = Command::new(command);
            for tok in iter {
                match tok {
                    Token::Operator(_) => println!("Parsing error"),
                    //recursively parse if it has parenths eg: (echo hello; cat new && ok)
                    Token::CommandOrArgument(x) => {
                        comm.arg(x);
                    }
                }
            }
            return Ok(Box::new(Some(Ast::Leaf(comm))));
        }
        _ => return Ok(Box::new(None)),
    }
}

// pub struct Node
// {
//     left  : Ast,
//     right : Ast,
//     operator : Operator,
// }