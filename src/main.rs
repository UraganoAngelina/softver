mod abstract_interval;
mod abstract_state;
mod abstract_domain;
mod ast;
pub mod lexer;
mod parser;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::Mutex;

// 9223372036854775807

pub static  M: Mutex<i64> = Mutex::new(0);
pub static N: Mutex<i64> = Mutex::new(0);
pub static ANALYSIS_FLAG: Mutex<i64>= Mutex::new(1);
pub static WIDENING_FLAG: Mutex<bool> = Mutex::new(false);

pub fn take_int() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

pub fn take_bool() -> bool {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Errore nella lettura dell'input");

    let trimmed = input.trim().to_lowercase();

    match trimmed.as_str() {
        "y" => return true,
        "n" => return false,
        _ => return true,
    }
}
fn main() {
    //test file path
    let program_file_path = Path::new("/home/alberto/Desktop/soft2ver/soft2ver/src/test/ifTest");

    //read from the file
    let contents = fs::read_to_string(program_file_path)
        .expect("Should have been able to read the program code");
    //read from the std input
    let mut _m = 0;
    let mut _n = 0;
    let mut _analysis =0;


    loop{
        println!("Type '1' or '2' to choose between denotational and abstract semantics to perform the analysis ");
        _analysis=take_int();
        if _analysis == 1 {
            break;
        }
        if _analysis == 2 {
            loop {
                println!("INSERT m value");
                _m = take_int();
                println!("INSERT n value");
                _n = take_int();
        
                if _m <= _n {
                    println!("Valid input: m = {}, n = {}", _m, _n);
                    break; // Exit the loop when condition is met
                } else {
                    println!("Invalid input. Ensure that and m <= n.");
                }
            }
            // Update M and N values
            {
                let mut global_m = M.lock().unwrap();
                *global_m = _m;
            }
            {
                let mut global_n = N.lock().unwrap();
                *global_n = _n;
            }
        
            let mut _wid = false;
            println!("Do you wanna use widening? type y or n, otherwise will be yes ");
            _wid = take_bool();
        
            {
                let mut global_wid_flag = WIDENING_FLAG.lock().unwrap();
                *global_wid_flag = _wid;
            }
            break;
        }
        else {
            println!("invalid input, ensure yor're typing '1' or '2' ");
        }
    }
    
   {
    let mut global_analysis_flag = ANALYSIS_FLAG.lock().unwrap();
    *global_analysis_flag = _analysis;
   }
    //lex parse and evaluate the program
    parser::analyze(contents);
}
