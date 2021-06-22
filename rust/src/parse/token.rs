use crate::value::{Value, Numeral, Text};
use super::{Stream, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
	Class,
	Method,
	Field,
	ClassField,
	ClassFn,
	Constructor,
	Function,

	Global,
	Local,

	If,
	Else,
	ComeFrom,
	While,
	Return,
	Try,
	Catch,
	Throw,
	Switch,
	Case,
	Assert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParenKind {
	Round,
	Square,
	Curly
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol {
	Endline,
	Comma,
	Colon,
	Dot,
	Equal,

	EqualEqual,
	NotEqual,
	LessThan,
	LessThanOrEqual,
	GreaterThan,
	GreaterThanOrEqual,
	Compare,

	Plus,
	PlusEqual,
	Hyphen,
	HyphenEqual,
	Asterisk,
	AsteriskEqual,
	Solidus,
	SolidusEqual,
	PercentSign,
	PercentSignEqual,

	Exclamation,
	AndAnd,
	OrOr
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
	Keyword(Keyword),
	Symbol(Symbol),
	LeftParen(ParenKind),
	RightParen(ParenKind),
	Literal(Value),
	Identifier(String),
	Label(String),
}

#[derive(Debug)]
pub struct Tokenizer<'a, I> {
	stream: &'a mut Stream<'a, I>
}

impl<'a, I> Tokenizer<'a, I> {
	pub fn new(stream: &'a mut Stream<'a, I>) -> Self {
		Self { stream }
	}
}

impl<I: Iterator<Item=char>> Tokenizer<'_, I> {
	pub const CLASS: &'static str           = "form";
	pub const METHOD: &'static str          = "change";
	pub const FIELD: &'static str           = "matter";
	pub const CLASS_FIELD: &'static str     = "essence";
	pub const CLASS_FN: &'static str        = "recall";
	pub const CONSTRUCTOR: &'static str     = "imitate";
	pub const FUNCTION: &'static str        = "journey";

	pub const GLOBAL: &'static str          = "renowned";
	pub const LOCAL: &'static str           = "nigh";

	pub const IF: &'static str              = "if";
	pub const ELSE: &'static str            = "alas";
	pub const COME_FROM: &'static str       = "whence";
	pub const WHILE: &'static str           = "whilst";
	pub const RETURN: &'static str          = "reward";
	pub const TRY: &'static str             = "attempt";
	pub const CATCH: &'static str           = "retreat"; // todo: deprecate
	pub const THROW: &'static str           = "catapult";
	pub const SWITCH: &'static str          = "fork";
	pub const CASE: &'static str            = "path";
	pub const ASSERT: &'static str          = "challenge";

	pub const TRUE: &'static str            = "yay";
	pub const FALSE: &'static str           = "nay";
	pub const NULL: &'static str            = "ni";

	pub fn next_keyword(&mut self) -> Option<Keyword> {
		macro_rules! keyword {
			($string:ident, $name:ident) => {
				if self.stream.take_identifier(Self::$string) {
					return Some(Keyword::$name)
				}
			};
		}

		keyword!(CLASS, Class);
		keyword!(METHOD, Method);
		keyword!(FIELD, Field);
		keyword!(CLASS_FIELD, ClassField);
		keyword!(CLASS_FN, ClassFn);
		keyword!(CONSTRUCTOR, Constructor);
		keyword!(FUNCTION, Function);

		keyword!(GLOBAL, Global);
		keyword!(LOCAL, Local);

		keyword!(IF, If);
		keyword!(ELSE, Else);
		keyword!(COME_FROM, ComeFrom);
		keyword!(WHILE, While);
		keyword!(RETURN, Return);
		keyword!(TRY, Try);
		keyword!(CATCH, Catch);
		keyword!(THROW, Throw);
		keyword!(SWITCH, Switch);
		keyword!(CASE, Case);
		keyword!(ASSERT, Assert);

		None
	}

	fn parse_arabic_numeral(&mut self) -> Option<Result<Numeral>> {
		use crate::value::numeral::NumeralParseError;

		let parsed = self.stream.take_while(|chr| chr.is_ascii_digit() || chr == '_')?;

		match self.stream.peek() {
			Some(chr) if chr.is_alphanumeric() => Some(Err(self.stream.error(NumeralParseError::BadTrailingChar(chr)))),
			_ => Some(Ok(parsed.parse().unwrap()))
		}
	}

	fn parse_roman_numeral(&mut self) -> Option<Result<Numeral>> {
		use crate::value::numeral::RomanNumeral;
		let parsed = self.stream.take_while(|chr| RomanNumeral::from_char(chr).is_some() || chr == '_')?;

		// if it's an identifier, then don't raise an error.
		if self.stream.peek().map_or(false, char::is_alphanumeric) {
			self.stream.put_back(parsed.chars());
			return None;
		}

		Some(parsed.parse().map_err(|err| self.stream.error(err)))
	}

	fn parse_numeral(&mut self) -> Option<Result<Numeral>> {
		"Ⅰ Ⅱ Ⅲ Ⅳ Ⅴ Ⅵ Ⅶ Ⅷ Ⅸ Ⅹ Ⅺ Ⅻ Ⅼ Ⅽ Ⅾ Ⅿ
		 ⅰ ⅱ ⅲ ⅳ ⅴ ⅵ ⅶ ⅷ ⅸ ⅹ ⅺ ⅻ ⅼ ⅽ ⅾ ⅿ";
		self.parse_roman_numeral().or_else(|| self.parse_arabic_numeral())
	}

	fn parse_text(&mut self) -> Option<Result<Text>> {
		fn is_fraktur(chr: char) -> bool {
			"𝔄𝔅ℭ𝔇𝔈𝔉𝔊ℌℑ𝔍𝔎𝔏𝔐𝔑𝔒𝔓𝔔ℜ𝔖𝔗𝔘𝔙𝔚𝔛𝔜ℨ𝔞𝔟𝔠𝔡𝔢𝔣𝔤𝔥𝔦𝔧𝔨𝔩𝔪𝔫𝔬𝔭𝔮𝔯𝔰𝔱𝔲𝔳𝔴𝔵𝔶𝔷".contains(chr)
		 // ^^^ make sure to  convert to their ascii equiv.
		}

		if let Some(fraktur) = self.stream.take_while(|chr| chr.is_whitespace() || is_fraktur(chr)) {
			return Some(Ok(Text::new(translate_fraktur()
				fraktur.chars()
					.map(|chr| if chr.is_whitespace() {
						chr
					} else {
						"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
					})
			)));
		}
		todo!()
	}

	pub fn next_literal(&mut self) -> Option<Result<Value>> {
		if let Some(numeral) = self.parse_numeral() {
			Some(numeral.map(Value::Numeral))
		} else if let Some(text) = self.parse_text() {
			Some(text.map(Value::Text))
		} else if self.stream.take_identifier(Self::TRUE) {
			Some(Ok(Value::Veracity(true)))
		} else if self.stream.take_identifier(Self::FALSE) {
			Some(Ok(Value::Veracity(false)))
		} else if self.stream.take_identifier(Self::NULL) {
			Some(Ok(Value::Null))
		} else {
			None
		}
	}

	fn next_from_stream(&mut self) -> Option<Result<Token>> {
		macro_rules! if_equals {
			($if_eql:ident, $if_not:ident) => {
				if self.stream.take_prefix("=") {
					Token::Symbol(Symbol::$if_eql)
				} else {
					Token::Symbol(Symbol::$if_not)
				}
			};
		}

		if let Some(kw) = self.next_keyword() {
			return Some(Ok(Token::Keyword(kw)));
		} else if let Some(literal) = self.next_literal() {
			return Some(literal.map(Token::Literal));
		}

		Some(Ok(match self.stream.next()? {
			// parens
			'(' => Token::LeftParen(ParenKind::Round),
			'[' => Token::LeftParen(ParenKind::Square),
			'{' => Token::LeftParen(ParenKind::Curly),
			')' => Token::RightParen(ParenKind::Round),
			']' => Token::RightParen(ParenKind::Square),
			'}' => Token::RightParen(ParenKind::Curly),

			// symbols
			';' => Token::Symbol(Symbol::Endline),
			',' => Token::Symbol(Symbol::Comma),
			':' => Token::Symbol(Symbol::Colon),
			'.' => Token::Symbol(Symbol::Dot),
			'=' => if_equals!(EqualEqual, Equal),
			'!' => if_equals!(NotEqual, Exclamation),
			'<' => 
				if self.stream.take_prefix("=>") {
					Token::Symbol(Symbol::Compare)
				} else {
					if_equals!(LessThanOrEqual, LessThan)
				},
			'>' => if_equals!(GreaterThanOrEqual, GreaterThan),
			'+' => if_equals!(PlusEqual, Plus),
			'-' => if_equals!(HyphenEqual, Hyphen),
			'*' => if_equals!(AsteriskEqual, Asterisk),
			'/' => if_equals!(SolidusEqual, Solidus),
			'%' => if_equals!(PercentSignEqual, PercentSign),
			'&' if self.stream.take_prefix("&") => Token::Symbol(Symbol::AndAnd),
			'|' if self.stream.take_prefix("|") => Token::Symbol(Symbol::OrOr),
			// '0'..='9' => self.parse_
			_ => todo!()
		}))
	}
}

impl<I: Iterator<Item=char>> Iterator for Tokenizer<'_, I> {
	type Item = Result<Token>;

	fn next(&mut self) -> Option<Self::Item> {
		self.next_from_stream()
	}
}
