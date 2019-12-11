use crate::parser::{AstNode, BinaryOperator, UnaryOperator};
use crate::object::Object;
use std::collections::HashMap;
use crate::parser::Operator;
use crate::error::SantaError;
use crate::function::{ParameterList, ArgumentList, Function};

#[derive(Debug)]
pub struct Scope<'s> {
    parent: Option<&'s mut Scope<'s>>,
    locals: HashMap<String, Object>,
}

impl<'s> Scope<'s> {
    pub fn new() -> Self{
        Self::with_parent(None)
    }

    pub fn with_parent(parent: Option<&'s Scope<'s>>) -> Self {
        Self {
            parent: None,
            locals: HashMap::new(),
        }
    }

    pub fn child(&'s mut self) -> Scope<'s>{
        Self::with_parent(Some(self))
    }

    fn find_variable(&mut self, name: &String) -> Option<&mut Object>{
        if self.locals.contains_key(name) {
            self.locals.get_mut(name)
        } else {
            if let Some(ref mut parent) = self.parent {
                parent.find_variable(name)
            } else {
                None
            }
        }
    }

    pub fn set_variable<'set>(&'set mut self, name: String, value: Object) {
        if let Some(var) = self.find_variable(&name) {
            *var = value;
        } else {
            self.locals.insert(name, value);
        }
    }

    pub fn get_variable(&self, name: &String) -> Option<Object> {
        if let Some(i) =  self.locals.get(name) {
            return Some(i.to_owned());
        } else {
            if let Some(parent) = &self.parent {
                parent.get_variable(name)
            } else {
                None
            }
        }
    }

    pub fn load_arglist(&mut self, arglist: ArgumentList, paramlist: ParameterList) {
        for (param, arg) in paramlist.positional.iter().zip(arglist.positional.iter()) {
            self.set_variable(param.clone(), arg.clone());
        }
    }
}

pub fn eval_node<'o>(node: Box<AstNode>, scope: &mut Scope) -> Result<Object, SantaError>{
    match *node {
        AstNode::None => return Ok(Object::None),
        AstNode::Expression(operatortype) =>
            match operatortype {
                Operator::Binary {operator, rhs, lhs} => {
                    let rhs_eval = eval_node(rhs, scope)?;
                    let lhs_eval = eval_node(lhs, scope)?;
                    match operator{
                        BinaryOperator::Add => lhs_eval.add(&rhs_eval),
                        BinaryOperator::Multiply => lhs_eval.multiply(&rhs_eval),
                        BinaryOperator::Divide => lhs_eval.divide(&rhs_eval),
                        BinaryOperator::Subtract => lhs_eval.subtract(&rhs_eval),
                    }
                },
                Operator::Unary {operator, expr} => {
                    let expr_eval = eval_node(expr, scope)?;

                    match operator {
                        UnaryOperator::Negate => expr_eval.negate()
                    }
                }
            },
        AstNode::Assignment {name, expression} => {
            let evaluated = eval_node(expression, scope)?;
            match *name {
                AstNode::Name(name) => {
                    scope.set_variable(name, evaluated.clone());
                    Ok(evaluated)
                }
                _ => Err(SantaError::InvalidOperationError {
                    cause: "Tried to assign to something that's not a variable name".into()
                })
            }
        }

        AstNode::Integer(integer) => {
            Ok(Object::Integer(integer))
        }
        AstNode::String(string) => {
            Ok(Object::String(string))
        }
        AstNode::Name(string) => {
            Ok(scope.get_variable(&string).ok_or(SantaError::InvalidOperationError {
                cause: "Variable not defined".into()
            })?)
        }
        AstNode::Functioncall { value, args  } => {
            let variable = eval_node(value, scope)?;
            let mut arguments = ArgumentList::new(vec![]);
            for i in args {
                arguments.positional.push(eval_node(i, scope)?)
            }

            variable.call(arguments)
        }
        AstNode::Function { name, parameterlist, code } => {
            let func = Object::Function(Function::User(parameterlist, code));

            // If you gave the function a name, assign it to a variable with that name.
            if let AstNode::Name(name) = *name {
                scope.set_variable(name, func.clone());
            }

            Ok(func)
        }
        _ => unimplemented!()
    }
}

pub fn eval(ast: Vec<Box<AstNode>>) {
    let mut scope = Scope::new();

    eval_with_scope(ast, &mut scope);
}

pub fn eval_with_scope<'o>(ast: Vec<Box<AstNode>>, scope: &mut Scope) -> Object {

    let mut last_answer = Object::None;
    for node in ast {
        match eval_node(node, scope) {
            Err(e) => {
                println!("{}", e);
                return last_answer;
            },
            Ok(i) => {
                last_answer = i;
            }
        };
    }

    last_answer

}

