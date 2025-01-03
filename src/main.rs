mod abstract_domain;
mod abstract_state;
mod ast;
pub mod lexer;
mod parser;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::Mutex;

pub static M: Mutex<i64> = Mutex::new(0);
pub static N: Mutex<i64> = Mutex::new(0);

pub fn take_int() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}
fn main() {
    //test file path
    let program_file_path = Path::new("/home/alberto/Desktop/soft2ver/soft2ver/src/test/ifTest");
    let state_file_path = Path::new("/home/alberto/Desktop/soft2ver/soft2ver/src/test/ifState");

    //read from the file
    let contents = fs::read_to_string(program_file_path)
        .expect("Should have been able to read the program code");
    let initial_state =
        fs::read_to_string(state_file_path).expect("Should have been able to read the state");

    println!("INSERT m value");
    let m = take_int();
    println!("INSERT n value");
    let n = take_int();

    // Aggiorna i valori di M e N
    {
        let mut global_m = M.lock().unwrap();
        *global_m = m;
    }
    {
        let mut global_n = N.lock().unwrap();
        *global_n = n;
    }

    //lex parse and evaluate the program
    parser::analyze(contents, initial_state);
}
