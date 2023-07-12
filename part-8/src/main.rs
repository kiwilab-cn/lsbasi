use std::{io::{self, Write}, fmt::Display};

#[derive(PartialEq, Copy, Clone)]
enum TokenType {
    INTEGER,
    EOF,
    DIV,
    MUL,
    PLUS,
    MINUS,
    LPAREN,
    RPAREN,
}

type CalcTokenType = TokenType;

impl Display for CalcTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CalcTokenType::EOF => write!(f, "EOF"),
            CalcTokenType::INTEGER => write!(f, "INTEGER"),
            CalcTokenType::PLUS => write!(f, "PLUS"),
            CalcTokenType::MINUS => write!(f, "MINUS"),
            CalcTokenType::DIV => write!(f, "DIV"),
            CalcTokenType::MUL => write!(f, "MUL"),
            CalcTokenType::LPAREN => write!(f, "("),
            CalcTokenType::RPAREN => write!(f, ")"),
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

struct Lexer<'a> {
    text: &'a String,
    pos: i32,
    current_char: Option<char>,
}

impl<'a> Display for Lexer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.current_char {
            Some(x) => write!(f, "text:{}, post:{}, current_char:{}", &self.text, &self.pos, x),
            None => write!(f, "text:{}, post:{}, current_char: None", &self.text, &self.pos),
        }
    }
}

impl<'a> Lexer<'a> {

    fn new(text: &'a String)-> Lexer<'a> {
        Lexer {
            text: &text,
            pos: 0,
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
            } else if current_char == '*' {
                self.advance();
                return Token{ genre: CalcTokenType::MUL, value: Some(Value::CHAR('*')) };
            } else if current_char == '/' {
                self.advance();
                return Token{ genre: CalcTokenType::DIV, value: Some(Value::CHAR('/')) };
            } else if current_char == '+' {
                self.advance();
                return Token{ genre: CalcTokenType::PLUS, value: Some(Value::CHAR('+')) };
            } else if current_char == '-' {
                self.advance();
                return Token{ genre: CalcTokenType::MINUS, value: Some(Value::CHAR('-')) };
            } else if current_char == '(' {
                self.advance();
                return Token{ genre: CalcTokenType::LPAREN, value: Some(Value::CHAR('(')) };
            } else if current_char == ')' {
                self.advance();
                return Token{ genre: CalcTokenType::RPAREN, value: Some(Value::CHAR(')')) };
            } else {
                println!("Failed to parse input word:{}", current_char);
                return Token{genre: CalcTokenType::EOF, value: None};
            }
        }

        return Token{genre: CalcTokenType::EOF, value: None};
    }
}

enum AST {
    BINOP(BinOp),
    NUM(Num),
    UNARYOP(UnaryOp),
}

struct BinOp {
    left: Box<AST>,
    token: Token,
    op: Token,
    right: Box<AST>,
}

impl BinOp {
    fn new(left: AST, op: Token, right: AST) -> BinOp {
        BinOp {
            left: Box::new(left),
            token: op,
            op: op,
            right: Box::new(right),
        }
    }
}

struct Num {
    token: Token,
    value: Option<Value>,
}

impl Num {
    fn new(token: Token) -> Num {
        Num {
            token: token,
            value: token.value,
        }
    }
}

struct UnaryOp {
    token: Token,
    op: Token,
    expr: Box<AST>,
}

impl UnaryOp {
    fn new(op: Token, expr: AST) -> UnaryOp {
        UnaryOp {
            token: op,
            op: op,
            expr: Box::new(expr)
        }
    }

}

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> Display for Parser<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.current_token {
            Some(x) => write!(f, "lexer:{}, current_token:{}", &self.lexer, x),
            None => write!(f, "lexer:{}, current_token: {}", &self.lexer, "None"),
        }
    }
}

impl<'a> Parser<'a> {

    fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer: lexer,
            current_token: None,
        }
    }

    fn eat(&mut self, token_type: CalcTokenType) -> Result<(), bool> {
        let current_token_type = self.current_token.as_ref().unwrap().genre;
        if current_token_type == token_type {
            self.current_token = Some(self.lexer.get_next_token());
            Ok(())
        } else {
            Err(true)
        }
    }


    fn factor(&mut self) -> Result<AST, char> {
        let token = self.current_token;
        let token_type = &token.unwrap().genre;
        let _value = &token.unwrap().value.unwrap();

        if token_type == &CalcTokenType::INTEGER {
            let _ = self.eat(CalcTokenType::INTEGER);
            return Ok(AST::NUM(Num::new(token.unwrap())));
        } else if token_type == &CalcTokenType::LPAREN {
            let _ = self.eat(CalcTokenType::LPAREN);
            let node = self.expr();
            let _ = self.eat(CalcTokenType::RPAREN);
            return node;
        } else if token_type == &CalcTokenType::PLUS {
            let _ = self.eat(CalcTokenType::PLUS);
            let node = AST::UNARYOP(UnaryOp::new(token.unwrap(), self.factor()?));
            return Ok(node);
        } else if token_type == &CalcTokenType::MINUS {
            let _ = self.eat(CalcTokenType::MINUS);
            let node = AST::UNARYOP(UnaryOp::new(token.unwrap(), self.factor()?));
            return Ok(node);
        }

        return Err('F')
    }

    fn term(&mut self) -> Result<AST, char> {

        let mut node = self.factor()?;
        let action = vec![CalcTokenType::MUL, CalcTokenType::DIV];
        while action.contains(&self.current_token.unwrap().genre) {
            let token = self.current_token.unwrap();
            if token.genre == CalcTokenType::MUL {
                let _= self.eat(CalcTokenType::MUL);
            } else if token.genre == CalcTokenType::DIV {
                let _= self.eat(CalcTokenType::DIV);
            }

            node = AST::BINOP(BinOp::new(node, token, self.factor()?));
        }

        return Ok(node);
    }

    fn expr(&mut self) -> Result<AST, char> {

        let mut node = self.term()?;
        let action = vec![CalcTokenType::PLUS, CalcTokenType::MINUS];
        while action.contains(&self.current_token.unwrap().genre) {
            let token = self.current_token.unwrap();
            if token.genre == CalcTokenType::PLUS{
                let _= self.eat(CalcTokenType::PLUS);
            } else if token.genre == CalcTokenType::MINUS {
                let _= self.eat(CalcTokenType::MINUS);
            }

            node = AST::BINOP(BinOp::new(node, token, self.term()?));
        }

        return Ok(node);
    }

    fn parse(&mut self) -> Result<AST, char> {
        self.current_token = Some(self.lexer.get_next_token());
        return self.expr();
    }
}

struct Interpreter<'a> {
    parser: Parser<'a>,
}

impl<'a> Interpreter<'a> {
    fn new(parser: Parser<'a>) -> Interpreter {
        Interpreter { parser: parser }
    }

    fn visit(&self, node: Box<AST>) -> i32 {
        match *node {
            AST::BINOP(x) => {
                self.visit_binop(x)
            },
            AST::NUM(y) => {
                self.visit_num(y)
            }
            AST::UNARYOP(z) => {
                self.visit_unaryop(z)
            }
        }
    }

    fn visit_binop(&self, node: BinOp) -> i32{
        let op_type = node.op.genre;

        match op_type {
            CalcTokenType::PLUS => self.visit(node.left) + self.visit(node.right),
            CalcTokenType::MINUS => self.visit(node.left) - self.visit(node.right),
            CalcTokenType::MUL => self.visit(node.left) * self.visit(node.right),
            CalcTokenType::DIV => self.visit(node.left) / self.visit(node.right),
            _ => {
                println!("Failed to found error op type: {}", op_type);
                0
            },
        }
    }

    fn visit_num(&self, node: Num) -> i32{
        match node.value.unwrap() {
            Value::CHAR(x) => {
                println!("Failed to get value: {}, it should be numeric.", x);
                0
            },
            Value::INT(y) => y,
        }
    }

    fn visit_unaryop(&self, node: UnaryOp) -> i32 {
        let op = node.op.genre;
        match op {
            CalcTokenType::PLUS => {
                self.visit(node.expr)
            },
            CalcTokenType::MINUS => {
                -self.visit(node.expr)
            },
            _ => {
                println!("Failed to parse error op type: {}", op);
                0
            },
        }
    }

    fn interpret(&mut self) -> Result<i32, char> {
        let tree = self.parser.parse()?;
        return Ok(self.visit(Box::new(tree)));
    }
}

fn main() -> io::Result<()>{
    loop {
        print!("calc > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let user_input = input.trim().to_string();
        let lexer = Lexer::new(&user_input);
        let parser = Parser::new(lexer);
        let mut interpreter = Interpreter::new(parser);
        let result = interpreter.interpret();

        match result {
            Ok(v) => println!("{v:?}"),
            Err(e) => println!("Error when calculate expression: {e:?}"),
        }
        io::stdout().flush().unwrap();
    }

}
