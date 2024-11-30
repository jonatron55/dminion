use rand::Rng;

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod lookahead;
pub mod parser;
pub mod pp;

pub fn roll<TRng: Rng>(input: &str, rng: &mut TRng) -> Result<i32, Box<dyn std::error::Error>> {
    let root = parser::parse(&input)?;
    let mut evaluator = eval::Evaluator::new(eval::Evaluation::Rand(rng));
    evaluator.eval(root.as_ref())
}
