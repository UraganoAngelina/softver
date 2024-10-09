mod ast;
mod parser;
pub mod lexer;

use std::env;
use std::fs;
use std::path::Path;
use ast::boolean::GreatEqual;
use ast::statement::Statement;
use ast::arithmetic::Numeral;
use ast::arithmetic::Add;
use ast::statement::Assign;
use ast::statement::While;

fn main() {
    let mut state = ast::State::new();
    
    let stmt = Assign {
        var_name: "x".to_string(),
        expr: Box::new(Add {
            left: Box::new(Numeral(5)),
            right: Box::new(Numeral(3)),
        }),
    };

    //test file path
    let program_file_path = Path::new("/home/alberto/Desktop/softver/src/test/pio");
    let state_file_path = Path::new("/home/alberto/Desktop/softver/src/test/factorialState");
    
    //read from the file
    let contents = fs::read_to_string(program_file_path)
        .expect("Should have been able to read the program code");
    let initial_state = fs::read_to_string(state_file_path).expect("Should have been able to read the state");

    //parse the program
    parser::parse(contents, initial_state);
    
    //evaluate the program

    


    
}