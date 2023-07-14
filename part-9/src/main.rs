use std::{io::{self, Write}, fmt::{Display}, collections::HashMap};

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
    ID,
    ASSIGN,
    BEGIN,
    END,
    SEMI,
    DOT,
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
            CalcTokenType::ID => write!(f, "ID"),
            CalcTokenType::ASSIGN => write!(f, ":="),
            CalcTokenType::BEGIN => write!(f, "BEGIN"),
            CalcTokenType::END => write!(f, "END"),
            CalcTokenType::SEMI => write!(f, ";"),
            CalcTokenType::DOT => write!(f, "."),
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
enum Value {
    CHAR(char),
    INT(i32),
    STRING(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(PartialEq)]
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

const RESERVED_KEYWORDS: HashMap<String, Token> = HashMap::from([
    (String::from("BEGIN"), Token{genre: CalcTokenType::BEGIN, value: Some(Value::STRING(String::from("BEGIN")))}),
    (String::from("END"), Token{genre: CalcTokenType::END, value: Some(Value::STRING(String::from("END")))}),
]);

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

    fn peek(&self) -> Option<char> {
        let peek_pos = self.pos + 1;
        if peek_pos > self.text.len() as i32 - 1 {
            None
        } else {
            self.text.chars().nth(peek_pos as usize)
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

    fn _id(self) -> Token {
        let mut result = String::from("");
        while self.current_char != None && self.current_char.unwrap().is_alphanumeric() {
            result.push(self.current_char.unwrap());
            self.advance();
        }

        if RESERVED_KEYWORDS.contains_key(&result) {
            *RESERVED_KEYWORDS.get(&result).unwrap()
        } else {
            Token { genre: CalcTokenType::ID, value: Some(Value::STRING(result)) }
        }
    }

    fn get_next_token(&mut self) -> Token {
        while let Some(current_char) = self.current_char {
            if current_char.is_whitespace() {
                self.skip_whitespace();
                continue;
            } else if current_char.is_alphabetic() {
                return self._id();
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
            } else if current_char == '.' {
                self.advance();
                return Token{ genre: CalcTokenType::DOT, value: Some(Value::CHAR('.')) };} else if current_char == ';' {self.advance();
                return Token{ genre: CalcTokenType::SEMI, value: Some(Value::CHAR(';')) };
            } else if current_char == ':' && self.peek().unwrap() == '=' {
                self.advance();
                self.advance();
                return Token{ genre: CalcTokenType::ASSIGN, value: Some(Value::STRING(String::from(":="))) };
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
    COMPOUND(Compound),
    NOOP(NoOp),
    VAR(Var),
    ASSIGN(Assign),
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

struct Compound {
    children : Vec<Box<AST>>,
}

impl Compound {
    fn new() -> Compound {
        Compound {
            children: Vec::new()
        }
    }
}

struct Assign {
    left: Box<AST>,
    op: Token,
    token: Token,
    right: Box<AST>,
}

impl Assign {
    fn new(left: AST, op: Token, right: AST) -> Assign {
        Assign {
            left: Box::new(left),
            op: op,
            token: op,
            right: Box::new(right)
        }
    }

}

struct Var {
    token: Token,
    value: Option<Value>,
}

impl Var {
    fn new(token: Token) -> Var{
        Var {
            token: token,
            value: token.value,
        }
    }
}

struct NoOp {

}

impl NoOp {
    fn new() -> NoOp {
        NoOp {  }
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

    fn program(&self) -> Result<AST, char> {
        let node = self.compound_statement();
        let _ = self.eat(CalcTokenType::DOT);
        return node;
    }

    fn compound_statement(&self) -> Result<AST, char> {
        let _ = self.eat(CalcTokenType::BEGIN);
        let mut nodes = self.statement_list().unwrap();

        let mut root = Compound::new();
        root.children.append(&mut nodes);

        return Ok(AST::COMPOUND(root));
    }

    fn statement_list(&self) -> Result<Vec<Box<AST>>, char> {
        let statement = self.statement().unwrap();
        let mut results = Vec::new();
        results.push(Box::new(statement));

        while self.current_token.unwrap().genre == CalcTokenType::SEMI {
            let _ = self.eat(CalcTokenType::SEMI);
            results.push(Box::new(self.statement().unwrap()));
        }

        if self.current_token.unwrap().genre == CalcTokenType::ID {
            return Err('G');
        }

        return Ok(results);
    }

    fn statement(&self) -> Result<AST, char> {
        let token_type =self.current_token.unwrap().genre;
        if token_type == CalcTokenType::BEGIN {
            self.compound_statement()
        } else if token_type == CalcTokenType::ID {
            self.assignment_statement()
        } else {
            self.empty()
        }
    }

    fn assignment_statement(&self) -> Result<AST, char> {
        let left = self.variable().unwrap();
        let token = self.current_token.unwrap();
        let _ = self.eat(CalcTokenType::ASSIGN);
        let right = self.expr().unwrap();
        let node = Assign::new(left, token, right);
        return Ok(AST::ASSIGN(node));
    }

    fn variable(&self) -> Result<AST, char> {
        let node = Var::new(self.current_token.unwrap());
        let _ = self.eat(CalcTokenType::ID);
        return Ok(AST::VAR(node));
    }

    fn empty(&self) -> Result<AST, char> {
        Ok(AST::NOOP(NoOp::new()))
    }

    fn factor(&mut self) -> Result<AST, char> {
        let token = self.current_token;
        let token_type = &token.unwrap().genre;
        let _value = &token.unwrap().value.unwrap();

        if token_type == &CalcTokenType::INTEGER {
            let _ = self.eat(CalcTokenType::INTEGER);

            Ok(AST::NUM(Num::new(token.unwrap())))
        } else if token_type == &CalcTokenType::LPAREN {
            let _ = self.eat(CalcTokenType::LPAREN);
            let node = self.expr();
            let _ = self.eat(CalcTokenType::RPAREN);

            node
        } else if token_type == &CalcTokenType::PLUS {
            let _ = self.eat(CalcTokenType::PLUS);
            let node = AST::UNARYOP(UnaryOp::new(token.unwrap(), self.factor()?));

            Ok(node)
        } else if token_type == &CalcTokenType::MINUS {
            let _ = self.eat(CalcTokenType::MINUS);
            let node = AST::UNARYOP(UnaryOp::new(token.unwrap(), self.factor()?));

            Ok(node)
        } else {
            self.variable()
        }
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
        let node = self.program();
        if self.current_token.unwrap().genre != CalcTokenType::EOF {
            Err('Z')
        } else {
            node
        }
    }
}

struct Interpreter<'a> {
    parser: Parser<'a>,
    temp: HashMap<Value, i32>,
}

impl<'a> Interpreter<'a> {
    fn new(parser: Parser<'a>) -> Interpreter {
        Interpreter {
            parser: parser,
            temp: HashMap::new(),
        }
    }

    fn visit(&self, node: Box<AST>) -> i32 {
        match *node {
            AST::BINOP(bin_op) => {
                self.visit_binop(bin_op)
            },
            AST::NUM(num) => {
                self.visit_num(num)
            },
            AST::UNARYOP(unary_op) => {
                self.visit_unaryop(unary_op)
            },
            AST::COMPOUND(compound) => {
                let _= self.visit_compound(compound);
                0
            },
            AST::ASSIGN(assign) => {
                let _= self.visit_assign(assign);
                0
            },
            AST::VAR(var) => {
                let _= self.visit_var(var);
                0
            },
            AST::NOOP(no_op) => {
                let _= self.visit_noop(no_op);
                0
            },
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
            Value::STRING(z) => {
                println!("Failed to get num value: {}", z);
                0
            }
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

    fn visit_compound(self, node: Compound) {
        for child in &node.children {
            let _= self.visit(*child);
        }
    }

    fn visit_assign(self, node: Assign) {
        let name = node.left;
        match *name {
            AST::VAR(var) => {
                self.temp.insert(var.value.unwrap(), self.visit(node.right));
            },
            _ => {},
        }
    }

    fn visit_var(self, node: Var) -> i32 {
        let var_name = node.value;
        let val = self.temp.get(&var_name.unwrap());
        *val.unwrap()
    }

    fn visit_noop(self, node: NoOp) -> i32 {
        0
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
        let temp = interpreter.temp;

        for (k, v) in &temp {
            println!("{}: {}", k, v);
        }

        match result {
            Ok(v) => println!("{v:?}"),
            Err(e) => println!("Error when calculate expression: {e:?}"),
        }
        io::stdout().flush().unwrap();
    }

}
