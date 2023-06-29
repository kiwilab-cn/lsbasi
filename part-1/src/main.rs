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
    fn new(text: String)-> Self {
       Self {
            text: text,
            pos: 0,
            current_token: None,
        }
    }

    fn get_next_token(&self) -> & Token<T> {
        let text = &self.text;
        if self.pos > text.len() {
            return Token{genre: TokenType::EOF, value: None};
        }

        let current_char = text.chars().nth(self.pos);
        if current_char.is_numeric {
            let token = Token{genre: TokenType::INTEGER, value: current_char};
            self.pos += 1;
            return token;
        }

        if current_char == '+' {
            let token = Token{genre: TokenType::PLUS, value: current_char};
            self.pos += 1;
            return token;
        }

    }

    fn eat(&self, token_type: CalcTokenType) {
        if &self.current_token.unwrap().genre == token_type {
            &self.curren_token = self.get_next_token();
        } else {

        }

    }


}


fn main() {
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
}
