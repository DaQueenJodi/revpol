use std::str::FromStr;
use rustyline as rl;
#[derive(Debug)]
enum OperatorFlavor {
	Add,
	Sub,
	Mul,
	Div
}

#[derive(Debug, Clone, Copy)]
enum Number {
	Float(f64)
}

macro_rules! impl_comp {
	($trait:ident, $func:ident) => {
		use std::ops::$trait;
		impl $trait for Number {
			type Output = Self;
			fn $func(self, other: Self) -> Self {
				match self {
					Number::Float(a) => {
						match other {
							Number::Float(b) => Number::Float(a.$func(b))
						}
					}
				}
			}
		}
	}
}
impl_comp!(Add, add);
impl_comp!(Sub, sub);
impl_comp!(Div, div);
impl_comp!(Mul, mul);

#[derive(Debug)]
enum Token {
	Operator(OperatorFlavor),
	Number(Number)
}

#[derive(Debug, PartialEq, Eq)]
struct TokenParseError;

impl FromStr for Token {
  type Err = TokenParseError;
	fn from_str(s: &str) -> Result<Token, Self::Err> {
		use OperatorFlavor::*;
		Ok(match s {
			"+" => Token::Operator(Add),
			"-" => Token::Operator(Sub),
			"*" => Token::Operator(Mul),
			"/" => Token::Operator(Div),
			n => {
				match n.parse::<f64>() {
					Ok(n) => Token::Number(Number::Float(n)),
					Err(_) => return Err(TokenParseError)
				}
			}
		})
	}
}

fn lexify(text: &str) -> Result<Vec<Token>, TokenParseError> {
	Ok(text.split(' ').map(|s| Token::from_str(s).unwrap()).collect())
}

macro_rules! do_op {
	($stack:expr, $func:ident) => {
		{
			let b = $stack.pop().unwrap();
			let a = $stack.pop().unwrap();
			a.$func(b)
		}
	}
}

fn interpretify(tokens: &[Token]) -> f64 {
	let mut stack: Vec<Number> = Vec::new();
	for token in tokens {
		use OperatorFlavor::*;
			let number = match token {
				Token::Number(n) => *n,
				Token::Operator(flavor) => {
					match flavor {
						Add => do_op!(stack, add),
						Sub => do_op!(stack, sub),
						Mul => do_op!(stack, mul),
						Div => do_op!(stack, div)
					}
				}
			};
			stack.push(number);
	}
	let Number::Float(n) = stack.pop().unwrap();
	n
}

fn main() -> rl::Result<()> {
	let mut line = rl::DefaultEditor::new()?;
	loop {
		let readline = line.readline(">> ")?;
		let tokens = lexify(&readline.trim()).unwrap();
		let result = interpretify(&tokens);
		if result.fract() == 0.0 {
			println!("{}", result as i32)
		} else {
			println!("{}", result)
		}
	}
}
