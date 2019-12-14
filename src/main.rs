#![allow(clippy::vec_box)]
#![allow(clippy::boxed_local)]
#![allow(clippy::ptr_arg)]

use crate::parser::parse_string_or_panic;
use clap::{App, Arg, SubCommand, AppSettings};
use std::fs;
use crate::manual::{run_manual, get_manual_id};

mod error;
mod eval;
mod function;
mod object;
mod parser;
mod builtins;
mod manual;

fn main() {
    get_manual_id();

    let matches = App::new("Santa Programming Language")
        .version("1.0.0")
        .author("Santa <noreply@santa.northpole>")
        .about("The santa programming system \n\nThis system is used internally at the north \
            pole Santa base for the indexing of the naughty and nice \
            list.")
        .setting(AppSettings::ArgRequiredElseHelp)

        .subcommand(
            SubCommand::with_name("run")
                .about("Run a santa file")
                .arg(Arg::with_name("filename").required(true))
        )

        .subcommand(
            SubCommand::with_name("manual")
                .about("Print the manual")
        )

        .get_matches();

    match matches.subcommand() {
        ("run", Some(matches)) => {
            let filename = matches
                .value_of("filename")
                .expect("Santa couldn't read your filename!");
            let file = fs::read_to_string(filename).expect("Santa's elves couldn't find your file!");

            let ast = parse_string_or_panic(&file);

            eval::eval(ast);
        }
        ("manual", Some(matches)) => {
            run_manual()
        },
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::{eval_node, eval_with_scope, Scope};
    use crate::function::{Function, ParameterList};
    use crate::object::Object;
    use crate::parser::AstNode::{Assignment, Expression, Integer, Name};
    use crate::parser::{parse_string_or_panic, BinaryOperator, Operator, UnaryOperator};
    use crate::manual::run_manual;

    #[test]
    fn test_simple_1() {
        let ast = parse_string_or_panic("a = 3;");

        assert_eq!(
            ast,
            vec![Box::new(Assignment {
                name: Box::new(Name("a".into())),
                expression: Box::new(Integer(3))
            })]
        );

        let mut scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap(), &mut scope).unwrap(),
            Object::Integer(3)
        );

        assert_eq!(scope.get_variable(&"a".into()), Some(Object::Integer(3)));
    }

    #[test]
    fn test_addition_1() {
        let ast = parse_string_or_panic("a = 3 + 5;");

        assert_eq!(
            ast,
            vec![Box::new(Assignment {
                name: Box::new(Name("a".into())),
                expression: Box::new(Expression(Operator::Binary {
                    operator: BinaryOperator::Add,
                    lhs: Box::new(Integer(3)),
                    rhs: Box::new(Integer(5)),
                }))
            })]
        );

        let mut scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap(), &mut scope).unwrap(),
            Object::Integer(8)
        );

        assert_eq!(scope.get_variable(&"a".into()), Some(Object::Integer(8)));
    }

    #[test]
    fn test_subtraction_1() {
        let ast = parse_string_or_panic("a = 3 - 5;");

        assert_eq!(
            ast,
            vec![Box::new(Assignment {
                name: Box::new(Name("a".into())),
                expression: Box::new(Expression(Operator::Binary {
                    operator: BinaryOperator::Subtract,
                    lhs: Box::new(Integer(3)),
                    rhs: Box::new(Integer(5)),
                }))
            })]
        );

        let mut scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap(), &mut scope).unwrap(),
            Object::Integer(-2)
        );

        assert_eq!(scope.get_variable(&"a".into()), Some(Object::Integer(-2)));
    }

    #[test]
    fn test_multiplication_1() {
        let ast = parse_string_or_panic("a = 3 * 5;");

        assert_eq!(
            ast,
            vec![Box::new(Assignment {
                name: Box::new(Name("a".into())),
                expression: Box::new(Expression(Operator::Binary {
                    operator: BinaryOperator::Multiply,
                    lhs: Box::new(Integer(3)),
                    rhs: Box::new(Integer(5)),
                }))
            })]
        );

        let mut scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap(), &mut scope).unwrap(),
            Object::Integer(15)
        );

        assert_eq!(scope.get_variable(&"a".into()), Some(Object::Integer(15)));
    }

    #[test]
    fn test_division_1() {
        let ast = parse_string_or_panic("a = 3 / 5;");

        assert_eq!(
            ast,
            vec![Box::new(Assignment {
                name: Box::new(Name("a".into())),
                expression: Box::new(Expression(Operator::Binary {
                    operator: BinaryOperator::Divide,
                    lhs: Box::new(Integer(3)),
                    rhs: Box::new(Integer(5)),
                }))
            })]
        );

        let mut scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap(), &mut scope).unwrap(),
            Object::Float(3.0 / 5.0)
        );

        assert_eq!(
            scope.get_variable(&"a".into()),
            Some(Object::Float(3.0 / 5.0))
        );
    }

    #[test]
    fn test_negation_1() {
        let ast = parse_string_or_panic("a = -3;");

        assert_eq!(
            ast,
            vec![Box::new(Assignment {
                name: Box::new(Name("a".into())),
                expression: Box::new(Expression(Operator::Unary {
                    operator: UnaryOperator::Negate,
                    expr: Box::new(Integer(3)),
                }))
            })]
        );

        let mut scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap(), &mut scope).unwrap(),
            Object::Integer(-3)
        );

        assert_eq!(scope.get_variable(&"a".into()), Some(Object::Integer(-3)));
    }

    #[test]
    fn test_operator_precedence_1() {
        let ast = parse_string_or_panic("a = 3 + 4 / 2;");

        assert_eq!(
            ast,
            vec![Box::new(Assignment {
                name: Box::new(Name("a".into())),
                expression: Box::new(Expression(Operator::Binary {
                    operator: BinaryOperator::Add,
                    lhs: Box::new(Integer(3)),
                    rhs: Box::new(Expression(Operator::Binary {
                        operator: BinaryOperator::Divide,
                        lhs: Box::new(Integer(4)),
                        rhs: Box::new(Integer(2)),
                    })),
                }))
            })]
        );

        let mut scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap(), &mut scope).unwrap(),
            Object::Float(5.0)
        );

        assert_eq!(scope.get_variable(&"a".into()), Some(Object::Float(5.0)));
    }

    #[test]
    fn test_operator_precedence_2() {
        let ast = parse_string_or_panic("a = (3 + 4) / 2;");

        assert_eq!(
            ast,
            vec![Box::new(Assignment {
                name: Box::new(Name("a".into())),
                expression: Box::new(Expression(Operator::Binary {
                    operator: BinaryOperator::Divide,
                    rhs: Box::new(Integer(2)),
                    lhs: Box::new(Expression(Operator::Binary {
                        operator: BinaryOperator::Add,
                        lhs: Box::new(Integer(3)),
                        rhs: Box::new(Integer(4)),
                    })),
                }))
            })]
        );

        let mut scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap(), &mut scope).unwrap(),
            Object::Float(7.0 / 2.0)
        );

        assert_eq!(
            scope.get_variable(&"a".into()),
            Some(Object::Float(7.0 / 2.0))
        );
    }

    #[test]
    fn test_multiline_1() {
        let ast = parse_string_or_panic("a = 3 + 2;b = a;");

        let mut scope = Scope::new();

        assert_eq!(ast.len(), 2);

        for node in ast {
            assert_eq!(eval_node(node, &mut scope).unwrap(), Object::Integer(5));
        }

        assert_eq!(scope.get_variable(&"a".into()), Some(Object::Integer(5)));

        assert_eq!(scope.get_variable(&"b".into()), Some(Object::Integer(5)));
    }

    #[test]
    fn test_multiline_2() {
        let ast = parse_string_or_panic("a = 3 + 2;b = a + 2;");

        let mut scope = Scope::new();

        assert_eq!(ast.len(), 2);

        let mut nodes = ast.into_iter();
        assert_eq!(
            eval_node(nodes.next().unwrap(), &mut scope).unwrap(),
            Object::Integer(5)
        );
        assert_eq!(
            eval_node(nodes.next().unwrap(), &mut scope).unwrap(),
            Object::Integer(7)
        );

        assert_eq!(scope.get_variable(&"a".into()), Some(Object::Integer(5)));

        assert_eq!(scope.get_variable(&"b".into()), Some(Object::Integer(7)));
    }

    #[test]
    fn test_functioncall_1() {
        let ast = parse_string_or_panic("b = a();");

        let mut scope = Scope::new();

        assert_eq!(ast.len(), 1);

        let mut nodes = ast.into_iter();

        // create a function called a
        scope.set_variable(
            "a".into(),
            Object::Function(Function::Builtin(ParameterList::new(vec![]), |scope| {
                Object::Integer(10)
            })),
        );

        assert_eq!(
            eval_node(nodes.next().unwrap(), &mut scope).unwrap(),
            Object::Integer(10)
        );

        assert_eq!(scope.get_variable(&"b".into()), Some(Object::Integer(10)));
    }

    #[test]
    fn test_functioncall_2() {
        let ast = parse_string_or_panic("b = a(3);");

        let mut scope = Scope::new();

        assert_eq!(ast.len(), 1);

        let mut nodes = ast.into_iter();

        // create a function called a
        scope.set_variable(
            "a".into(),
            Object::Function(Function::Builtin(
                ParameterList::new(vec!["x".into()]),
                |scope| match scope.get_variable(&"x".into()) {
                    Some(i) => i,
                    None => Object::None,
                },
            )),
        );

        assert_eq!(
            eval_node(nodes.next().unwrap(), &mut scope).unwrap(),
            Object::Integer(3)
        );

        assert_eq!(scope.get_variable(&"b".into()), Some(Object::Integer(3)));
    }

    #[test]
    fn test_function_1() {
        let ast = parse_string_or_panic("function a () {}");

        let mut scope = Scope::new();

        assert_eq!(ast.len(), 1);

        let mut nodes = ast.into_iter();

        assert_eq!(
            eval_node(nodes.next().unwrap(), &mut scope).unwrap(),
            Object::Function(Function::User(ParameterList::new(vec![]), vec![]))
        );

        assert_eq!(
            scope.get_variable(&"a".into()),
            Some(Object::Function(Function::User(
                ParameterList::new(vec![]),
                vec![]
            )))
        );
    }

    #[test]
    fn test_function_2() {
        let ast = parse_string_or_panic(
            "
function a (x) {
    return x + 1;
}",
        );

        let mut scope = Scope::new();

        assert_eq!(ast.len(), 1);

        eval_with_scope(ast, &mut scope);

        match scope.get_variable(&"a".into()) {
            Some(Object::Function(Function::User(x, _))) => {
                assert_eq!(x.positional, vec![String::from("x")]);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_function_3() {
        let ast = parse_string_or_panic(
            "
function a (x) {
    return x + 1;
}

a(3);
",
        );

        let mut scope = Scope::new();

        assert_eq!(ast.len(), 2);

        assert_eq!(eval_with_scope(ast, &mut scope), Object::Integer(4));

        match scope.get_variable(&"a".into()) {
            Some(Object::Function(Function::User(x, _))) => {
                assert_eq!(x.positional, vec![String::from("x")]);
            }
            _ => panic!(),
        }
    }

    #[test]
    #[ignore]
    fn test_print_1() {
        let ast = parse_string_or_panic(
            "
a = 3;
b = 4;
x = a + b;

print(x);
",
        );

        let mut scope = Scope::new();
        eval_with_scope(ast, &mut scope);
    }



    #[test]
    #[ignore]
    fn test_manual() {
        run_manual();
    }
}
