mod ast;
mod parser;
pub mod lexer;
mod abstract_domain;
mod abstract_state;
use std::fs;
use std::path::Path;


fn main() {    
    //test file path
    let program_file_path = Path::new("/home/alberto/Desktop/soft2ver/soft2ver/src/test/whileTest");
    let state_file_path = Path::new("/home/alberto/Desktop/soft2ver/soft2ver/src/test/whileState");
    
    //read from the file
    let contents = fs::read_to_string(program_file_path)
        .expect("Should have been able to read the program code");
    let initial_state = fs::read_to_string(state_file_path).expect("Should have been able to read the state");

    //lex parse and evaluate the program
    parser::analyze(contents, initial_state);
 
}