use std::io;
use strum_macros::Display;

#[derive(Display)]
enum TokenType {
    INTEGER,
    PLUS,
    EOF
}

type CalcTokenType = TokenType;

struct Token<T> {
    genre: CalcTokenType,
    value: T,
}

impl<T> Token<T> where T: std::fmt::Display {
    fn display(&self) -> String {
        format!("Token({}, {})", &self.genre, &self.value)
    }
}

struct Interpreter<'a, T> {
    text: String,
    pos: i32,
    current_token: Option<&'a Token<T>>,
}

impl<'a, T> Interpreter<'a, T> {
    fn new(text: String)-> Interpreter<'a, T>{
       Interpreter {
            text: text,
            pos: 0,
            current_token: None,
        }
    }
}

impl<'a, T> Interpreter<'a, T>
where
    T: PartialOrd + Clone + Copy {
    fn get_next_token(&self) -> &Token<T> {
        let text = &self.text;
        if self.pos as usize > text.len() {
            return &Token{genre: TokenType::EOF, value: '\0'};
        }

        let current_char = text.chars().nth(self.pos as usize).unwrap();
        if current_char.is_numeric() {
            let token = Token{genre: TokenType::INTEGER, value: current_char.to_digit(10).unwrap()};
            self.pos += 1;
            return &token;
        } else if current_char == '+' {
            let token = Token{genre: TokenType::PLUS, value: current_char};
            self.pos += 1;
            return &token;
        }

        return &Token { genre: (TokenType::EOF), value: '\0' };
    }
}


impl<'a, T> Interpreter<'a, T> {
    fn eat(&self, token_type: CalcTokenType)-> Result<(), bool> {
        if &self.current_token.unwrap().genre == token_type {
            &self.current_token = self.get_next_token();
            return Ok(());
        } else {
            return Err(false);
        }
    }
}

impl<'a, T> Interpreter<'a, T> {
    fn expr(&self) -> i32 {
        &self.current_token = &self.get_next_token();
        let left = &self.current_token;
        &self.eat(CalcTokenType::INTEGER);

        let op = &self.current_token;
        &self.eat(CalcTokenType::PLUS);

        let right = &self.current_token;
        &self.eat(CalcTokenType::INTEGER);

        let result = left.unwrap().value + right.unwrap().value;
        return result;
    }
}

fn main() -> io::Result<()>{
    /*
    let x = Token {genre: TokenType::INTEGER, value: 3};
    println!("{}, {}", x.genre.to_string(), x.value);
    println!("{}", x.display());

    let y: Interpreter<'_, u32> = Interpreter::new(String::from("Test ABC"));
    let z: Interpreter<'_, u32> = Interpreter{
        text: String::from("Test ABC."),
        pos: 0,
        current_token: None,
    };

    println!("{}, {}", z.text, y.text);
    */
    loop {
        let mut input = String::new();
        print!("calc >");
        io::stdin().read_line(&mut input)?;
        println!("input: {}", input.trim())
    }

}
