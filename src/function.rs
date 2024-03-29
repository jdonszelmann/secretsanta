use crate::error::SantaError;
use crate::eval::{eval_block_with_scope, Scope};
use crate::function::Function::{Builtin, User};
use crate::object::Object;
use crate::parser::AstNode;
use std::fmt::{Debug, Formatter, Error};
use std::rc::Rc;
use std::cell::RefCell;

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
    pub fn empty() -> Self {Self { positional: vec![]}}
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
    Builtin(ParameterList, fn(Rc<RefCell<Scope>>) -> Result<Object, SantaError>),
    User(ParameterList, Rc<RefCell<Scope>>, Vec<Box<AstNode>>),
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Builtin(_, b) => write!(f, "Builtin function at {:p}", b),
            User(args, _closure,  _) => write!(
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
            Self::User(argsu, _closure, u) => {
                if let Self::User(argsou, _closure, ou) = other {
                    u == ou && argsu == argsou
                } else {
                    false
                }
            }
        }
    }
}

impl Function {
    pub fn call(&self, argumentlist: ArgumentList) -> Result<Object, SantaError> {

        match self {
            Self::Builtin(params, b) => {
                let scope = Scope::new();
                scope.borrow_mut().load_arglist(argumentlist, params.clone())?;
                b(scope)
            }
            Self::User(params, closure, ast) => {
                let scope = Scope::child(closure.clone());
                scope.borrow_mut().load_arglist(argumentlist, params.clone())?;
                let a = eval_block_with_scope(&ast, scope);
                a
            }
        }
    }
}
