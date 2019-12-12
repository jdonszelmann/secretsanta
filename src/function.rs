use crate::eval::{eval_with_scope, Scope};
use crate::function::Function::{Builtin, User};
use crate::object::Object;
use crate::parser::AstNode;
use failure::_core::fmt::{Debug, Error, Formatter};

/// An ArgumentList is a a struct holding information about
/// what parameters a function wants.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterList {
    pub positional: Vec<String>,
}

impl ParameterList {
    pub fn new(positional: Vec<String>) -> Self {
        Self { positional }
    }
}

/// A ParameterList is a struct which are the parameters
/// given to a function when called.
#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentList {
    pub positional: Vec<Object>,
}

impl ArgumentList {
    pub fn new(positional: Vec<Object>) -> Self {
        Self { positional }
    }
}

#[derive(Clone)]
pub enum Function {
    Builtin(ParameterList, fn(&mut Scope) -> Object),
    User(ParameterList, Vec<Box<AstNode>>),
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Builtin(_, b) => write!(f, "Builtin function at {:p}", b),
            User(args, _) => write!(
                f,
                "Function({})",
                args.positional
                    .iter()
                    .fold(String::new(), |acc, num| acc + &num.to_string() + ", ")
            ),
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Builtin(argsb, b) => {
                if let Self::Builtin(argsob, ob) = other {
                    std::ptr::eq(b, ob) && argsb == argsob
                } else {
                    false
                }
            }
            Self::User(argsu, u) => {
                if let Self::User(argsou, ou) = other {
                    u == ou && argsu == argsou
                } else {
                    false
                }
            }
        }
    }
}

impl Function {
    pub fn call(&self, argumentlist: ArgumentList) -> Object {
        let mut scope = Scope::new();

        match self {
            Self::Builtin(params, b) => {
                scope.load_arglist(argumentlist, params.clone());
                b(&mut scope)
            }
            Self::User(params, ast) => {
                scope.load_arglist(argumentlist, params.clone());
                eval_with_scope(ast.to_vec(), &mut scope)
            }
        }
    }
}
