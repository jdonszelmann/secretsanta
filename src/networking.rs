use crate::eval::Scope;
use crate::function::{ParameterList, Function, ArgumentList};
use crate::error::SantaError;
use crate::object::Object;
use std::cell::RefCell;
use std::rc::Rc;
use crate::manual::{MANUAL_ID, NETWORKING_1, increment_manual_id, NETWORKING_2};
use colored::Colorize;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use crate::database::{Database, get_default_db, GLOBAL_DATABASE};
use std::sync::MutexGuard;

const NAMES: [&str; 57] = [
    "Loyd Pellegrino",
    "Deangelo Nilsson",
    "Talitha Martines",
    "Lela Oppenheimer",
    "Cheyenne Bartlett",
    "Yesenia Stenson",
    "Daniel Lovelace",
    "Laura Pircalaboiu",
    "Robbin Baauw",
    "Dany Sluijk",
    "Jonathan Donszelmann",
    "Victor Roest",
    "Ricardo Vogel",
    "Julius de Jeu",
    "Kristi Hambleton",
    "Benedict Nemec",
    "Margrett Shortt",
    "Juli Tames",
    "Jacquelyne Vandeventer",
    "Sanda Kean",
    "Sean Lard",
    "Milda Zambrano",
    "Ema Hessling",
    "Shante Gamon",
    "Cristine Blanford",
    "Melvina Lilly",
    "Rosena Carlon",
    "Angelyn Crocker",
    "Kitty Truesdale",
    "Lauralee Tweed",
    "Kristie Bogue",
    "Mayola Rumble",
    "Harriet Landa",
    "Gerard Glascock",
    "Richelle Mizer",
    "Tess Mohney",
    "Ryann Ragan",
    "Maude Hinesley",
    "Steven Sones",
    "Danielle Storms",
    "Gwen Leon",
    "Johnny Sova",
    "Larue Villegas",
    "Francene Behrendt",
    "Lili Armes",
    "Jong Weston",
    "Sabina Baldonado",
    "Genna Denis",
    "Olga Salas",
    "Twana Topps",
    "Patrica Toal",
    "Evelia Willms",
    "Barbra Slattery",
    "Marlo Kabel",
    "Laticia Geddie",
    "Na Perino",
    "Cordie Debose",
];

static mut NETWORK_HANDLER: Option<Function> = None;

fn builtin_network_listen(scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError> {
    let mut rng = rand::thread_rng();
    let mut local_db = get_default_db();

    if let Some(func) = unsafe {&NETWORK_HANDLER} {

        for i in 0..20{
            let record = rng.gen_range(0, local_db.records.len());

            match rng.gen_range(0, 2) {
                0 => {
                    let value = [true, false][rng.gen_range(0,2)];

                    local_db.set_first("id".into(), Object::Integer(record as i64), "isnaughty".into(), Object::Boolean(value));

                    func.call(ArgumentList::new(vec![
                        Object::String(format!("update id {}; set isnaughty=<{}>", record, value)),
                    ]))?;
                },
                1 => {
                    let numnames = NAMES.len();
                    let value = NAMES[rng.gen_range(0, numnames)];
                    local_db.set_first("id".into(), Object::Integer(record as i64), "name".into(), Object::String(value.into()));

                    func.call(ArgumentList::new(vec![
                        Object::String(format!("update id {}; set name=<{}>", record, value)),
                    ]))?;
                },
                _ => unimplemented!(),
            }

            sleep(Duration::from_millis(420));
        }

        if unsafe{MANUAL_ID} == NETWORKING_1 {
            println!(
                "{}",
                "You successfully registered a network handler!".yellow()
            );
            increment_manual_id();
            return Ok(Object::None);
        }


    }

    if unsafe{MANUAL_ID} == NETWORKING_2 {
        let database: MutexGuard<Database> = GLOBAL_DATABASE.lock().unwrap();

//        dbg!(database.tables.get("list".into()).unwrap());
//        dbg!(local_db.tables.get("list".into()).unwrap());

        if *database == local_db {
            println!(
                "{}",
                "Well done!, you executed all queries correctly!".yellow()
            );
            increment_manual_id();
        } else {
            println!(
                "{}",
                "Unfortunately you made some mistakes in the execution of the queries. Try again!".bright_red()
            );
        }
    }

    Ok(Object::None)
}

fn builtin_register_network_handler(scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError>{
    if let Some(Object::Function(func)) = scope.borrow().get_variable(&"function".into()) {


        unsafe {NETWORK_HANDLER = Some(func)}
        Ok(Object::None)
    } else {
        Err(SantaError::DatabaseError {cause: "register_network_handler expected a function as argument".into()})
    }
}

pub fn get_network_builtins(scope: &mut Scope) {
    scope.add_builtin_fn(
        "listen".into(),
        ParameterList::empty(),
        builtin_network_listen,
    );

    scope.add_builtin_fn(
        "register_network_handler".into(),
        ParameterList::new(vec!["function".into()]),
        builtin_register_network_handler
    );

}

#[cfg(test)]
mod tests {
    use crate::parser::parse_string_or_panic;
    use crate::eval::{Scope, eval_with_scope};

    #[test]
    fn test_networking_1() {
        let ast = parse_string_or_panic(
            "
register_network_handler(function (data){
    print(data);
});

listen();
            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());
    }
}

