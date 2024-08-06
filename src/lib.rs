

mod types;
mod general;
mod preprocessor;

use preprocessor::{Define, IncludeReqType};
use types::NumWord;

fn include_req_impl(f: String, t: IncludeReqType) -> Option<String> {
    match t {
        IncludeReqType::Relative => {
            match std::fs::read_to_string(&f) {
                Ok(data) => Some(data),
                Err(_) => None
            }
        }
        IncludeReqType::DirList => {
            None
        }
    }
}

pub fn compile(files: Vec<String>) {
    for f in files {
        let file_data = std::fs::read_to_string(&f).unwrap();
        let preprocessed = preprocessor::process(
            file_data,
            &f,
            include_req_impl, 
            Some(&vec![Define::new("GLOBAL".to_string(), vec![], vec![NumWord::new("\"Hello\"".to_string(), "".to_string(), 0, 0)])]),
        );

        let preprocessed = match preprocessed {
            Ok(words) => words,
            Err(errs) => {
                for e in errs {
                    println!("Error: {}", e);
                }
                //panic!("");
                panic!();
            }
        };

        //println!("{:?}", preprocessed.defines);
    }
}