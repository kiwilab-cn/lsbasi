use strum_macros::Display;

#[derive(Display)]
enum TokenType {
    INTEGER,
    PLUS,
    EOF
}

struct Token<T> {
    type: TokenType,
    value: T,
}

impl<T> Token<T> {
    fn str(&self) -> String {
        let value = repr(self.value);
        format!("Token({}, {})", self.type, value)
    }

    fn repr(&self) -> String() {
        str(&self)
    }
}


fn main() {
    println!("Hello, world!");
}
