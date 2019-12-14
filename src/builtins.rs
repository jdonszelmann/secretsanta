use crate::function::ParameterList;
use crate::eval::Scope;
use crate::object::Object;

fn builtin_print(scope: &mut Scope) -> Object {
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