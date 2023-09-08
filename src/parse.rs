use pest::{Parser as PestParser, iterators::Pair};

use crate::{lval::{Lval, blispr, sexpr, qexpr, num, sym, add}, lenv::Lenv, error::ReplispResult, eval::lval_eval};

#[derive(Parser)]
#[grammar = "replisp_grammar.pest"] // relative to src
struct ReplispParser;

fn is_bracket_or_eoi(parsed: &Pair<Rule>) -> bool {
	if parsed.as_rule() == Rule::EOI {
		return true;
	}
	let c = parsed.as_str();
	c == "(" || c == ")" || c == "{" || c == "}"
}

// Read a rule with children into the given containing Lval
fn read_to_lval(v: &mut Lval, parsed: Pair<Rule>) -> ReplispResult<()> {
	for child in parsed.into_inner() {
		if is_bracket_or_eoi(&child) {
			continue;
		}
		add(v, &*lval_read(child)?)?;
	}
	Ok(())
}

fn lval_read(parsed: Pair<Rule>) -> ReplispResult<Box<Lval>> {
	match parsed.as_rule() {
		Rule::program => {
			let mut ret = blispr();
			read_to_lval(&mut ret, parsed)?;
			Ok(ret)
		},
		Rule::expr => lval_read(parsed.into_inner().next().unwrap()),
		Rule::sexpr => {
			let mut ret = sexpr();
			read_to_lval(&mut ret, parsed)?;
			Ok(ret)
		},
		Rule::qexpr => {
			let mut ret = qexpr();
			read_to_lval(&mut ret, parsed)?;
			Ok(ret)
		},
		Rule::num => Ok(num(parsed.as_str().parse::<i64>()?)),
		Rule::symbol => Ok(sym(parsed.as_str())),
		_ => unreachable!(), // COMMENT/WHITESPACE etc
	}
}

pub fn eval_str(env: &mut Lenv, source: &str) -> ReplispResult<Box<Lval>> {
	let parsed = ReplispParser::parse(Rule::program, source)?.next().unwrap();
	println!("{}", parsed);
	let mut lval_ptr = lval_read(parsed)?;
	println!("Parsed: {:?}", *lval_ptr);
	lval_eval(env, &mut lval_ptr)
}