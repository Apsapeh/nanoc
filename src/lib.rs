mod types;
mod preprocessor;

fn include_req_impl(f: String) -> String {
    std::fs::read_to_string(f).unwrap()
}

pub fn compile(files: Vec<String>) {
    for f in files {
        let preprocessed = preprocessor::process(
            &std::fs::read_to_string(&f).unwrap(),
            &f,
            include_req_impl);
    }
}