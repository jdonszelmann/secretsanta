#![allow(clippy::vec_box)]
#![allow(clippy::boxed_local)]
#![allow(clippy::ptr_arg)]

use crate::manual::{get_manual_id, run_manual, set_manual_id, MANUAL_ID};
use crate::parser::parse_string_or_panic;
use clap::{App, AppSettings, Arg, SubCommand};
use std::fs;

mod builtins;
mod error;
mod eval;
mod function;
mod manual;
mod object;
mod parser;
mod database;
mod networking;

fn main() {
    get_manual_id();

    let version = format!("1.2.{}", unsafe {MANUAL_ID});

    let matches = App::new("Santa Programming Language")
        .version(version.as_str())
        .author("Santa <santa@santa.northpole>")
        .about(
            "The santa programming system \n\nThis system is used internally at Santa's north \
             pole base for the indexing of the naughty and nice list.",
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("run")
                .about("Run a santa file")
                .arg(Arg::with_name("filename").required(true)),
        )
        .subcommand(SubCommand::with_name("manual").about("Print the manual"))
        .subcommand(
            SubCommand::with_name("RESET")
                .arg(Arg::with_name("version").required(true))
                .about("DO NOT USE, RESETS THE PROGRAMMING LANGUAGE")
                .setting(AppSettings::Hidden),
        )
        .get_matches();

    match matches.subcommand() {
        ("run", Some(matches)) => {
            let filename = matches
                .value_of("filename")
                .expect("Santa couldn't read your filename!");
            let file =
                fs::read_to_string(filename).expect("Santa's elves couldn't find your file!");

            let ast = parse_string_or_panic(&file);

            eval::eval(ast);
        }
        ("manual", Some(_)) => run_manual(),
        ("RESET", Some(matches)) => {
            let version = matches.value_of("version").expect("No version");
            set_manual_id(version.parse().expect("Integer expected"));
        }
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::{eval_node, eval_with_scope, Scope, eval_with_scope_err};
    use crate::function::{Function, ParameterList};
    use crate::object::Object;
    use crate::parser::AstNode::{Assignment, Expression, Integer, Name};
    use crate::parser::{parse_string_or_panic, BinaryOperator, Operator, UnaryOperator};
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::error::SantaError;
    use crate::manual::{MANUAL_ID, FUNCTIONS};

    #[test]
    fn test_simple_1() {
        let ast = parse_string_or_panic("a = 3;");

        assert_eq!(
            ast,
            vec![Box::new(Assignment {
                name: Box::new(Name("a".into())),
                expression: Box::new(Integer(3)),
                indexes: vec![]
            })]
        );

        let scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Integer(3)
        );

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(3)));
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
                })),
                indexes: vec![]
            })]
        );

        let scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Integer(8)
        );

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(8)));
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
                })),
                indexes: vec![]
            })]
        );

        let scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Integer(-2)
        );

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(-2)));
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
                })),
                indexes: vec![]
            })]
        );

        let scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Integer(15)
        );

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(15)));
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
                })),
                indexes: vec![]
            })]
        );

        let scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Float(3.0 / 5.0)
        );

        assert_eq!(
            scope.borrow().get_variable(&"a".into()),
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
                })),
                indexes: vec![]
            })]
        );

        let scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Integer(-3)
        );

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(-3)));
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
                })),
                indexes: vec![]
            })]
        );

        let scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Float(5.0)
        );

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Float(5.0)));
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
                })),
                indexes: vec![]
            })]
        );

        let scope = Scope::new();

        assert_eq!(
            eval_node(ast.into_iter().next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Float(7.0 / 2.0)
        );

        assert_eq!(
            scope.borrow().get_variable(&"a".into()),
            Some(Object::Float(7.0 / 2.0))
        );
    }

    #[test]
    fn test_multiline_1() {
        let ast = parse_string_or_panic("a = 3 + 2;b = a;");

        let scope = Scope::new();

        assert_eq!(ast.len(), 2);

        for node in ast {
            assert_eq!(
                eval_node(node.as_ref(), scope.clone()).unwrap(),
                Object::Integer(5)
            );
        }

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(5)));

        assert_eq!(scope.borrow().get_variable(&"b".into()), Some(Object::Integer(5)));
    }

    #[test]
    fn test_multiline_2() {
        let ast = parse_string_or_panic("a = 3 + 2;b = a + 2;");

        let scope = Scope::new();

        assert_eq!(ast.len(), 2);

        let mut nodes = ast.into_iter();
        assert_eq!(
            eval_node(nodes.next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Integer(5)
        );
        assert_eq!(
            eval_node(nodes.next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Integer(7)
        );

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(5)));

        assert_eq!(scope.borrow().get_variable(&"b".into()), Some(Object::Integer(7)));
    }

    #[test]
    fn test_functioncall_1() {
        let ast = parse_string_or_panic("b = a();");

        let scope = Scope::new();

        assert_eq!(ast.len(), 1);

        let mut nodes = ast.into_iter();

        // create a function called a
        scope.borrow_mut().set_variable(
            "a".into(),
            Object::Function(Function::Builtin(ParameterList::new(vec![]), |_| {
                Ok(Object::Integer(10))
            })),
        );

        assert_eq!(
            eval_node(nodes.next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Integer(10)
        );

        assert_eq!(scope.borrow().get_variable(&"b".into()), Some(Object::Integer(10)));
    }

    #[test]
    fn test_functioncall_2() {
        let ast = parse_string_or_panic("b = a(3);");

        let scope = Scope::new();

        assert_eq!(ast.len(), 1);

        let mut nodes = ast.into_iter();

        // create a function called a
        scope.borrow_mut().set_variable(
            "a".into(),
            Object::Function(Function::Builtin(
                ParameterList::new(vec!["x".into()]),
                |scope| match scope.borrow().get_variable(&"x".into()) {
                    Some(i) => Ok(i),
                    None => Ok(Object::None),
                },
            )),
        );

        assert_eq!(
            eval_node(nodes.next().unwrap().as_ref(), scope.clone()).unwrap(),
            Object::Integer(3)
        );

        assert_eq!(scope.borrow().get_variable(&"b".into()), Some(Object::Integer(3)));
    }

    #[test]
    fn test_function_1() {
        let ast = parse_string_or_panic(
            "
function a (x) {
    return x + 1;
}",
        );

        let scope = Scope::new();

        assert_eq!(ast.len(), 1);

        eval_with_scope(ast, scope.clone());

        let var = scope.borrow().get_variable(&"a".into());
        match var {
            Some(Object::Function(Function::User(x, closure, _))) => {
                assert_eq!(x.positional, vec![String::from("x")]);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_function_2() {
        let ast = parse_string_or_panic(
            "
function a (x) {
    return x + 1;
}

a(3);
",
        );

        let scope = Scope::new();

        assert_eq!(ast.len(), 2);

        assert_eq!(eval_with_scope(ast, scope.clone()), Object::Integer(4));

        let var = scope.borrow().get_variable(&"a".into());
        match var {
            Some(Object::Function(Function::User(x, closure, _))) => {
                assert_eq!(x.positional, vec![String::from("x")]);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_function_3() {
        let ast = parse_string_or_panic(
            "
a = function (x) {
    return x + 1;
};

a(3);
",
        );

        let scope = Scope::new();

        assert_eq!(ast.len(), 2);

        assert_eq!(eval_with_scope(ast, scope.clone()), Object::Integer(4));

        let var = scope.borrow().get_variable(&"a".into());
        match var {
            Some(Object::Function(Function::User(x, closure, _))) => {
                assert_eq!(x.positional, vec![String::from("x")]);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_function_4() {
        let ast = parse_string_or_panic(
            "

x = 5;
function a () {
    return x + 1;
}


assert(a() == 6);
",
        );

        let scope = Scope::new();

        assert_eq!(eval_with_scope_err(ast, scope.clone()), Ok(Object::None));

    }

    #[test]
    fn test_print_1() {
        let ast = parse_string_or_panic(
            "
a = 3;
b = 4;
x = a + b;

print(x);
",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());
    }

    #[test]
    fn test_reassign_1() {
        let ast = parse_string_or_panic(
            "
a = 3;
a = a + 2;
",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope.clone()), Object::Integer(5));

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(5)));
    }

    #[test]
    fn test_if_1() {
        let ast = parse_string_or_panic(
            "
a = false;

if a {
    b = 3;
} else {
    b = 5;
}
",
        );

        let scope = Scope::new();
        // Should be returned from function
        assert_eq!(eval_with_scope(ast, scope.clone()), Object::Integer(5));

        // But not be in scope
        assert_eq!(scope.borrow().get_variable(&"b".into()), None);
    }

    #[test]
    fn test_if_2() {
        let ast = parse_string_or_panic(
            "
a = false;

b = 2;

if a {
    b = 3;
} else {
    b = 5;
}
",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope.clone()), Object::Integer(5));

        assert_eq!(scope.borrow().get_variable(&"b".into()), Some(Object::Integer(5)));
    }

    #[test]
    fn test_if_3() {
        let ast = parse_string_or_panic(
            "
a = true;

b = 2;

if a {
    b = 3;
} else {
    b = 5;
}
",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope.clone()), Object::Integer(3));

        assert_eq!(scope.borrow().get_variable(&"b".into()), Some(Object::Integer(3)));
    }

    #[test]
    fn test_if_assignment_1() {
        let ast = parse_string_or_panic(
            "
a = true;

x = if a {
    3;
} else {
    5;
};

",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope.clone()), Object::Integer(3));

        assert_eq!(scope.borrow().get_variable(&"x".into()), Some(Object::Integer(3)));
    }

    #[test]
    fn test_boolean_1() {
        let ast = parse_string_or_panic(
            "
a = true;
b = false;

c = a + b;
",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope.clone()), Object::Integer(1));

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Boolean(true)));
        assert_eq!(
            scope.borrow().get_variable(&"b".into()),
            Some(Object::Boolean(false))
        );
        assert_eq!(scope.borrow().get_variable(&"c".into()), Some(Object::Integer(1)));
    }

    #[test]
    fn test_float_1() {
        let ast = parse_string_or_panic(
            "
a = 3.14;
",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope.clone()), Object::Float(3.14));

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Float(3.14)));
    }

    #[test]
    fn test_float_2() {
        let ast = parse_string_or_panic(
            "
a = 3.1 + 4.9;
",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope.clone()), Object::Float(8.0));

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Float(8.0)));
    }

    #[test]
    fn test_float_3() {
        let ast = parse_string_or_panic(
            "
a = 3.2 + 4;
",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope.clone()), Object::Float(7.2));

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Float(7.2)));
    }

    #[test]
    fn test_string_1() {
        let ast = parse_string_or_panic(
            "
a = \"test\";
",
        );

        let scope = Scope::new();
        assert_eq!(
            eval_with_scope(ast, scope.clone()),
            Object::String("test".into())
        );

        assert_eq!(
            scope.borrow().get_variable(&"a".into()),
            Some(Object::String("test".into()))
        );
    }

    #[test]
    fn test_comparison_1() {
        let ast = parse_string_or_panic("3==5;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(false));
    }

    #[test]
    fn test_comparison_2() {
        let ast = parse_string_or_panic("3==3;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(true));
    }

    #[test]
    fn test_comparison_3() {
        let ast = parse_string_or_panic("3==3.0;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(true));
    }

    #[test]
    fn test_comparison_4() {
        let ast = parse_string_or_panic("3.0==3;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(true));
    }

    #[test]
    fn test_comparison_5() {
        let ast = parse_string_or_panic("3 > 3;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(false));
    }

    #[test]
    fn test_comparison_6() {
        let ast = parse_string_or_panic("3 < 3;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(false));
    }

    #[test]
    fn test_comparison_8() {
        let ast = parse_string_or_panic("3 >= 3;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(true));
    }

    #[test]
    fn test_comparison_9() {
        let ast = parse_string_or_panic("3 <= 3;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(true));
    }

    #[test]
    fn test_comparison_10() {
        let ast = parse_string_or_panic("4 > 3;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(true));
    }

    #[test]
    fn test_comparison_11() {
        let ast = parse_string_or_panic("3 < 4;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(true));
    }

    #[test]
    fn test_comparison_12() {
        let ast = parse_string_or_panic("3 != 4;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(true));
    }

    #[test]
    fn test_comparison_13() {
        let ast = parse_string_or_panic("3 != 3;");

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::Boolean(false));
    }

    #[test]
    fn test_while_1() {
        let ast = parse_string_or_panic(
            "
a = 0;
while a < 10 {
    a = a + 1;
}

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(10)));
    }

    #[test]
    fn test_stringrepeat_1() {
        let ast = parse_string_or_panic(
            "
a = \"yeet\" * 4;

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::String("yeetyeetyeetyeet".into())));
    }

    #[test]
    fn test_concat_1() {
        let ast = parse_string_or_panic(
            "
a = \"yeet\" + 4;

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::String("yeet4".into())));
    }

    #[test]
    fn test_concat_2() {
        let ast = parse_string_or_panic(
            "
a = \"yeet\" + 4.1;

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::String("yeet4.1".into())));
    }

    #[test]
    fn test_concat_3() {
        let ast = parse_string_or_panic(
            "
a = \"yeet\" + \"yeet\";

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::String("yeetyeet".into())));
    }

    #[test]
    fn test_stringindex_1() {
        let ast = parse_string_or_panic(
            "
a = \"yeet\"[3];

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::String("t".into())));
    }

    #[test]
    fn test_list_1() {
        let ast = parse_string_or_panic(
            "
a = [1, 2, 3, 4];

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::List(Rc::new(RefCell::new(vec![
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
            Object::Integer(4),
        ])))));
    }

    #[test]
    fn test_list_2() {
        let ast = parse_string_or_panic(
            "
a = [1, 2, 3, 4];

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_ne!(scope.borrow().get_variable(&"a".into()), Some(Object::List(Rc::new(RefCell::new(vec![
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(4),
            Object::Integer(4),
        ])))));
    }

    #[test]
    fn test_list_3() {
        let ast = parse_string_or_panic(
            "
a = [1, 2, 3, 4][2];

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(3)));
    }

    #[test]
    fn test_list_4() {
        let ast = parse_string_or_panic(
            "
a = [1, 2,] + [3,] + [4];

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::List(Rc::new(RefCell::new(vec![
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
            Object::Integer(4),
        ])))));
    }

    #[test]
    fn test_list_5() {
        let ast = parse_string_or_panic(
            "
a = [1, 2,] * 3;

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::List(Rc::new(RefCell::new(vec![
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(1),
            Object::Integer(2),
        ])))));
    }

    #[test]
    fn test_list_6() {
        let ast = parse_string_or_panic(
            "
a = [1, 2,];
a[1] = 3;

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::List(Rc::new(RefCell::new(vec![
            Object::Integer(1),
            Object::Integer(3),
        ])))));
    }

    #[test]
    fn test_list_7() {
        let ast = parse_string_or_panic(
            "
a = [[1,2]];
a[0][1] = 3;

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::List(Rc::new(RefCell::new(vec![
            Object::List(Rc::new(RefCell::new(vec![
                Object::Integer(1),
                Object::Integer(3),
            ])))
        ])))));
    }

    #[test]
    fn test_list_8() {
        let ast = parse_string_or_panic(
            "
a = [[1,2]];
b = a[0][1];
            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        let var = scope.borrow().get_variable(&"b".into());
        assert_eq!(var, Some(Object::Integer(2)))
    }

    #[test]
    fn test_map_1() {
        let ast = parse_string_or_panic(
            "
a = {1: 2, 3: 4, 5: 6};

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        let mut map = HashMap::new();
        map.insert(Object::Integer(1), Object::Integer(2));
        map.insert(Object::Integer(3), Object::Integer(4));
        map.insert(Object::Integer(5), Object::Integer(6));
        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Map(Rc::new(RefCell::new(map)))));
    }

    #[test]
    fn test_map_2() {
        let ast = parse_string_or_panic(
            "
a = {1: 2, 3: 4, 5: 6}[5];

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());


        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(6)));
    }

    #[test]
    fn test_map_3() {
        let ast = parse_string_or_panic(
            "
a = {1: 2, 3: 4, 5: 6};
a[3] = 5;
a = a[3];

            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());


        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(5)));
    }

    #[test]
    fn test_if_4() {
        let ast = parse_string_or_panic(
            "
a = 5;
b = 2;

if a > b {
    print(1);
}
            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope);
    }

    #[test]
    fn test_vararg_1() {
        let ast = parse_string_or_panic(
            "
function a(*x) {
    assert(len(x)==1);
}

a(5);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::None);
    }

    #[test]
    fn test_vararg_2() {
        let ast = parse_string_or_panic(
            "
function a(*x) {
    assert(len(x)==2);
}

a(5, 6);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::None);
    }

    #[test]
    fn test_vararg_3() {
        let ast = parse_string_or_panic(
            "
function a(y, *x) {
    assert(len(x)==0);
}

a(5);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope(ast, scope), Object::None);
    }


    #[test]
    fn test_vararg_4() {
        let ast = parse_string_or_panic(
            "
function a(x) {
}

a(5, 6, 7);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope_err(ast, scope), Err(SantaError::InvalidOperationError {cause: "Too many arguments for function".into()}));
    }


    #[test]
    fn test_vararg_5() {
        let ast = parse_string_or_panic(
            "
function a(x, z) {
}

a(5);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope_err(ast, scope), Err(SantaError::InvalidOperationError {cause: "Not enough arguments for function".into()}));
    }


    #[test]
    fn test_vararg_6() {
        let ast = parse_string_or_panic(
            "
function a(x, y, *z) {
}

a(5);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope_err(ast, scope), Err(SantaError::InvalidOperationError {cause: "Not enough arguments for function".into()}));
    }

    #[test]
    fn test_assert_1() {
        let ast = parse_string_or_panic(
            "
assert(false);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope_err(ast, scope), Err(SantaError::AssertionError));
    }

    #[test]
    fn test_vararg_7() {
        let ast = parse_string_or_panic(
            "
print(1,2,3);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope_err(ast, scope), Ok(Object::None));
    }

    #[test]
    fn test_manual_1() {
        let ast = parse_string_or_panic(
            "
function sum(*values) {
    length = len(values);
    total = 0;
    index = 0;

    while index < length {
        total = total + values[index];
        index = index + 1;
    }

    return total;
}

assert(sum(1,2) == 3);
assert(sum(1,2,3) == 6);
assert(sum() == 0);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope_err(ast, scope), Ok(Object::None));
    }

    #[test]
    fn test_manual_2() {
        let ast = parse_string_or_panic(
            "
a = function(x) {
    return x + 1;
};

assert(a(3) == 4);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope_err(ast, scope), Ok(Object::None));
    }

    #[test]
    fn test_manual_3() {
        let ast = parse_string_or_panic(
            "
function a(x) {
    return x + 1;
}

assert(a(3) == 4);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope_err(ast, scope), Ok(Object::None));
    }


    #[test]
    fn test_manual_4() {
        unsafe{MANUAL_ID = FUNCTIONS};

        let ast = parse_string_or_panic(
            "
function assert_eq(a, b) {
    assert(a == b);
    return 42;
}

assert_eq(5,5);
            ",
        );

        let scope = Scope::new();
        assert_eq!(eval_with_scope_err(ast, scope), Ok(Object::Integer(42)));
    }
}
