//use std::fmt;
#[derive(Debug)]
enum TokenKind {
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Slash,
	Star,

	//One Or Two Character Token
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	// Literals
	Identifier,
	String,
	Number,

	// Keywords
	And,
	Class,
	Else,
	False,
	Fun,
	For,
	If,
	Nil,
	Or,
	Print,
	Return,
	Super,
	This,
	True,
	Var,
	While,
	Eof,
}
#[derive(Debug)]
pub struct Token {
	kind: TokenKind,
	literal:  char,
	line: u16,
}

#[derive(Debug)]
pub struct Scanner {
	source: String,
	pub tokens:  Vec<Token>,
	start: u32,
	current: u32,
	line: u16,
}

impl Scanner {
	pub fn new(s: String) -> Scanner {
		Scanner {
			source: s,
			tokens: Vec::new(),
			start: 0,
			current: 0,
			line: 1,
		}
	}
	pub fn scan_tokens(&mut self) -> &Vec<Token> {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token();
		}
		return &self.tokens;
	}
	fn scan_token(&mut self) {
		let c = self.advance();
		match c {
			'(' => self.add_token(TokenKind::LeftParen,c),

			_ => println!("Unexpected character: {}", c),
		}
	}

	fn add_token(&mut self, kind:TokenKind, literal: char) {
		let token = Token{
			kind, 
			literal:literal,
			line:0,
			};
		self.tokens.push(token);
	}

	fn is_at_end(&self) -> bool {
		self.current as usize >= self.source.chars().count()
	}

	fn advance(&mut self) -> char {
		self.current += 1;
		//let index =
		self.source.chars().nth(self.current as usize - 1).unwrap()
	}
}
