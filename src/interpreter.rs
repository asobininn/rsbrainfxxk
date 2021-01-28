use crate::{Node, Operation, Result, parser};
use std::{u8, char, io::{self, Read}};

const AREA_SIZE: usize = 256;
type ProgramArea = [u8; AREA_SIZE];
type Output = Result<()>;

pub struct Interpreter {
    area: ProgramArea,
    pointer: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            area: [0; AREA_SIZE],
            pointer: 0
        }
    }

    pub fn from_ast(&mut self, ast: Vec<Node>) -> Output {
        let eval = Eval::new();
        for node in ast {
            eval.eval(&node, &mut self.area, &mut self.pointer);
        }
        Ok(())
    }

    pub fn from_source(&mut self, source: &str) -> Output {
        let ast: Vec<Node> = parser::parse(source).unwrap();
        self.from_ast(ast)?;
        Ok(())
    }

}

struct Eval;

impl Eval {
    pub fn new() -> Self { Self }

    pub fn eval(&self, node: &Node, area: &mut [u8], pointer: &mut usize) {
        match node {
            Node::Operation { op } => {
                match op {
                    Operation::PInc => {
                        *pointer += 1;
                        if *pointer == AREA_SIZE { *pointer = 0 }
                    },
                    Operation::PDec => {
                        if *pointer == 0 { *pointer = AREA_SIZE }
                        *pointer -= 1;
                    },
                    Operation::VInc => {
                        if area[*pointer] == u8::MAX { area[*pointer] = 0 }
                        else { area[*pointer] += 1 }
                    },
                    Operation::VDec => {
                        if area[*pointer] == 0 { area[*pointer] = u8::MAX }
                        else { area[*pointer] -= 1 }
                    },
                    Operation::Output => print!("{}", area[*pointer] as char),
                    Operation::Input => {
                        let input = io::stdin()
                            .bytes()
                            .next()
                            .and_then(|r| r.ok())
                            .map(|b| b as u8)
                            .unwrap();
                        area[*pointer] = input;
                    },
                }
            },
            Node::Repetition { child } => {
                while area[*pointer] != 0 {
                    for node in child.iter() {
                        self.eval(node, area, pointer);
                    }
                }
            },
        }
    }
}
