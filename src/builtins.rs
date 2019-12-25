use crate::eval::Scope;
use crate::function::ParameterList;
use crate::manual::{increment_manual_id, MANUAL_ID, BASICS, DATABASES, DATABASES_TEST2};
use crate::object::Object;
use colored::Colorize;
use crate::error::SantaError;
use crate::database::{get_db_builtins, ACCESSED_DB};
use std::process::exit;
use std::cell::RefCell;
use std::rc::Rc;
use crate::networking::get_network_builtins;

fn builtin_print(scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError> {
    if unsafe { MANUAL_ID } == BASICS {
        println!(
            "{}",
            "You used the print function for the first time!".yellow()
        );
        increment_manual_id();
    }



    if let Some(Object::List(lst)) = scope.borrow().get_variable(&"args".into()) {
        for i in lst.borrow().iter() {

            if unsafe { MANUAL_ID } == DATABASES && unsafe { ACCESSED_DB }  && i == &Object::Integer(12) {
                println!(
                    "{}",
                    "You found the right answer to Test 1!".yellow()
                );
                increment_manual_id();
            }

            print!("{} ",i);
        }
        println!();

    } else {
        return Err(SantaError::InvalidOperationError {cause: "No args found".into()})
    }



    Ok(Object::None)
}

fn builtin_list_push(scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError> {
    if let Some(Object::List(list)) = scope
        .borrow()
        .get_variable(&"list".into()) {

        list.borrow_mut().push(scope
            .borrow()
            .get_variable(&"value".into()).ok_or(SantaError::InvalidOperationError {cause: "No value found".into()})?)
    } else {
        return Err(SantaError::InvalidOperationError {cause: "First parameter to push not a list".into()});
    }


    Ok(Object::None)
}

fn builtin_len(scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError> {
    if let Some(obj) = scope
        .borrow()
        .get_variable(&"value".into()) {

        Ok(Object::Integer(match obj {
            Object::String(s) => s.len(),
            Object::List(l) => l.borrow().len(),
            Object::Map(m) => m.borrow().len(),
            i =>  return Err(SantaError::InvalidOperationError {cause: format!("length of {} not defined", i)})
        } as i64))
    } else {
        Err(SantaError::InvalidOperationError {cause: "No parameters found to len function".into()})
    }
}

fn builtin_assert(scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError> {

    if let Some(Object::Boolean(b)) = scope
        .borrow()
        .get_variable(&"arg".into()) {

        if b {

            if unsafe { MANUAL_ID } == DATABASES_TEST2 && unsafe { ACCESSED_DB } {
                println!(
                    "{}",
                    "You used the assert function for the first time!".yellow()
                );
                increment_manual_id();
            }

            Ok(Object::None)
        } else {
            Err(SantaError::AssertionError)
        }


    } else {
        Err(SantaError::InvalidOperationError {cause: "The assert function expects a single boolean.".into()})
    }
}

fn builtin_exit(scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError> {

    if let Some(Object::Integer(code)) = scope
        .borrow()
        .get_variable(&"code".into()) {

        exit(code as i32);
    } else {
        Err(SantaError::InvalidOperationError {cause: "The exit function expects a single integer exit code.".into()})
    }
}

pub fn get_builtins(scope: &mut Scope) {
    scope.set_variable("SANTA_VERSION".into(), Object::Integer(unsafe{MANUAL_ID} as i64));

    scope.add_builtin_fn(
        "print",
        ParameterList::new(vec!["*args".into()]),
        builtin_print,
    );

    scope.add_builtin_fn(
        "list_push",
        ParameterList::new(vec!["list".into(), "value".into()]),
        builtin_list_push,
    );

    scope.add_builtin_fn(
        "len",
        ParameterList::new(vec!["value".into()]),
        builtin_len,
    );

    scope.add_builtin_fn(
        "assert",
        ParameterList::new(vec!["arg".into()]),
        builtin_assert,
    );

    scope.add_builtin_fn(
        "exit",
        ParameterList::new(vec!["code".into()]),
        builtin_exit,
    );


    get_db_builtins(scope);
    get_network_builtins(scope);
}
