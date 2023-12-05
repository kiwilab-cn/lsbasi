//use std::{io::{self, Write}, fmt::Display};
use std::{
    fmt::Display,
    io::{self, Write},
};

#[derive(PartialEq)]
enum TokenType {
    INTEGER,
    PLUS,
    EOF,
}

type CalcTokenType = TokenType;

impl Display for CalcTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CalcTokenType::EOF => write!(f, "EOF"),
            CalcTokenType::INTEGER => write!(f, "INTEGER"),
            CalcTokenType::PLUS => write!(f, "PLUS"),
        }
    }
}

struct Token {
    genre: CalcTokenType,
    value: Option<char>,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(t) => write!(f, "Token({}, {})", &self.genre, t),
            None => write!(f, "Token({}, None)", &self.genre),
        }
    }
}

struct Interpreter {
    text: String,
    pos: i32,
    current_token: Option<Token>,
}

impl Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.current_token {
            Some(t) => write!(
                f,
                "text:{}, post:{}, current_token:{}",
                &self.text, &self.pos, t
            ),
            None => write!(
                f,
                "text:{}, post:{}, current_token: None",
                &self.text, &self.pos
            ),
        }
    }
}

impl Interpreter {
    fn new(text: String) -> Interpreter {
        Interpreter {
            text,
            pos: 0,
            current_token: None,
        }
    }

    fn get_next_token(&mut self) -> Token {
        let text = &self.text;
        let pos = self.pos as usize;

        if pos > text.len() - 1 {
            return Token {
                genre: CalcTokenType::EOF,
                value: None,
            };
        }

        let current_char = text.chars().nth(pos).unwrap();

        if current_char.is_digit(10) {
            let token = Token {
                genre: CalcTokenType::INTEGER,
                value: Some(current_char),
            };
            self.pos += 1;
            return token;
        }

        if current_char == '+' {
            let token = Token {
                genre: CalcTokenType::PLUS,
                value: Some(current_char),
            };
            self.pos += 1;
            return token;
        }

        return Token {
            genre: CalcTokenType::EOF,
            value: None,
        };
    }

    fn eat(&mut self, token_type: CalcTokenType) -> Result<(), bool> {
        let current_token_type = self.current_token.take().unwrap().genre;
        if current_token_type == token_type {
            self.current_token = Some(self.get_next_token());
            Ok(())
        } else {
            Err(true)
        }
    }

    fn expr(&mut self) -> u32 {
        self.current_token = Some(self.get_next_token());

        let left = &self
            .current_token
            .as_ref()
            .unwrap()
            .value
            .unwrap()
            .to_digit(10)
            .unwrap();
        let _ = self.eat(CalcTokenType::INTEGER);

        let _op = &self.current_token.as_ref().unwrap();
        let _ = self.eat(CalcTokenType::PLUS);

        let right = &self
            .current_token
            .as_ref()
            .unwrap()
            .value
            .unwrap()
            .to_digit(10)
            .unwrap();
        let _ = self.eat(CalcTokenType::INTEGER);

        let result = left + right;
        result
    }
}

fn main() -> io::Result<()> {
    loop {
        print!("calc >");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let user_input = input.trim().to_string();
        let mut interpreter = Interpreter::new(user_input);
        let result = interpreter.expr();

        println!("{}", result);
        io::stdout().flush().unwrap();
    }
}
