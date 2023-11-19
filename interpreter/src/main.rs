use std::path::Path;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
#[derive(PartialEq)]
enum Token{
    Identifier(String),
    Word(String),
    Equal,
    Comma,
    LeftSquareBracket,
    RightSquareBracket,
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

#[derive(Debug)]
enum Node{
    Assignment(String, Vec<Vec<String>>)
}
fn parse(token_stream: Vec<Token>) -> Vec<Node> {
    let mut parse_tree: Vec<Node> = Vec::new();
    let mut i = 0;
    while i < token_stream.len(){
        match &token_stream[i]{
            Token::Identifier(name) => {
                i += 1;
                match &token_stream[i]{
                    Token::Equal => {
                        i += 1;
                        match &token_stream[i]{
                            Token::LeftSquareBracket => {
                                let mut value: Vec<Vec<String>> = Vec::new();
                                i += 1;
                                let mut temp_vec: Vec<String> = Vec::new();
                                while &token_stream[i] != &Token::RightSquareBracket{
                                    match &token_stream[i]{
                                        Token::Word(name) => temp_vec.push(name.as_str().to_string()),
                                        Token::Comma => {
                                            value.push(temp_vec.clone());
                                            temp_vec.clear();
                                        },
                                        _ => panic!("all arguments in vector should be string or comma")
                                    }
                                    i += 1;
                                }
                                parse_tree.push(Node::Assignment(name.as_str().to_string(), value));
                                i += 1;
                            },
                            _ => panic!("third argument should be left square bracket (for now)")
                        }
                    },
                    _ => panic!("second argument should be equal sign (for now)")
                }
            },
            _ => panic!("first argument should be identifier (for now)")
        }
    }
    parse_tree
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
        Ok(_) => {
            let ts = lex(s.as_str());
            let pt = parse(ts);
            println!("{:?}", pt)
        }
    }
}