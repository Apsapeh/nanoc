//#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Debug, PartialEq)]
pub struct NumWord {
    pub word: String,
    pub filename: String,
    pub line: usize,
    pub col: usize,
}

impl  NumWord  {
    pub fn new(word: String, filename: String, line: usize, col: usize) -> NumWord {
        NumWord {
            word,
            filename,
            line, 
            col,
        }
    }
    
}