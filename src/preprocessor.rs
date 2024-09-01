use std::cell::RefCell;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::{Rc, Weak};

use crate::general::CStandard;
use crate::types::NumWord;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct Define {
    pub name: String,
    pub args: Vec<String>,
    pub value: Vec<NumWord>,
}

impl Define {
    pub fn new(name: String, args: Vec<String>, value: Vec<NumWord>) -> Define {
        Define { name, args, value }
    }
}

#[derive(PartialEq, Clone)]
pub enum IncludeReqType {
    Relative,
    DirList,
}

struct PreprocessorOut {
    pub result: Vec<NumWord>,
    pub defines: Vec<Define>,
    pub include_once_files: Vec<IncludeOnceFile>,
    pub errors: Vec<String>,
}

#[derive(PartialEq, Clone)]
struct IncludeOnceFile {
    pub hash: u64,
    pub filename: String,
    pub mode: Option<IncludeReqType>,
}

#[derive(Debug)]
struct ConditionalTree {
    pub children: Vec<Rc<RefCell<ConditionalTree>>>,
    pub body: Vec<NumWord>,
    //pub parent: Option<&'a RefCell<ConditionalTree <'a>>>,
    pub parent: Option<Weak<RefCell<ConditionalTree>>>,
}

impl ConditionalTree {
    pub fn print(&self) {
        println!("count: {}", self.children.len());
        for ch in &self.children {
            ch.borrow().print();
        }
    }
}

pub fn process<'a>(
    input: String,
    filename: &'a str,
    include_req: fn(String, IncludeReqType) -> Option<String>,
    global_defines: Option<&Vec<Define>>,
) -> Result<Vec<NumWord>, Vec<String>> {
    let global_defines = match global_defines {
        Some(d) => d.clone(),
        None => vec![],
    };

    let buff_include_once_files = vec![];

    let i = input.clone();
    let include_req = |f: String, t: Option<IncludeReqType>| match t {
        Some(t) => include_req(f, t),
        None => Some(i.clone()),
    };

    let p = preprocessor_body(
        input,
        None,
        filename,
        &include_req,
        global_defines,
        buff_include_once_files,
        &vec![],
    );

    if p.errors.is_empty() {
        Ok(p.result)
    } else {
        Err(p.errors)
    }
}

fn preprocessor_body<'a>(
    input: String,
    input_file_mode: Option<IncludeReqType>,
    filename: &'a str,
    include_req: &impl Fn(String, Option<IncludeReqType>) -> Option<String>,
    mut defines: Vec<Define>,
    mut include_once_files: Vec<IncludeOnceFile>,
    recursion_include_guadrd_stack: &Vec<IncludeOnceFile>,
) -> PreprocessorOut {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();
    let mut errors = vec![];
    let mut result: Vec<NumWord> = vec![];
    let recursion_include_guadrd_stack = [
        recursion_include_guadrd_stack.clone(),
        vec![IncludeOnceFile {
            hash,
            filename: String::from(filename),
            mode: input_file_mode.clone(),
        }],
    ]
    .concat();

    let a = include_once_files.iter().find(|x| x.hash == hash);
    if a.is_some() {
        let po_d = include_req(a.unwrap().filename.clone(), a.unwrap().mode.clone());

        if po_d.is_some() {
            if po_d.unwrap() == input {
                return PreprocessorOut {
                    result: vec![],
                    defines,
                    include_once_files,
                    errors: vec![],
                };
            }
        }
    }

    //let mut defines: Vec<Define> = global_defines.clone();
    //let mut include_once_files: Vec<IncludeOnceFile> = buff_include_once_files.clone();
    let stripped = strip_file(input, String::from(filename));
    let mut skip_stack = vec![];
    let root_conditional_tree = Rc::new(RefCell::new(ConditionalTree { children: vec![], body: vec![], parent: None }));
    let mut current_conditional_tree = Rc::downgrade(&root_conditional_tree);
    for line in stripped {
        //println!("{:?}", line);
        // Defines (with recursion)

        if line[0].word == "#" {

            if line[1].word == "ifdef" || line[1].word == "ifndef" {
                if line.len() < 3 {
                    errors.push(format!("Invalid {}: {:?}", line[1].word, line));
                    continue;
                }
                if let None = defines.iter().find(|x| x.name == line[2].word) {
                    skip_stack.push(line[1].word == "ifdef");
                } else {
                    skip_stack.push(line[1].word == "ifndef");
                }
                let new_node = Rc::new(RefCell::new(ConditionalTree {
                    children: vec![],
                    body: vec![],
                    parent: Some(Weak::clone(&current_conditional_tree)),
                }));
                println!("CC_1: {:?}", current_conditional_tree.upgrade().unwrap().borrow());

                let new_c = Rc::downgrade(&new_node);
                current_conditional_tree.upgrade().unwrap().borrow_mut().children.push(new_node);
                current_conditional_tree = new_c;
                //current_conditional_tree.
                //current_conditional_tree = Rc::clone(&rc);
            } else if line[1].word == "if" {
            } else if line[1].word == "elif" {
            } else if line[1].word == "else" {
            } else if line[1].word == "endif" {
                let a = match &current_conditional_tree.upgrade().unwrap().borrow().parent {
                    Some(s) => Weak::clone(s),
                    None => {

                        root_conditional_tree.borrow().print();
                        //&root_conditional_tree
                        Rc::downgrade(&root_conditional_tree)
                        //Rc::clone(&root_conditional_tree)
                    }
                };

                println!("CC: {:?}", current_conditional_tree.upgrade().unwrap().borrow());
                println!("A: {:?}", a.upgrade().unwrap().borrow());

                

                current_conditional_tree = a;

                if (current_conditional_tree.upgrade().unwrap().borrow().parent.is_none()) {
                    root_conditional_tree.borrow().print();
                }

                //current_conditional_tree = Rc::clone(a);
                //current_conditional_tree = Rc::clone(&root_conditional_tree);
                
                if skip_stack.len() == 0 {
                    errors.push(format!("Invalid endif: {:?}", line));
                    continue;
                }
                skip_stack.pop();
                continue;
            }
        }

        if skip_stack.contains(&true) {
            continue;
        }

        

        if line[0].word == "#" {

            /*if line[1].word == "ifdef" {
                if line.len() < 3 {
                    errors.push(format!("Invalid ifdef: {:?}", line));
                    continue;
                }
                if let None = defines.iter().find(|x| x.name == line[2].word) {
                    skip_stack.push(true);
                } else {
                    skip_stack.push(false);
                }
            } else if line[1].word == "ifndef" {
            } else if line[1].word == "if" {
            } else if line[1].word == "elif" {
            } else if line[1].word == "else" {
            } else if line[1].word == "endif" {
                if skip_stack.len() == 0 {
                    errors.push(format!("Invalid endif: {:?}", line));
                    continue;
                }
                skip_stack.pop();
            }

            else*/ if line[1].word == "define" {
                if line.len() < 3 {
                    //panic!("Invalid define: {:?}", line);
                    errors.push(format!("Invalid define: {:?}", line));
                    continue;
                }

                let name = line[2].clone().word;
                if !is_avaliable_name(name.as_str(), &vec![], CStandard::C89) {
                    //panic!("Invalid define name, used unallowed symbols: {:?}", line);
                    errors.push(format!(
                        "Invalid define name, used unallowed symbols: {:?}",
                        line
                    ));
                    continue;
                }
                //println!("Found define: {}", name);

                let mut args = vec![];
                let mut i = 3;
                if line.len() >= 4 && line[3].word == "(" {
                    i = 4;
                    while line[i].word != ")" {
                        //if line[i].word.is

                        if line[i].word == "," {
                            i += 1;
                            continue;
                        }
                        args.push(line[i].clone().word);
                        i += 1;
                    }
                    i += 1;
                    //println!("Args: {:?}", args);
                }

                let mut value = vec![];
                let mut is_string = false;
                while i < line.len() {
                    let w = line[i].clone().word;
                    if line[i].word == "\"" {
                        is_string = !is_string;
                    }

                    if is_string || (w != " " && w != "\t") {
                        value.push(line[i].clone());
                    }

                    i += 1;
                }

                if let Some(define) = defines.iter_mut().find(|x| x.name == name) {
                    define.args = args;
                    define.value = value;
                } else {
                    defines.push(Define::new(name, args, value));
                }

                //println!("Define: {:?}", defines.last().unwrap());
            } else if line[1].word == "include" {
                //println!("Found include: {:?}", line);

                if line.len() < 5 {
                    errors.push(format!("Invalid include: {:?}", line));
                    continue;
                }

                let (find_char, rec_type) = match line[2].word.as_str() {
                    "<" => (String::from(">"), IncludeReqType::DirList),
                    "\"" => (String::from("\""), IncludeReqType::Relative),
                    _ => {
                        errors.push(format!("Invalid include: {:?}", line));
                        continue;
                    }
                };

                let mut include_path = String::new();
                for w in &line[3..] {
                    if w.word == find_char {
                        break;
                    }
                    include_path.push_str(&w.word);
                }

                let path = std::path::Path::new(&filename)
                    .parent()
                    .unwrap()
                    .join(include_path)
                    .to_str()
                    .unwrap()
                    .to_string();

                //println!("Include path: {:?}", include_path);
                println!("Filename: {:?}", path);

                //input.push_str("string");

                let include_data = match include_req(path.clone(), Some(rec_type.clone())) {
                    Some(data) => data,
                    None => {
                        errors.push(format!("Include error: {:?}", line));
                        continue;
                    }
                };

                let mut hasherd = DefaultHasher::new();
                include_data.hash(&mut hasherd);
                let hash = hasherd.finish();

                let a = recursion_include_guadrd_stack
                    .iter()
                    .find(|x| x.hash == hash);
                if a.is_some() {
                    let po_d = include_req(a.unwrap().filename.clone(), a.unwrap().mode.clone());

                    if po_d.is_some() {
                        if po_d.unwrap() == include_data {
                            errors.push(format!("Recursive include: {:?}", line));
                            continue;
                        }
                    }
                }

                let mut p = preprocessor_body(
                    include_data,
                    Some(rec_type),
                    path.as_str(),
                    include_req,
                    defines,
                    include_once_files,
                    &recursion_include_guadrd_stack,
                );

                //result.append(&mut words.result);
                result.append(&mut p.result);
                defines = p.defines;
                include_once_files = p.include_once_files;
                //defines.append(&mut words.defines);

                errors.append(&mut p.errors);

                // TODO: Add body
                //let include = include_req(line[1].word.to_string());
                //result.extend(process(include.as_str(), filename, include_req).into_iter());
            } else if line[1].word == "undef" {
                if line.len() < 3 {
                    errors.push(format!("Invalid undef: {:?}", line));
                    continue;
                }

                defines.retain(|x| x.name != line[2].word);
            }  else if line[1].word == "error" {
                /*let mut msg = String::new();
                for w in &line[2..] {
                    msg.push_str(&w.word);
                }
                errors.push(format!("Error: {}", msg));*/
                errors.push(format!("Error: {:?}", line));
            } else if line[1].word == "warning" {
            } else if line[1].word == "pragma" {
                if line[2].word == "once" {
                    include_once_files.push(IncludeOnceFile {
                        hash,
                        filename: filename.to_string(),
                        mode: input_file_mode.clone(),
                    })
                }
            }
        } else {
            let mut skip_n: u64 = 0;
            for (idx, word) in line.iter().enumerate() {
                if skip_n > 0 {
                    skip_n -= 1;
                    continue;
                }

                if let Some(_) = defines.iter().find(|x| x.name == word.word) {
                    //println!("Found define: {}", word.word);

                    let mut _line = line[idx..].to_vec(); // Текущая строка
                    let mut define_result_string = vec![word.clone()]; // Строка от прошлого парсина
                                                                       //let mut s;
                    let mut tmp_define_result_string = vec![];

                    let mut is_first_iter = true;
                    loop {
                        let mut has_macro = false;
                        let mut skip_n_define: u64 = 0;
                        for (word_it_idx, word_it) in define_result_string.iter().enumerate() {
                            if skip_n_define > 0 {
                                skip_n_define -= 1;
                                continue;
                            }

                            if let Some(define) = defines.iter().find(|x| x.name == word_it.word) {
                                has_macro = true;

                                if define.args.is_empty() {
                                    tmp_define_result_string
                                        .extend(define.value.clone().into_iter());
                                } else {
                                    let mut brackets_counter: i32 = 0;
                                    let mut args = vec![];
                                    let mut tmp_arg = vec![];
                                    for _word in _line.iter().skip(word_it_idx + 1) {
                                        if is_first_iter {
                                            skip_n += 1;
                                        }

                                        skip_n_define += 1;

                                        if _word.word == "(" {
                                            brackets_counter += 1;

                                            if brackets_counter == 1 {
                                                continue;
                                            }
                                        } else if _word.word == ")" {
                                            brackets_counter -= 1;

                                            if brackets_counter <= 0 {
                                                break;
                                            }
                                        }

                                        if _word.word == "," && brackets_counter == 1 {
                                            args.push(tmp_arg.clone());
                                            tmp_arg.clear();
                                        } else {
                                            tmp_arg.push(_word);
                                        }
                                    }
                                    args.push(tmp_arg.clone());

                                    if args.len() != define.args.len() {
                                        errors.push(format!(
                                            "Invalid define args count, expected: {}, got: {}, args: {:?}",
                                            define.args.len(),
                                            args.len(),
                                            args
                                        ));
                                        continue;
                                    }

                                    //println!("Args: {:?}", args);

                                    let mut is_next_concat = false;
                                    for value_word in define.value.iter() {
                                        //println!("Value word: {:?}", value_word);
                                        if value_word.word == "##" {
                                            is_next_concat = true;
                                            continue;
                                        }

                                        if let Some(arg) = define
                                            .args
                                            .iter()
                                            .enumerate()
                                            .find(|x| x.1 == &value_word.word)
                                        {
                                            if is_next_concat {
                                                if tmp_define_result_string.len() > 0 {
                                                    tmp_define_result_string
                                                        .last_mut()
                                                        .unwrap()
                                                        .word
                                                        .push_str(&args[arg.0].last().unwrap().word)
                                                }
                                            } else {
                                                tmp_define_result_string.extend(
                                                    args[arg.0]
                                                        .iter()
                                                        .map(|&x| x.clone())
                                                        .collect::<Vec<NumWord>>(),
                                                );
                                            }
                                        } else {
                                            if is_next_concat {
                                                if tmp_define_result_string.len() > 0 {
                                                    tmp_define_result_string
                                                        .last_mut()
                                                        .unwrap()
                                                        .word
                                                        .push_str(&value_word.word)
                                                }
                                            } else {
                                                tmp_define_result_string.push(value_word.clone());
                                            }
                                        }

                                        is_next_concat = false;
                                    }
                                }
                            } else {
                                tmp_define_result_string.push(word_it.clone());
                            }
                        }

                        define_result_string = tmp_define_result_string.clone();
                        _line = define_result_string.clone();
                        tmp_define_result_string.clear();

                        is_first_iter = false;

                        if !has_macro {
                            break;
                        }
                    }

                    //if !define_result_string.is_empty() {
                    result.extend(define_result_string.into_iter());
                    //}
                } else {
                    result.push(word.clone());
                }
            }
        }
    }

    let mut r_str = String::new();
    for word in &result {
        r_str.push_str(&word.word);
        r_str.push(' ');
    }
    println!("{}", r_str);

    // let mut iter = global_defines.iter();
    // while let Some(d) = iter.next() {
    //     defines.retain(|x| d != x);
    // }

    // let mut iter = buff_include_once_files.iter();
    // while let Some(f) = iter.next() {
    //     include_once_files.retain(|x| f != x);
    // }

    PreprocessorOut {
        result,
        defines,
        include_once_files,
        errors,
    }
}

fn strip_file<'a>(input: String, filename: String) -> Vec<Vec<NumWord>> {
    let mut stripped_result: Vec<Vec<NumWord>> = Vec::new();
    //let mut processed_input = vec![];
    let mut muiltiline_comment_depth: usize = 0;
    let mut concate_next_line = 0;
    for (line_n, line) in input.lines().enumerate() {
        if line.trim().is_empty() || line.trim().starts_with("//") {
            continue;
        }

        let mut result = Vec::new();
        let mut last = 0;
        let mut index_diff = 0;
        let mut in_string = false;
        let mut is_single_line_comment = false;
        let mut define_word_idx: usize = 0;
        let mut is_include = false;
        let mut skip_n: usize = 0;
        let concate_this_line = concate_next_line > 0;
        for (index, mut matched) in line.match_indices(|c| {
            [
                ' ', '\t', '(', ')', '{', '}', ';', '.', ',', '>', '<', '-', '+', '*', '/', '%',
                '&', '|', '!', '\'', '"', '\\', '#',
            ]
            .contains(&c)
        }) {
            if skip_n != 0 {
                skip_n -= 1;
                last = index + 1;
                continue;
            }

            if matched == "\"" {
                in_string = !in_string;
            } /*else if matched == "<" {
                  in_string = true;
              } else if matched == ">" {
                  in_string = false;
              }*/

            if is_include && matched == "<" {
                in_string = true;
            } else if is_include && matched == ">" {
                in_string = false;
            }

            let next_char = match line.get(index + 1..index + 2) {
                Some(s) => s,
                None => "",
            };

            let next_next_char = match line.get(index + 2..index + 3) {
                Some(s) => s,
                None => "",
            };

            if matched == "/" && next_char == "/" && !in_string {
                //muiltiline_comment_depth = 0;
                is_single_line_comment = true;
                break;
            }
            if matched == "/" && next_char == "*" && !in_string {
                if last != index {
                    result.push(NumWord::new(
                        line[last..index].to_string(),
                        filename.clone(),
                        line_n + 1,
                        last + 1,
                    ))
                }
                muiltiline_comment_depth += 1;

                last = index + 1;
                continue;
            }
            if matched == "*" && next_char == "/" && !in_string {
                muiltiline_comment_depth -= 1;
                last = index + 1;
                skip_n = 1;
                continue;
            }

            if muiltiline_comment_depth == 0 {
                if let Some(c) = [
                    "+=", "-=", "++", "--", ">=", "<=", "==", "!=", "*=", "/=", "%=", "|=", "&=",
                    "^=", "->", "&&", "||", ">>", "<<", "##",
                ]
                .into_iter()
                .find(|&x| x == format!("{}{}", matched, next_char))
                {
                    matched = c;
                    skip_n = 1;
                }

                if let Some(c) = [">>=", "<<="]
                    .into_iter()
                    .find(|&x| x == format!("{}{}", matched, next_next_char))
                {
                    matched = c;
                    skip_n = 2;
                }

                if matched == "\\" && next_char == "" {
                    concate_next_line = 2;
                    //last = index;
                    //continue;
                    //panic!("{:?}\n{:?}", next_char, line);
                }

                if last != index {
                    if last > 0 && &line[last - 1..index] == "#define" {
                        define_word_idx += 1;
                        //panic!("{:?}", matched);
                    } else if last > 0 && &line[last - 1..index] == "#include" {
                        is_include = true;
                    }
                    result.push(NumWord::new(
                        line[last..index].to_string(),
                        filename.clone(),
                        line_n + 1,
                        last + 1 - index_diff,
                    ));
                    //println!("{}: {}", line[last..index].to_string(), index_diff);
                    index_diff += line[last..index].len() - line[last..index].chars().count();
                }
                if (in_string || (matched != " " && matched != "\t") || define_word_idx == 2)
                    && concate_next_line < 2
                {
                    result.push(NumWord::new(
                        matched.to_string(),
                        filename.clone(),
                        line_n + 1,
                        index + 1 - index_diff,
                    ));
                    //println!("{}", matched);
                }
                last = index + matched.len();
                //last_char_diff = 0;//matched.len() - matched.chars().count();
            }
            //index_diff += matched.len() - matched.chars().count();

            if define_word_idx != 0 {
                define_word_idx += 1
            }
        }
        if last < line.len()
            && muiltiline_comment_depth == 0
            && !is_single_line_comment
            && concate_next_line <= 1
        {
            result.push(NumWord::new(
                line[last..].to_string(),
                filename.clone(),
                line_n + 1,
                last + 1 - index_diff,
            ))
        }
        if !result.is_empty() {
            if concate_next_line == 1 || concate_this_line {
                if stripped_result.len() > 0 {
                    stripped_result.last_mut().unwrap().extend(result);
                }
            } else {
                stripped_result.push(result);
            }
        }

        if concate_next_line > 0 {
            concate_next_line -= 1;
        }
        //concate_next_line = false;
        //println!("{:?}", result);
    }
    stripped_result
}

fn is_avaliable_name(name: &str, keywords: &Vec<&str>, cstd: CStandard) -> bool {
    if keywords.contains(&name) {
        return false;
    }

    if name.len() == 0 {
        return false;
    }

    let ch = name.chars().next().unwrap();
    match cstd {
        CStandard::C89 => {
            if !ch.is_alphabetic() && ch != '_' && ch != '$' {
                return false;
            }
        }
        CStandard::C99 => {
            if ch.is_ascii_control()
                || ch.is_numeric()
                || ch.is_ascii_punctuation() && ch != '_' && ch != '$'
            {
                return false;
            }
        }
    }

    for c in name.chars() {
        match cstd {
            CStandard::C89 => {
                if !c.is_alphanumeric() && c != '_' && c != '$' {
                    return false;
                }
            }
            CStandard::C99 => {
                if c.is_ascii_control() || c.is_ascii_punctuation() && c != '_' && c != '$' {
                    return false;
                }
            }
        }
    }

    true
}
