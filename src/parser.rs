use crate::ast::*;
use pest::{Parser, error};
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "grammar.pest"]
struct RsBrainFxxkParser;

pub fn parse(source: &str) -> Result<Vec<Node>, error::Error<Rule>> {
    let mut ast: Vec<Node> = Vec::new();
    let pairs = RsBrainFxxkParser::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}

fn build_ast_from_expr(pair: Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::Operation => {
            parse_operation(pair)
        },
        Rule::Repetition => {
            let mut child = vec![];
            let pairs = pair.into_inner();
            for pair in pairs {
                if let Rule::Expr = pair.as_rule() {
                    child.push(build_ast_from_expr(pair));
                }
            }
            parse_repetition(child)
        },
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn parse_operation(pair: Pair<Rule>) -> Node {
    Node::Operation {
        op: match pair.as_str() {
            ">" => Operation::PInc,
            "<" => Operation::PDec,
            "+" => Operation::VInc,
            "-" => Operation::VDec,
            "." => Operation::Output,
            "," => Operation::Input,
            unknown => panic!("Unknwon operation: {:?}", unknown),
        },
    }
}

fn parse_repetition(child: Vec<Node>) -> Node {
    Node::Repetition {
        child: Box::new(child),
    }
}
