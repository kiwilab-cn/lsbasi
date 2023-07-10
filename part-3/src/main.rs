use std::{io::{self, Write}, fmt::Display};

#[derive(PartialEq, Copy, Clone)]
enum TokenType {
    INTEGER,
    PLUS,
    EOF,
    MINUS
}

type CalcTokenType = TokenType;

impl Display for CalcTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CalcTokenType::EOF => write!(f, "EOF"),
            CalcTokenType::INTEGER => write!(f, "INTEGER"),
            CalcTokenType::PLUS => write!(f, "PLUS"),
            CalcTokenType::MINUS => write!(f, "MINUS"),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Value {
    CHAR(char),
    INT(i32),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Copy, Clone)]
struct Token {
    genre: CalcTokenType,
    value: Option<Value>,
}


impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(t) => write!(f, "Token({}, {})", &self.genre, t),
            None => write!(f, "Token({}, None)", &self.genre),
        }
    }
}

struct Interpreter<'a> {
    text: &'a String,
    pos: i32,
    current_token: Option<Token>,
    current_char: Option<char>,
}

impl<'a> Display for Interpreter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.current_token, &self.current_char) {
            (Some(x), Some(y)) => write!(f, "text:{}, post:{}, current_token:{}, current_char:{}",
                              &self.text, &self.pos, x, y),
            (Some(x), None) => write!(f, "text:{}, post:{}, current_token:{}, current_char:{}",
                              &self.text, &self.pos, x, "None"),
            (None, Some(y)) => write!(f, "text:{}, post:{}, current_token:{}, current_char:{}",
                              &self.text, &self.pos, "None", y),
            (None, None) => write!(f, "text:{}, post:{}, current_token: {}, current_char:{}",
                           &self.text, &self.pos, "None", "None"),
        }
    }
}

impl<'a> Interpreter<'a> {
    fn new(text: &'a String)-> Interpreter<'a> {
        Interpreter {
            text: &text,
            pos: 0,
            current_token: None,
            current_char: Some(text.chars().nth(0).unwrap()),
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos > self.text.len() as i32 - 1 {
            self.current_char = None
        } else {
            self.current_char = Some(self.text.chars().nth(self.pos as usize).unwrap())
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char != None && self.current_char.unwrap().is_whitespace() {
            self.advance();
        }
    }

    fn integer(&mut self) -> i32 {
        let mut result = String::from("");

        while self.current_char != None && self.current_char.unwrap().is_digit(10) {
            result.push(self.current_char.unwrap());
            self.advance();
        }

        return result.parse::<i32>().unwrap();
    }

    fn get_next_token(&mut self) -> Token {
        while let Some(current_char) = self.current_char {
            if current_char.is_whitespace() {
                self.skip_whitespace();
                continue;
            } else if current_char.is_digit(10) {
                return Token{ genre: CalcTokenType::INTEGER, value: Some(Value::INT(self.integer()))};
            } else if current_char == '+' {
                self.advance();
                return Token{ genre: CalcTokenType::PLUS, value: Some(Value::CHAR('+')) };
            } else if current_char == '-' {
                self.advance();
                return Token{ genre: CalcTokenType::MINUS, value: Some(Value::CHAR('-')) };
            } else {
                println!("Failed to parse input word:{}", current_char);
                return Token{genre: CalcTokenType::EOF, value: None};
            }
        }

        return Token{genre: CalcTokenType::EOF, value: None};
    }

    fn eat(&mut self, token_type: CalcTokenType) -> Result<(), bool> {
        let current_token_type = self.current_token.as_ref().unwrap().genre;
        if current_token_type == token_type {
            self.current_token = Some(self.get_next_token());
            Ok(())
        } else {
            Err(true)
        }
    }

    fn term(&mut self) -> Result<i32, char> {
        let token = self.current_token;
        let _ = self.eat(CalcTokenType::INTEGER);
        let value = &token.unwrap().value.unwrap();

        match value {
            &Value::INT(x) => {
                return Ok(x);
            },
            &Value::CHAR(y) => {
                return Err(y);
            },
        }
    }

    fn expr(&mut self) -> Result<i32, char> {
        self.current_token = Some(self.get_next_token());

        let mut result = self.term().unwrap();
        let action = vec![CalcTokenType::PLUS, CalcTokenType::MINUS];
        while action.contains(&self.current_token.unwrap().genre) {
            let token = self.current_token;
            if token.unwrap().genre == CalcTokenType::PLUS {
                let _= self.eat(CalcTokenType::PLUS);
                result += self.term().unwrap();
            } else if token.unwrap().genre == CalcTokenType::MINUS {
                let _= self.eat(CalcTokenType::MINUS);
                result -= self.term().unwrap();
            }
        }

        return Ok(result);
    }
}


fn main() -> io::Result<()>{
    loop {
        print!("calc > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let user_input = input.trim().to_string();
        let mut interpreter = Interpreter::new(&user_input);
        let result = interpreter.expr();

        match result {
            Ok(v) => println!("{v:?}"),
            Err(e) => println!("Error when calculate expression: {e:?}"),
        }
        io::stdout().flush().unwrap();
    }

}
