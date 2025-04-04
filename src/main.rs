mod abstract_domain;
mod abstract_interval;
mod abstract_state;
mod ast;
pub mod lexer;
mod parser;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::Mutex;
use std::sync::MutexGuard;

#[macro_use]
extern crate lazy_static;


// 9223372036854775807

lazy_static! {
    static ref CONSTANTS_VECTOR: Mutex<Vec<i64>> = Mutex::new(Vec::new());
}
pub static M: Mutex<i64> = Mutex::new(0);
pub static N: Mutex<i64> = Mutex::new(0);
pub static ANALYSIS_FLAG: Mutex<i64> = Mutex::new(1);
pub static WIDENING_FLAG: Mutex<bool> = Mutex::new(false);
pub static NARROWING_FLAG : Mutex<bool> = Mutex::new(false);

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
    let mut _analysis = 0;

    loop {
        println!("Type '1' or '2' to choose between denotational and abstract semantics to perform the analysis ");
        _analysis = take_int();
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
                    let mut vec = CONSTANTS_VECTOR
                        .lock()
                        .expect("failed to lock constant vector");
                    vec.push(_m);
                    vec.push(_n);
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
            let mut _narrow = false;
            if _wid {
                println!("Do you wanna use narrowing? type y or n, otherwise will be yes ");
                _narrow = take_bool();
            }
            {
                let mut global_narrow_flag = NARROWING_FLAG.lock().unwrap();
                *global_narrow_flag = _narrow;
            }
            break;
        } else {
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

pub fn find_max(vec: &mut MutexGuard<'_, Vec<i64>>, value: i64) -> i64 {
    if let Some(max_val) = vec.iter()
                              .filter(|&&x| x <= value)
                              .cloned()
                              .max() {
        //println!("inf found {}", max_val);
        max_val
    } else {
        unreachable!("ERROR IN THE INF SEARCH");
    }
}
pub fn find_min(vec: &mut MutexGuard<'_, Vec<i64>>, value: i64) -> i64 {
    println!("sup search for value {} ", value);
    println!("Vec content: {:?}", *vec);
    
    // Cerca il massimo valore minore o uguale a value
    if let Some(max_val) = vec.iter()
                              .filter(|&&x| x >= value)
                              .cloned()
                              .min() {
        //println!("sup found {}", max_val);
        max_val
    } else {
        unreachable!("ERROR IN THE SUP SEARCH");
    }
}
