use crate::function::ParameterList;
use crate::eval::Scope;
use crate::object::Object;
use crate::manual::{MANUAL_ID, increment_manual_id};

fn builtin_print(scope: &mut Scope) -> Object {
    if unsafe{MANUAL_ID} == 0 {
        increment_manual_id();
    }



    println!("{}", scope.get_variable(
        &"var".into()
    ).expect("No printable var found"));
    Object::None
}

pub fn get_builtins(scope: &mut Scope) {
    scope.add_builtin_fn(
        "print",
        ParameterList::new(vec![
            "var".into()
        ]),
        builtin_print
    );
}