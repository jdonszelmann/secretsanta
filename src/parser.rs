use crate::error::SantaError;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive;
use crate::function::ParameterList;
use crate::eval::{eval_node, Scope};
use crate::object::Object;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct SantaParser;

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
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
    Name(String),
    String(String),
    Function {
        name: Box<AstNode>,
        parameterlist: ParameterList,
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
    None,
}

impl AstNode{
    pub fn boxed(self) -> Box<AstNode> {
        Box::new(self)
    }
}

fn integer_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    Ok(AstNode::Integer(
        pair.as_str()
            .parse()
            .map_err(|_| SantaError::ParseTreeError {
                cause: "Couldn't parse to integer".into()
            })?).boxed()
    )
}

fn name_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    Ok(AstNode::Name(pair.as_str().into()).boxed())
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

    for i in parameterlist{
        // can *only* be `name` nodes meaning that they can be
        // evaluated directly into strings.
        if let AstNode::Name(name) = *i{
            parameters.push(name);
        } else {
            return Err(SantaError::ParseTreeError {
                cause: "Couldn't parse parameter".into(),
            });
        }
    }

    let real_parameterlist = ParameterList::new(parameters);

    let function_block = file_to_ast(block)?;

    Ok(Box::new(AstNode::Function {
        name,
        parameterlist: real_parameterlist,
        code: function_block,
    }))
}

fn atom_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let r = pair.as_rule();
    match r {
        Rule::name => name_to_ast(pair),
        Rule::integer => integer_to_ast(pair),
        Rule::expr => expression_to_ast(pair),
        Rule::function => function_to_ast(pair),
        _ => Err(SantaError::ParseTreeError {
            cause: "Not implemented".into(),
        }),
    }
}

fn parameterlist_to_ast(pair: Pair<Rule>) -> Result<Vec<Box<AstNode>>, SantaError> {
    let mut result = vec![];

    let mut innerpair = pair.into_inner();
    while let Some(arg) = innerpair.next() {
        result.push(name_to_ast(arg)?)
    }

    Ok(result)
}

fn argumentlist_to_ast(pair: Option<Pair<Rule>>) -> Result<Vec<Box<AstNode>>, SantaError> {
    if let Some(pair) = pair {
        let mut result = vec![];

        let mut innerpair = pair.into_inner();
        while let Some(arg) = innerpair.next() {
            result.push(expression_to_ast(arg)?)
        }

        Ok(result)
    } else {
        Ok(vec![])
    }
}

fn atomexpr_to_ast(pair: Pair<Rule>) -> Result<Box<AstNode>, SantaError> {
    let mut innerpair = pair.into_inner();
    let atom = atom_to_ast(innerpair.next().ok_or(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    })?);


    match innerpair.next() {
        Some(i) => match i.as_rule() {
            Rule::functioncall => {
                Ok(Box::new(AstNode::Functioncall {
                    value: atom?,
                    args: argumentlist_to_ast(i.into_inner().next())?,
                }))
            }
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
        "-" => {
            AstNode::Expression(Operator::Unary {
              operator: UnaryOperator::Negate,
                expr: factor_to_ast(inner_pair.next().ok_or(SantaError::ParseTreeError {
                    cause: "Couldn't parse".into(),
                })?)?,
            }).boxed()
        }
        _ => {
            atomexpr_to_ast(next)?
        }
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
                _ => return Err(SantaError::ParseTreeError {
                    cause: "Invalid operator".into(),
                })
            },
            lhs: result,
            rhs: factor_to_ast(curr)?
        }).boxed();
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
               _ => return Err(SantaError::ParseTreeError {
                   cause: "Invalid operator".into(),
               })
           },
            lhs: result,
            rhs: term_to_ast(curr)?
        }).boxed();
        result = new_result;
    }

    Ok(result.boxed())
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
                expression: expression_to_ast(expression)?,
            }.boxed())
        },
        Rule::function => {
            function_to_ast(pair)
        },
        _ => Err(SantaError::ParseTreeError {
            cause: "Not implemented".into(),
        }),
    }
}

fn file_to_ast(pairs: Pair<Rule>) -> Result<Vec<Box<AstNode>>, SantaError> {
    let mut ast = vec![];

    for pair in pairs.into_inner() {
        let r = pair.as_rule();

        match r {
            Rule::statement => {
                ast.push(statement_to_ast(pair.into_inner().next().ok_or(SantaError::ParseTreeError {
                    cause: "Couldn't parse".into(),
                })?)?);
            },
            Rule::EOI => break,
            _ => return Err(SantaError::ParseTreeError {
                cause: "Not implemented".into(),
            }),
        }
    }

    Ok(ast)
}

pub fn parse_string(input: &str) -> Result<Vec<Box<AstNode>>, SantaError> {
    let pairs = SantaParser::parse(Rule::file, input).map_err(|e| SantaError::ParseError {
        cause: format!("{}", e),
    })?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::file => {
                return file_to_ast(pair);
            }
            _ => {
                return Err(SantaError::ParseTreeError {
                    cause: "Couldn't parse".into(),
                });
            }
        }
    }

    return Err(SantaError::ParseTreeError {
        cause: "Couldn't parse".into(),
    });
}


pub fn parse_string_or_panic(input: &str) -> Vec<Box<AstNode>> {
    match parse_string(input) {
        Ok(i) => i,
        Err(e) => {
            match e {
                SantaError::ParseError {cause} => {
                    println!("{}", cause);
                }
                _ => println!("{}", e)
            };

            panic!()
        }
    }
}