#[derive(Debug)]
pub struct NumWord <'a> {
    pub word: &'a str,
    pub filename: &'a str,
    pub line: usize,
    pub col: usize,
}

impl <'a> NumWord <'a> {
    pub fn new(word: &'a str, filename: &'a str, line: usize, col: usize) -> NumWord<'a> {
        NumWord {
            word,
            filename,
            line, 
            col
        }
    }
    
}