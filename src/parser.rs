use crate::error::SantaError;
use crate::function::ParameterList;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct SantaParser;

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,

    Equals,
    NotEquals,
    Greater,
    Less,
    GreaterEquals,
    LessEquals,

    Index,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Negate,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Binary {
        operator: BinaryOperator,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Unary {
        operator: UnaryOperator,
        expr: Box<AstNode>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstNode {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Name(String),
    String(String),
    Function {
        name: Box<AstNode>,
        parameterlist: ParameterList,
        code: Vec<Box<AstNode>>,
    },
    IfStatement {
        condition: Box<AstNode>,
        code: Vec<Box<AstNode>>,
        elsecode: Vec<Box<AstNode>>,
    },
    WhileLoop {
        condition: Box<AstNode>,
        code: Vec<Box<AstNode>>,
    },
    Expression(Operator),
    Assignment {
        name: Box<AstNode>,
        expression: Box<AstNode>,
    },
    Functioncall {
        value: Box<AstNode>,
        args: Vec<Box<AstNode>>,
    },
    Return(Box<AstNode>),
    None,
}

impl AstNode {
    pub fn boxed(self) -> Box<AstNode> {
        Box::new(self)
    }
}

fn integer_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    Ok(AstNode::Integer(
        pair.as_str()
            .parse()
            .map_err(|_| SantaError::ParseTreeError {
                cause: "Couldn't parse to integer".into(),
            })?,
    )
    .boxed())
}

fn float_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    Ok(AstNode::Float(
        pair.as_str()
            .parse()
            .map_err(|_| SantaError::ParseTreeError {
                cause: "Couldn't parse to float".into(),
            })?,
    )
    .boxed())
}

fn boolean_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    Ok(AstNode::Boolean(
        pair.as_str()
            .parse()
            .map_err(|_| SantaError::ParseTreeError {
                cause: "Couldn't parse to boolean".into(),
            })?,
    )
    .boxed())
}

fn name_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    Ok(AstNode::Name(pair.as_str().into()).boxed())
}

fn string_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    Ok(AstNode::String(pair.as_str().replace(&['"', '\"'][..], "").into()).boxed())
}

fn return_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    Ok(Box::new(AstNode::Return(comparison_to_ast(
        pair.into_inner().next().ok_or(SantaError::ParseTreeError {
            cause: "Couldn't parse to integer".into(),
        })?,
    )?)))
}

fn ifstatement_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let mut inner_pair = pair.into_inner();

    let condition = comparison_to_ast(inner_pair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?)?;

    let code_block = block_to_ast(inner_pair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?)?;

    let else_block = block_to_ast(inner_pair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?)?;

    Ok(Box::new(AstNode::IfStatement {
        condition,
        code: code_block,
        elsecode: else_block,
    }))
}

fn whileloop_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let mut inner_pair = pair.into_inner();

    let condition = comparison_to_ast(inner_pair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?)?;

    let code_block = block_to_ast(inner_pair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?)?;

    Ok(Box::new(AstNode::WhileLoop {
        condition,
        code: code_block,
    }))
}

fn function_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let mut inner_pair = pair.into_inner();

    let possible_name = inner_pair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?;

    let (name, possible_parameterlist) = if possible_name.as_rule() == Rule::name {
        let next = inner_pair.next().ok_or(SantaError::ParseTreeError {
            cause: "Couldn't parse".into(),
        })?;
        (name_to_ast(possible_name)?, next)
    } else {
        (Box::new(AstNode::None), possible_name)
    };

    let (parameterlist, block) = if possible_parameterlist.as_rule() == Rule::parameterlist {
        let next = inner_pair.next().ok_or(SantaError::ParseTreeError {
            cause: "Couldn't parse".into(),
        })?;
        (parameterlist_to_ast(possible_parameterlist)?, next)
    } else {
        (vec![], possible_parameterlist)
    };

    let mut parameters = Vec::new();

    for i in parameterlist {
        // can *only* be `name` nodes meaning that they can be
        // evaluated directly into strings.
        if let AstNode::Name(name) = *i {
            parameters.push(name);
        } else {
            return Err(SantaError::ParseTreeError {
                cause: "Couldn't parse parameter".into(),
            });
        }
    }

    let real_parameterlist = ParameterList::new(parameters);

    let function_block = block_to_ast(block)?;

    Ok(Box::new(AstNode::Function {
        name,
        parameterlist: real_parameterlist,
        code: function_block,
    }))
}

fn atom_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let r = pair.as_rule();
    match r {
        Rule::ifstatement => ifstatement_to_ast(pair),
        Rule::name => name_to_ast(pair),
        Rule::integer => integer_to_ast(pair),
        Rule::boolean => boolean_to_ast(pair),
        Rule::float => float_to_ast(pair),
        Rule::string => string_to_ast(pair),
        Rule::comparison => comparison_to_ast(pair),
        Rule::function => function_to_ast(pair),
        _ => Err(SantaError::ParseTreeError {
            cause: "Not implemented".into(),
        }),
    }
}

fn parameterlist_to_ast(pair: Pair<Rule>) -> Result<Vec<Box<AstNode>>, SantaError> {
    let mut result = vec![];

    for param in pair.into_inner() {
        result.push(name_to_ast(param)?)
    }

    Ok(result)
}

fn argumentlist_to_ast(pair: Option<Pair<Rule>>) -> Result<Vec<Box<AstNode>>, SantaError> {
    if let Some(pair) = pair {
        let mut result = vec![];

        let innerpair = pair.into_inner();
        for arg in innerpair {
            result.push(comparison_to_ast(arg)?)
        }

        Ok(result)
    } else {
        Ok(vec![])
    }
}



fn index_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    comparison_to_ast(pair)
}

fn atomexpr_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let mut innerpair = pair.into_inner();
    let atom = atom_to_ast(innerpair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?);

    match innerpair.next() {
        Some(i) => match i.as_rule() {
            Rule::functioncall => Ok(Box::new(AstNode::Functioncall {
                value: atom?,
                args: argumentlist_to_ast(i.into_inner().next())?,
            })),
            Rule::index => Ok(Box::new(AstNode::Expression(
                Operator::Binary {
                    operator: BinaryOperator::Index,
                    lhs: atom?,
                    rhs: index_to_ast(i.into_inner().next().ok_or(SantaError::ParseTreeError {
                        cause: "Couldn't parse".into(),
                    })?)?
                }
            ))),
            _ => atom,
        },
        _ => atom,
    }
}

fn factor_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let mut inner_pair = pair.into_inner();
    let next = inner_pair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?;

    let strnxt = next.as_str();
    Ok(match strnxt {
        "-" => AstNode::Expression(Operator::Unary {
            operator: UnaryOperator::Negate,
            expr: factor_to_ast(inner_pair.next().ok_or(SantaError::ParseTreeError {
                cause: "Couldn't parse".into(),
            })?)?,
        })
        .boxed(),
        _ => atomexpr_to_ast(next)?,
    })
}

fn term_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let mut inner_pair = pair.into_inner();

    let mut result = factor_to_ast(inner_pair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?)?;

    while let Some(operator) = inner_pair.next() {
        let curr = inner_pair.next().ok_or(SantaError::ParseTreeError {
            cause: "Couldn't parse".into(),
        })?;

        let new_result = AstNode::Expression(Operator::Binary {
            operator: match operator.as_str() {
                "*" => BinaryOperator::Multiply,
                "/" => BinaryOperator::Divide,
                _ => {
                    return Err(SantaError::ParseTreeError {
                        cause: "Invalid operator".into(),
                    })
                }
            },
            lhs: result,
            rhs: factor_to_ast(curr)?,
        })
        .boxed();
        result = new_result;
    }

    Ok(result)
}

fn expression_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let mut inner_pair = pair.into_inner();

    let mut result = term_to_ast(inner_pair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?)?;

    while let Some(operator) = inner_pair.next() {
        let curr = inner_pair.next().ok_or(SantaError::ParseTreeError {
            cause: "Couldn't parse".into(),
        })?;

        let new_result = AstNode::Expression(Operator::Binary {
            operator: match operator.as_str() {
                "+" => BinaryOperator::Add,
                "-" => BinaryOperator::Subtract,
                _ => {
                    return Err(SantaError::ParseTreeError {
                        cause: "Invalid operator".into(),
                    })
                }
            },
            lhs: result,
            rhs: term_to_ast(curr)?,
        })
        .boxed();
        result = new_result;
    }

    Ok(result.boxed())
}

fn comparison_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let mut inner_pair = pair.into_inner();
    let expr = expression_to_ast(inner_pair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?)?;

    if let Some(operator) = inner_pair.next() {
        let binop = match operator.as_str() {
            "==" => BinaryOperator::Equals,
            "!=" => BinaryOperator::NotEquals,
            ">=" => BinaryOperator::GreaterEquals,
            "<=" => BinaryOperator::LessEquals,
            ">" => BinaryOperator::Greater,
            "<" => BinaryOperator::Less,
            _ => {
                return Err(SantaError::ParseTreeError {
                    cause: "Couldn't parse".into(),
                })
            }
        };

        return Ok(Box::new(AstNode::Expression(Operator::Binary {
            operator: binop,
            lhs: expr,
            rhs: expression_to_ast(inner_pair.next().ok_or(SantaError::ParseTreeError {
                cause: "Couldn't parse".into(),
            })?)?,
        })));
    } else {
        Ok(expr)
    }
}

fn statement_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let r = pair.as_rule();
    match r {
        Rule::assignment => {
            let mut pair = pair.into_inner();
            let name = pair.next().ok_or(SantaError::ParseTreeError {
                cause: "Couldn't parse".into(),
            })?;

            let expression = pair.next().ok_or(SantaError::ParseTreeError {
                cause: "Couldn't parse".into(),
            })?;

            Ok(AstNode::Assignment {
                name: name_to_ast(name)?,
                expression: comparison_to_ast(expression)?,
            }
            .boxed())
        }
        Rule::comparison => comparison_to_ast(pair),
        Rule::function => function_to_ast(pair),
        Rule::ifstatement => ifstatement_to_ast(pair),
        Rule::whileloop => whileloop_to_ast(pair),
        Rule::returnstatement => return_to_ast(pair),
        _ => Err(SantaError::ParseTreeError {
            cause: "Not implemented".into(),
        }),
    }
}

fn block_to_ast(pairs: Pair<Rule>) -> Result<Vec<Box<AstNode>>, SantaError> {
    file_to_ast(pairs)
}

fn file_to_ast(pairs: Pair<Rule>) -> Result<Vec<Box<AstNode>>, SantaError> {
    let mut ast = vec![];

    for pair in pairs.into_inner() {
        let r = pair.as_rule();

        match r {
            Rule::statement => {
                ast.push(statement_to_ast(pair.into_inner().next().ok_or(
                    SantaError::ParseTreeError {
                        cause: "Couldn't parse".into(),
                    },
                )?)?);
            }
            Rule::EOI => break,
            _ => {
                return Err(SantaError::ParseTreeError {
                    cause: "Not implemented".into(),
                })
            }
        }
    }

    Ok(ast)
}

pub fn parse_string(input: &str) -> Result<Vec<Box<AstNode>>, SantaError> {
    let mut pairs = SantaParser::parse(Rule::file, input).map_err(|e| SantaError::ParseError {
        cause: format!("{}", e),
    })?;

    let first_pair = pairs.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?;

    match first_pair.as_rule() {
        Rule::file => file_to_ast(first_pair),
        _ => Err(SantaError::ParseTreeError {
            cause: "Couldn't parse".into(),
        }),
    }
}

pub fn parse_string_or_panic(input: &str) -> Vec<Box<AstNode>> {
    match parse_string(input) {
        Ok(i) => i,
        Err(e) => {
            match e {
                SantaError::ParseError { cause } => {
                    println!("{}", cause);
                }
                _ => println!("{}", e),
            };

            panic!()
        }
    }
}
