use function_ir::Node;
use parser_error::{Expected, ParseError, ParseResult};

type BNode = Box<Node>;

fn check_index<'a>(input: &'a [char], current_index: usize) -> ParseResult<()> {
    if current_index >= input.len() {
        Err(ParseError::UnexpectedEnd)
    } else {
        Ok(())
    }
}

fn try_incr_index<'a>(input: &'a [char], current_index: usize) -> ParseResult<usize> {
    let mut index = current_index + 1;
    check_index(&input, index)?;

    while input[index] == ' ' {
        index += 1;
        check_index(&input, index)?;
    }

    Ok(index)
}

fn incr_index<'a>(input: &'a [char], current_index: usize) -> usize {
    let mut index = current_index + 1;

    while index < input.len() && input[index] == ' ' {
        index += 1;
    }

    index
}

pub fn parse_expression<'a>(input: &'a [char], current_index: usize) -> ParseResult<BNode> {
    let (root, index) = parse_add(&input, current_index)?;

    if index < input.len() {
        Err(ParseError::UnconsumedInput(index))
    } else {
        Ok(root)
    }
}

fn parse_add<'a>(input: &'a [char], current_index: usize) -> ParseResult<(BNode, usize)> {
    let (mut base, mut index) = parse_mul(&input, current_index)?;

    while index < input.len() && (input[index] == '-' || input[index] == '+') {
        let op_index = index;
        index = try_incr_index(&input, index)?;
        let (term, new_index) = parse_mul(&input, index)?;

        if input[op_index] == '-' {
            base = Box::new(Node::Sub(base, term));
        } else {
            base = Box::new(Node::Add(base, term));
        }

        index = new_index;
    }

    Ok((base, index))
}

fn parse_mul<'a>(input: &'a [char], current_index: usize) -> ParseResult<(BNode, usize)> {
    let (mut base, mut index) = parse_exp(&input, current_index)?;

    while index < input.len() && (input[index] == '*' || input[index] == '/') {
        let op_index = index;
        index = try_incr_index(&input, index)?;
        let (term, new_index) = parse_exp(&input, index)?;

        if input[op_index] == '*' {
            base = Box::new(Node::Mul(base, term));
        } else {
            base = Box::new(Node::Div(base, term));
        }

        index = new_index;
    }

    Ok((base, index))
}

fn parse_exp<'a>(input: &'a [char], current_index: usize) -> ParseResult<(BNode, usize)> {
    let (base, mut index) = parse_primary(&input, current_index)?;

    if index < input.len() && input[index] == '^' {
        index = try_incr_index(&input, index)?;

        let (exp, index) = parse_primary(&input, index)?;
        let exp_node = Box::new(Node::Exp(base, exp));

        Ok((exp_node, index))
    } else {
        Ok((base, index))
    }
}

fn parse_primary<'a>(input: &'a [char], current_index: usize) -> ParseResult<(BNode, usize)> {
    let mut index = current_index;

    if input[index] == '(' {
        index = try_incr_index(&input, index)?;

        // TODO replace base with expression
        let (base, mut index) = parse_add(&input, index)?;
        check_index(&input, index)?;

        if input[index] == ')' {
            index = incr_index(&input, index);
            Ok((base, index))
        } else {
            Err(ParseError::UnexpectedChar {
                pos: index,
                c: input[index],
                exp: Expected::Char(')'),
            })
        }
    } else {
        parse_base(&input, index)
    }
}

fn parse_base<'a>(input: &'a [char], current_index: usize) -> ParseResult<(BNode, usize)> {
    let mut index = current_index;
    check_index(&input, index)?;

    // TODO negative needs to also bind to primary, and bind less strognly than
    // exponentiation to
    // avoid uneccassary confustion
    let mut negative = false;
    if input[index] == '-' {
        negative = true;
        index = try_incr_index(&input, index)?;
    }

    let (base, new_index) = match input[index] {
        'x' | 'y' | 'z' => {
            let node = Node::Variable(input[index]);
            index = incr_index(&input, index);
            (Box::new(node), index)
        }
        d if d.is_digit(10) => parse_number(input, index)?,
        c => {
            return Err(ParseError::UnexpectedChar {
                c: c,
                pos: index,
                exp: Expected::Base,
            })
        }
    };

    let result_node;
    if negative {
        let constant = Box::new(Node::Constant(-1.0));
        result_node = Box::new(Node::Mul(constant, base));
    } else {
        result_node = base;
    }

    Ok((result_node, new_index))
}

fn parse_number<'a>(input: &'a [char], current_index: usize) -> ParseResult<(BNode, usize)> {
    let mut index = current_index;
    if !input[index].is_digit(10) {
        return Err(ParseError::UnexpectedChar {
            c: input[index],
            pos: index,
            exp: Expected::Constant,
        });
    }

    let mut buffer = String::new();
    let mut found_decimal = false;
    while input[index].is_digit(10) {
        buffer.push(input[index]);
        index += 1;

        if index >= input.len() {
            break;
        }

        if (!found_decimal) && input[index] == '.' {
            buffer.push(input[index]);
            index += 1;
            found_decimal = true;
        }
    }

    let constant = buffer.parse()?;
    let bnode = Box::new(Node::Constant(constant));
    Ok((bnode, incr_index(&input, index - 1)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn assert_constant(result: &Node, expected: f32) {
        if let &Node::Constant(ref c) = result {
            assert_similiar!(c, expected);
        } else {
            panic!(format!("Expected constant, found {:?}", result))
        }
    }

    fn assert_variable(result: &Node, expected: char) {
        if let &Node::Variable(c) = result {
            assert_eq!(c, expected);
        } else {
            panic!(format!("Expected variable, found {:?}", result))
        }
    }

    #[test]
    fn test_parse_number() {
        let mut input: Vec<char>;
        let mut result;

        input = "123".chars().collect();
        result = parse_number(&input, 0).unwrap();
        assert_constant(&result.0, 123f32);

        input = "123.1232".chars().collect();
        result = parse_number(&input, 0).unwrap();
        assert_constant(&result.0, 123.1232);

        input = "0.1010110".chars().collect();
        result = parse_number(&input, 0).unwrap();
        assert_constant(&result.0, 0.1010110);

        input = "0.101a0110".chars().collect();
        result = parse_number(&input, 0).unwrap();
        assert_constant(&result.0, 0.101);
        assert_eq!(result.1, 5);

        input = "0.a101a0110".chars().collect();
        result = parse_number(&input, 0).unwrap();
        assert_constant(&result.0, 0.0);
        assert_eq!(result.1, 2);

        input = " 1 + 2 + 34 + 456543.23 + 0.101a0110".chars().collect();
        result = parse_number(&input, 9).unwrap();
        assert_constant(&result.0, 34.0);
        assert_eq!(result.1, 12);

        input = "b0.a101a0110".chars().collect();
        let err = parse_number(&input, 0).unwrap_err();
        assert_eq!(format!("{}", err), "Looking for Constant, found b at 0");
    }

    #[test]
    fn test_parse_base() {
        let mut input: Vec<char>;
        let mut result;

        input = "x".chars().collect();
        result = parse_base(&input, 0).unwrap();
        assert_variable(&result.0, 'x');

        input = "1 + z / y".chars().collect();
        result = parse_base(&input, 4).unwrap();
        assert_variable(&result.0, 'z');
        assert_eq!(result.1, 6);

        input = "0.0 + x * 9".chars().collect();
        result = parse_base(&input, 10).unwrap();
        assert_constant(&result.0, 9.0);
        assert_eq!(result.1, 11);

        input = "-2.0".chars().collect();
        result = parse_base(&input, 0).unwrap();
        assert_eq!(
            format!("{:?}", result.0),
            "Mul(Constant(-1.0), Constant(2.0))"
        );

        input = "-".chars().collect();
        let err = parse_base(&input, 0).unwrap_err();
        assert_eq!(format!("{}", err), "Unexpected end of input");
    }

    #[test]
    fn test_parse_primary() {
        let mut input: Vec<char>;
        let mut result;

        input = "x".chars().collect();
        result = parse_primary(&input, 0).unwrap();
        assert_variable(&result.0, 'x');

        input = "0.0 + x * 9".chars().collect();
        result = parse_primary(&input, 10).unwrap();
        assert_constant(&result.0, 9.0);
        assert_eq!(result.1, 11);

        input = "(-2.0)".chars().collect();
        result = parse_primary(&input, 0).unwrap();
        assert_eq!(
            format!("{:?}", result.0),
            "Mul(Constant(-1.0), Constant(2.0))"
        );

        input = "1 + 2 * (0.131) + 4".chars().collect();
        result = parse_primary(&input, 8).unwrap();
        assert_constant(&result.0, 0.131);
        assert_eq!(result.1, 16);

        input = "(0.131.".chars().collect();
        let err = parse_primary(&input, 0).unwrap_err();
        assert_eq!(format!("{}", err), "Looking for character: ), found . at 6");

        input = "(0.131".chars().collect();
        let err = parse_primary(&input, 0).unwrap_err();
        assert_eq!(format!("{}", err), "Unexpected end of input");
    }

    #[test]
    fn test_parse_exp() {
        let mut input: Vec<char>;
        let mut result;

        input = "x".chars().collect();
        result = parse_exp(&input, 0).unwrap();
        assert_variable(&result.0, 'x');

        input = "0.0 + x * 9".chars().collect();
        result = parse_exp(&input, 10).unwrap();
        assert_constant(&result.0, 9.0);
        assert_eq!(result.1, 11);

        input = "(-2.0)".chars().collect();
        result = parse_exp(&input, 0).unwrap();
        assert_eq!(
            format!("{:?}", result.0),
            "Mul(Constant(-1.0), Constant(2.0))"
        );

        input = "0.131^x".chars().collect();
        result = parse_exp(&input, 0).unwrap();
        assert_eq!(
            format!("{:?}", result.0),
            "Exp(Constant(0.131), Variable(\'x\'))"
        );

        input = "(0.131)^(-1.2332)".chars().collect();
        result = parse_exp(&input, 0).unwrap();
        assert_eq!(
            format!("{:?}", result.0),
            "Exp(Constant(0.131), Mul(Constant(-1.0), Constant(1.2332)))"
        );

        input = "(0.131)^".chars().collect();
        let err = parse_exp(&input, 0).unwrap_err();
        assert_eq!(format!("{}", err), "Unexpected end of input");

        input = "((1.0)^(1.0))^x".chars().collect();
        result = parse_exp(&input, 0).unwrap();
        assert_eq!(
            format!("{:?}", result.0),
            "Exp(Exp(Constant(1.0), Constant(1.0)), Variable(\'x\'))"
        );
    }

    #[test]
    fn test_parse_mul() {
        let mut input: Vec<char>;
        let mut result;

        input = "x".chars().collect();
        result = parse_mul(&input, 0).unwrap();
        assert_variable(&result.0, 'x');

        input = "1.0 * 1.0 * 1.0".chars().collect();
        result = parse_mul(&input, 0).unwrap();
        assert_eq!(
            format!("{:?}", result.0),
            "Mul(Mul(Constant(1.0), Constant(1.0)), Constant(1.0))"
        );

        input = "1.0 * 1.0 * 1.0^x".chars().collect();
        result = parse_mul(&input, 0).unwrap();
        assert_eq!(
            format!("{:?}", result.0),
            "Mul(Mul(Constant(1.0), Constant(1.0)), Exp(Constant(1.0), Variable(\'x\')))"
        );

        input = "(0.131)^(-1.0) * -2.0 * 4.3".chars().collect();
        result = parse_mul(&input, 0).unwrap();
        assert_eq!(
            format!("{:?}", result.0),
            "Mul(Mul(Exp(Constant(0.131), Mul(Constant(-1.0), Constant(1.0))), Mul(Constant(-1.0), Constant(2.0))), Constant(4.3))"
        );
    }

    #[test]
    fn test_parse_add() {
        let mut input: Vec<char>;
        let mut result;

        input = "x".chars().collect();
        result = parse_add(&input, 0).unwrap();
        assert_variable(&result.0, 'x');

        input = "1.0 + 2.0 * 3.0 - 4.0".chars().collect();
        result = parse_add(&input, 0).unwrap();
        assert_eq!(
            format!("{:?}", result.0),
            "Sub(Add(Constant(1.0), Mul(Constant(2.0), Constant(3.0))), Constant(4.0))"
        );
    }

    // TODO add more parse_expression tests now that the old ones moved to
    // function_ir
}
