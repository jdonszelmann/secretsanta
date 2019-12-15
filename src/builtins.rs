use crate::eval::Scope;
use crate::function::ParameterList;
use crate::manual::{increment_manual_id, MANUAL_ID};
use crate::object::Object;
use colored::Colorize;

fn builtin_print(scope: &mut Scope) -> Object {
    if unsafe { MANUAL_ID } == 0 {
        println!(
            "{}",
            "You used the print function for the first time!".yellow()
        );
        increment_manual_id();
    }

    println!(
        "{}",
        scope
            .get_variable(&"var".into())
            .expect("No printable var found")
    );
    Object::None
}

pub fn get_builtins(scope: &mut Scope) {
    scope.add_builtin_fn(
        "print",
        ParameterList::new(vec!["var".into()]),
        builtin_print,
    );
}
