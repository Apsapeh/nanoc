use crate::NumWord;

#[derive(Debug)]
pub enum Token {
    Integer(NumWord),
    Float(Vec<NumWord>),
    String(Vec<NumWord>),
    Char(NumWord),
    Word(NumWord),
    RoundBracketBegin(NumWord),     // (
    RoundBracketEnd(NumWord),       // )
    SquareBracketBegin(NumWord),    // [
    SquareBracketEnd(NumWord),      // ]
    BlockBegin(NumWord),            // {
    BlockEnd(NumWord),              // }
    Dot(NumWord),                   // .
    Comma(NumWord),                 // ,
    SemiColon(NumWord),             // ;
    Asterisk(NumWord),              // *
    Ampersand(NumWord),             // &
    VerticalBar(NumWord),           // |
    Caret(NumWord),                 // ^
    Plus(NumWord),                  // +
    Minus(NumWord),                 // -
    Increment(NumWord),             // ++
    Decrement(NumWord),             // --
    Slash(NumWord),                 // /
    BackSlash(NumWord),             // \
    Equal(NumWord),                 // =
    Mod(NumWord),                   // %
    NotEqual(NumWord),              // !=
    Greater(NumWord),               // >
    GreaterEqual(NumWord),          // >=
    Less(NumWord),                  // <
    LessEqual(NumWord),             // <=
    And(NumWord),                   // &&
    Or(NumWord),                    // ||
    Not(NumWord),                   // !
    RightShift(NumWord),            // >>
    LeftShift(NumWord),             // <<
    PlusSet(NumWord),               // +=
    MinusSet(NumWord),              // -=
    MultiplySet(NumWord),           // *=
    DivideSet(NumWord),             // /=
    ModSet(NumWord),                // %=
    AndSet(NumWord),                // &=
    OrSet(NumWord),                 // |=
    XorSet(NumWord),                // ^=
    RightShiftSet(NumWord),         // >>=
    LeftShiftSet(NumWord),          // <<=
}

pub fn lex(input: Vec<NumWord>) -> Vec<Token> {
    let mut tokens = vec![];
    
    for i in 0..input.len() {
        //let w = &input[i];
        //let prev = if i > 0 { Some(&input[i + 1]) } else { None };

        //println!("Word: '{}' -- {}", w.word, is_integer(&w.word));
        if is_integer(&input[i].word) {
            tokens.push(Token::Integer(input[i]));
        }
        //tokens.push(Token::Word(w));
    }

    println!("Tokens: {:?}", tokens);

    tokens
}

const DEC: [u8; 2] = [b'0', b'9'];
/*const BIN: [u8; 2] = [b'0', b'1'];
const OCT: [u8; 2] = [b'0', b'7'];
const HEX: [u8; 6] = [b'0', b'9', b'a', b'f', b'A', b'F'];


fn get_integer_byte_kit(s: &str) -> Option<(&[u8], usize)> {
    let b = s.as_bytes();
    let start_i;
    let char_kit;
    if b.len() > 1 && b[0] == b'0' {
        if b.len() > 2 {
            if b[1] == b'b' || b[1] == b'b' {
                char_kit = BIN.as_ref();
            } else if b[1] == b'x' || b[1] == b'X' {
                char_kit = HEX.as_ref();
            } else {
                return None;
            }

            start_i = 2;
        } else {
            char_kit = OCT.as_ref();
            start_i = 1;
        }
    } else {
        char_kit = DEC.as_ref();
        start_i = 0;
    }

    Some((char_kit, start_i))
}*/


fn is_integer(s: &str) -> bool {
    if !s.is_ascii() {return false}
    
    let b = s.as_bytes();
    if DEC[0] <= b[0] && b[0] <= DEC[1] {
        return true
    }
    
    /*let (char_kit, start_i) = match get_integer_byte_kit(s) {
        Some(b) => b,
        None => return false
    };

    'outer: for i in start_i..b.len() {
        for j in 0..char_kit.len() / 2 {
            if char_kit[j] <= b[i] && b[i] <= char_kit[j+1] {
                continue 'outer;
            }
        }
        return false
    }*/

    false
}