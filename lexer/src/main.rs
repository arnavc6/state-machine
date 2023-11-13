use std::path::Path;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum Token{
    Identifier(String),
    Word(String),
    Equal,
    Comma,
    LeftSquareBracket,
    RightSquareBracket,
    Global
}

fn lex(code: &str) -> Vec<Token>{
    let mut token_stream: Vec<Token> = Vec::new();
    let chars: Vec<char> = code.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i < chars.len(){
        match chars[i]{
            '=' => {
                token_stream.push(Token::Equal);
            },
            ',' => {
                token_stream.push(Token::Comma);
            },
            '[' => {
                token_stream.push(Token::LeftSquareBracket);
            },
            ']' => {
                token_stream.push(Token::RightSquareBracket);
            },
            '#' => {
                token_stream.push(Token::Global);
            }
            ' ' => {},
            '\t' => {},
            '\n' => {},
            '"' => {
                if !chars[i+1..].contains(&'"'){
                    panic!("invalid string constructor");
                }
                i += 1;
                let mut st = String::from("");
                while chars[i] != '"'{
                    st.push(chars[i]);
                    i += 1;
                } 
                token_stream.push(Token::Word(st));
            },
            _ => {
                let mut id = String::from("");
                while chars[i] != ' ' && chars[i] != '\n' && i < chars.len(){
                    if chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-'{
                        id.push(chars[i]);
                        i += 1;
                    } else{
                        panic!("invalid identifier constructor: {}", chars[i]);
                    }
                }
                token_stream.push(Token::Identifier(id));
            }
        }
        i += 1;
    }
    token_stream
}

fn main() {
    let in_path = Path::new("./files/a_code.txt");
    let in_display = in_path.display();

    let mut file = match File::open(&in_path) {
        Err(why) => panic!("couldn't open {}: {}", in_display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", in_display, why),
        Ok(_) => println!("{:?}", lex(s.as_str()))
    }
}
