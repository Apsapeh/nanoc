use crate::types::NumWord;

struct Define<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
    pub value: Vec<NumWord<'a>>,
}

impl <'a> Define<'a>{
    pub fn new(name: &'a str, args: Vec<&'a str>, value: Vec<NumWord<'a>>) -> Define<'a> {
        Define {
            name,
            args,
            value,
        }
    }
}

pub fn process<'a>(input: &'a str, filename: &'a str, include_req: fn(String) -> String) -> Vec<NumWord<'a>> {
    let mut result = vec![];
    //let mut defines = vec![];
    for line in &strip_file(input, filename) {
        //println!("{:?}", line);
        // Defines (with recursion)
        if line.len() >= 1 && line[0].word == "#define" {
            if line.len() < 2 {
                panic!("Invalid define: {:?}", line);
            }

            let name = line[1].word;
            println!("Found define: {}", name);

            if line.len() >= 3 && line[2].word == "(" {
                let mut args = vec![];
                let mut i = 3;
                while line[i].word != ")" {
                    if line[i].word == "," {
                        i += 1;
                        continue;
                    }
                    args.push(line[i].word);
                    i += 1;
                }
                println!("Args: {:?}", args);
            }
        }
    }

        /*if line.starts_with("#define") {
            if line.ends_with("\\") {
                // Multiline macro
                let mut def_macro = line.trim_end_matches("\\").to_string();
                let mut j = i + 1;
                loop {
                    let next_line = input.lines().nth(j).unwrap();
                    if next_line.ends_with("\\") {
                        def_macro.push_str(next_line.trim_end_matches("\\"));
                        j += 1;
                    } else {
                        def_macro.push_str(next_line);
                        break;
                    }
                }

                skip_to = j+1;
                println!("Found macro: {}", def_macro);
            } else {
                // Single line macro
                println!("Found macro: {}", line);
            }
        }*/
    result
}



fn strip_file<'a>(input: &'a str, filename: &'a str) -> Vec<Vec<NumWord<'a>>> {
    let mut stripped_result: Vec<Vec<NumWord>> = Vec::new();
    //let mut processed_input = vec![];
    for (line_n, line) in input.lines().enumerate() {
        if line.trim().is_empty() || line.trim().starts_with("//") {
            continue;
        }

        let mut result = Vec::new();
        let mut last = 0;
        let mut in_string = false;
        let mut define_word_idx: usize = 0;
        let mut muiltiline_comment_depth: usize = 0;
        let mut skip_n: usize = 0;
        for (index, matched) in line.match_indices(|c: char| {
            [
                ' ', '\t', '(', ')', '{', '}', ';', '.', ',', '>', '<', '-', '+', '*', '/', '%',
                '&', '|', '!', '\'', '"',
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
            } else if matched == "<" {
                in_string = true;
            } else if matched == ">" {
                in_string = false;
            }

            let next_char = match line.get(index + 1..index + 2) {
                Some(s) => s,
                None => "",
            };

            if matched == "/" && next_char == "/" && !in_string {
                muiltiline_comment_depth = 1;
                break;
            }
            if matched == "/" && next_char == "*" && !in_string {
                if last != index {
                    result.push(NumWord::new(&line[last..index], &filename, line_n+1, last+1))
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
                if last != index {
                    if &line[last..index] == "#define" {
                        define_word_idx += 1;
                    }
                    result.push(NumWord::new(&line[last..index], &filename, line_n+1, last+1))

                }
                if in_string || (matched != " " && matched != "\t") || define_word_idx == 2 {
                    result.push(NumWord::new(matched, &filename, line_n+1, index+1))
                }
                last = index + matched.len();
            }

            if define_word_idx != 0 {
                define_word_idx += 1
            }
        }
        if last < line.len() && muiltiline_comment_depth == 0 {
            result.push(NumWord::new(&line[last..], &filename, line_n+1, last+1))
        }
        stripped_result.push(result);
        //println!("{:?}", result);
    }
    stripped_result
}