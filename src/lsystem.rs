use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::{iterator, map_res, opt},
    error::{Error, ErrorKind},
    IResult,
};

pub type Instructions = Vec<Instruction>;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Symbol(char),
    Branch(Instructions),
}

fn to_symbol(input: &str) -> Result<Instruction, Box<dyn std::error::Error>> {
    if input.len() != 1 {
        return Err(format!(
            "'{}' is not a single instruction that is able to be parsed",
            input
        )
        .into());
    }

    // safe to unwrap since input is exactly one character
    let c = input.chars().next().unwrap();

    if is_branch_symbol(c) {
        Err(format!("{} is branch symbol", input).into())
    } else if c.is_whitespace() {
        Err(format!("'{}' is whitespace", input).into())
    } else if c == ';' {
        Err(format!("'{}' is terminator", input).into())
    } else {
        Ok(Instruction::Symbol(c))
    }
}

fn is_branch_symbol(c: char) -> bool {
    c == '[' || c == ']'
}

fn single_instruction(input: &str) -> IResult<&str, Instruction> {
    map_res(take_while_m_n(1, 1, |c| !is_branch_symbol(c)), to_symbol)(input)
}

fn simple_instructions(input: &str) -> IResult<&str, Instructions> {
    let mut it = iterator(input, single_instruction);

    let parsed: Instructions = it.collect();
    if parsed.is_empty() {
        Err(nom::Err::Error(Error {
            input,
            code: ErrorKind::Fail,
        }))
    } else {
        let (input, ()) = it.finish()?;

        Ok((input, parsed))
    }
}

fn branch(input: &str) -> IResult<&str, Instructions> {
    let (input, _) = tag("[")(input)?;
    let (input, instrs) = instructions(input)?;
    let (input, _) = tag("]")(input)?;

    Ok((input, vec![Instruction::Branch(instrs)]))
}

fn instructions(input: &str) -> IResult<&str, Instructions> {
    let (input, _) = opt(remove_whitespace)(input)?;

    let mut it = iterator(
        input,
        alt((simple_instructions, branch, |input| {
            remove_whitespace(input).map(|(input, _)| (input, vec![]))
        })),
    );

    let parsed = it.flatten().collect();
    let (input, ()) = it.finish()?;

    Ok((input, parsed))
}

fn remove_whitespace(input: &str) -> IResult<&str, ()> {
    let mut it = iterator(input, alt((tag(" "), tag("\n"), tag("\t"))));
    let l: Vec<_> = it.collect();
    if l.is_empty() {
        Err(nom::Err::Error(Error {
            input,
            code: ErrorKind::Fail,
        }))
    } else {
        it.finish()
    }
}

type Rule = (Instruction, Instructions);

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, _) = opt(remove_whitespace)(input)?;

    let (input, from) = single_instruction(input)?;
    let (input, _) = opt(remove_whitespace)(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, _) = opt(remove_whitespace)(input)?;
    let (input, target) = instructions(input)?;

    Ok((input, (from, target)))
}

#[derive(Debug, PartialEq)]
pub struct LSystem {
    word: Instructions,
    axiom: Instructions,
    rules: Vec<Rule>,
}

fn terminate<F, G>(f: F) -> impl Fn(&str) -> IResult<&str, G>
where
    F: Fn(&str) -> IResult<&str, G>,
{
    move |input| {
        let (input, res) = f(input)?;
        let (input, _) = opt(remove_whitespace)(input)?;
        let (input, _) = tag(";")(input)?;
        Ok((input, res))
    }
}

fn lsystem(input: &str) -> IResult<&str, LSystem> {
    let (input, instr) = terminate(instructions)(input)?;
    let mut it = iterator(input, terminate(rule));
    let rules = it.collect();
    let (input, ()) = it.finish()?;

    Ok((
        input,
        LSystem {
            word: instr.clone(),
            axiom: instr,
            rules,
        },
    ))
}

impl LSystem {
    pub fn from_str(input: &str) -> Result<Self, Box<dyn std::error::Error + '_>> {
        let (_, lsystem) = lsystem(input)?;
        Ok(lsystem)
    }

    fn step(&mut self) {
        self.word = self
            .word
            .iter()
            .flat_map(|instr| instr.apply(&self.rules))
            .collect();
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.word = self.axiom.clone();
    }
}

impl Instruction {
    fn apply(&self, rules: &[Rule]) -> Instructions {
        for rule in rules {
            if &rule.0 == self {
                return rule.1.clone();
            }
        }

        use Instruction::*;
        match self {
            Symbol(c) => vec![Symbol(*c)],
            Branch(instrs) => vec![Branch(
                instrs.iter().flat_map(|instr| instr.apply(rules)).collect(),
            )],
        }
    }
}

impl Iterator for LSystem {
    type Item = Instructions;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.word.clone();

        self.step();

        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_instructions() {
        use Instruction::*;
        assert_eq!(
            Ok(("", vec![Symbol('F'), Symbol('G')])),
            simple_instructions("FG")
        )
    }

    #[test]
    fn test_simple_instructions_branch_separated() {
        use Instruction::*;
        assert_eq!(
            Ok(("[FGFGF]", vec![Symbol('F'), Symbol('G')])),
            simple_instructions("FG[FGFGF]")
        )
    }

    #[test]
    fn test_branching() {
        use Instruction::*;
        assert_eq!(
            Ok(("", vec![Branch(vec![Symbol('F'), Symbol('G')])])),
            branch("[FG]")
        )
    }

    #[test]
    fn test_symbols() {
        use Instruction::*;
        assert_eq!(
            Ok((
                "",
                vec![
                    Symbol('F'),
                    Symbol('G'),
                    Branch(vec![Symbol('F'), Symbol('G'), Symbol('F')]),
                    Symbol('F'),
                    Symbol('G')
                ]
            )),
            instructions("FG[FGF]FG")
        )
    }

    #[test]
    fn missing_branckets() {
        assert_eq!(instructions("FGFGHA[DOAIJD").unwrap().0, "[DOAIJD")
    }

    #[test]
    fn too_many_branckets() {
        assert_eq!(instructions("FGFGHA[DOAIJD]]").unwrap().0, "]")
    }

    #[test]
    fn single_rule() {
        use Instruction::*;
        assert_eq!(
            Ok((
                "",
                (Symbol('A'), vec![Symbol('K'), Symbol('J'), Symbol('H')])
            )),
            rule("A->KJH")
        )
    }

    #[test]
    fn rule_whitespace_before() {
        use Instruction::*;
        assert_eq!(
            Ok((
                "",
                (Symbol('A'), vec![Symbol('K'), Symbol('J'), Symbol('H')])
            )),
            rule("  \t\nA->KJH")
        )
    }
}
