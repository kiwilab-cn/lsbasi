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
