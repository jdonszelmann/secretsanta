use crate::object::{Object, vec_to_list};
use std::collections::HashMap;
use crate::error::SantaError;
use std::ops::{Deref, DerefMut};
use crate::manual::{NAME, MANUAL_ID, DATABASES_TEST3, increment_manual_id};
use crate::eval::Scope;
use std::cell::RefCell;
use std::rc::Rc;
use crate::function::ParameterList;
use lazy_static::lazy_static;
use std::sync::Mutex;
use colored::Colorize;

pub static mut ACCESSED_DB: bool = false;

#[derive(Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub records: Vec<Vec<Object>>,
}

impl Table {
    pub fn new(name: String, columns: Vec<String>) -> Self {
        Self {
            name,
            columns,
            records: vec![],
        }
    }

    pub fn add_record(&mut self, record: Vec<Object>) -> Result<(), SantaError> {
        if record.len() < self.columns.len() {
            return Err(SantaError::DatabaseError { cause: "Record size did not match".into()})
        }
        self.records.push(record);

        Ok(())
    }

    pub fn get_first(&self, column: String, value: Object) -> Result<&Vec<Object>, SantaError> {
        let index = self.columns.iter().position(|i| i == &column).ok_or(SantaError::DatabaseError {cause: "Column doesn't exist".into()})?;

        unsafe {ACCESSED_DB = true}

        self.records.iter().filter(|i|  i[index] == value).next().ok_or(SantaError::DatabaseError {
            cause: format!("Value {} not found in database", value)
        })
    }

    pub fn set_first(&mut self, column: String, value: Object, newcolumn: String, newvalue: Object) -> Result<(), SantaError> {
        if unsafe {MANUAL_ID} == DATABASES_TEST3 {
            if (value == Object::String("Tim Anema".into()) || value == Object::Integer(42)) &&
                newcolumn == "isnaughty" && newvalue == Object::Boolean(false){
                println!(
                    "{}",
                    "You changed your own status to nice!".yellow()
                );
                increment_manual_id();
            }
        }

        let index = self.columns.iter().position(|i| i == &column).ok_or(SantaError::DatabaseError {cause: "Column doesn't exist".into()})?;
        let newindex = self.columns.iter().position(|i| i == &newcolumn).ok_or(SantaError::DatabaseError {cause: "Column doesn't exist".into()})?;

        unsafe {ACCESSED_DB = true}

        let value = self.records.iter_mut().filter(|i|  i[index] == value).next().ok_or(SantaError::DatabaseError {
            cause: format!("Value {} not found in database", value)
        })?;

        value[newindex] = newvalue;

        Ok(())
    }

    pub fn get_all(&self, column: String, value: Object) -> Result<Vec<&Vec<Object>>, SantaError> {
        let index = self.columns.iter().position(|i| i == &column).ok_or(SantaError::DatabaseError {cause: "Column doesn't exist".into()})?;

        unsafe {ACCESSED_DB = true}

        Ok(self.records.iter().filter(|i|  i[index] == value).collect())
    }

    fn get_records(&self) -> Result<&Vec<Vec<Object>>, SantaError> {

        unsafe {ACCESSED_DB = true}

        Ok(&self.records)
    }
}

#[derive(Debug, PartialEq)]
pub struct Database {
    pub tables: HashMap<String, Table>,
    pub current_table: String
}

unsafe impl Send for Database{}
unsafe impl Sync for Database{}

impl Database {
    fn new() -> Self {
        let mut hm = HashMap::new();
        hm.insert("default".into(), Table::new("Default".into(), vec![]));

        Self {
            tables: hm,
            current_table: "default".into(),
        }
    }

    fn set_current_table(&mut self, table: String) -> Result<(), SantaError> {
        if self.tables.contains_key(&table) {
            self.current_table = table;
            Ok(())
        } else {
            Err(SantaError::DatabaseError {cause: "Table doesnt't exist!".into()})
        }
    }

    fn add_table(&mut self, table: Table) {
        self.tables.insert(table.name.clone(), table);
    }

    fn get_table_names(&self) -> Vec<&String> {
        self.tables.keys().collect()
    }

    fn get_table(&self, name: String) -> Option<&Table> {
        self.tables.get(&name)
    }

    fn get_table_mut(&mut self, name: String) -> Option<&mut Table> {
        self.tables.get_mut(&name)
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Table {
        self.tables.get_mut(&self.current_table).unwrap()
    }
}

impl Deref for Database {
    type Target = Table;

    fn deref(&self) -> &Table {
        self.tables.get(&self.current_table).unwrap()
    }
}


pub fn get_default_db() -> Database {
    let mut db = Database::new();
    let listtable = Table::new("list".into(), vec!["id".into(), "name".into(), "isnaughty".into()]);
    db.add_table(listtable);


    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(0),Object::String("Timmy Chang".into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(1),Object::String("Vernie Goodale".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(2),Object::String("Siu Oneil".into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(3),Object::String("Leah Dunstan".into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(4),Object::String("Kenny Maynard".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(5),Object::String("Chrystal Krieger".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(6),Object::String("Elisha Waldrep".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(7),Object::String("Lanell Biro".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(8),Object::String("Annett Duhe".into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(9),Object::String("Marilu Lesane".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(10),Object::String("Ilene Montealegre".into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(11),Object::String("Prudence Helbing".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(12),Object::String("Lorri Dalman".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(13),Object::String("Liz Manley".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(14),Object::String("Bea Sund".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(15),Object::String("Jerica Jeffreys".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(16),Object::String("Jose Robert".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(17),Object::String("Lashawn Adams".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(18),Object::String("Jerri Soucie".into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(19),Object::String("Lazaro Baynes".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(20),Object::String("Camellia Netzer".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(21),Object::String("Pasquale Hiner".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(22),Object::String("Mikel Carron".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(23),Object::String("Hank Dragon".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(24),Object::String("Nickolas Hamann".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(25),Object::String("Deshawn Santistevan".into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(26),Object::String("Jalisa Moose".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(27),Object::String("Angelena Flett".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(28),Object::String("Tommie Strand".into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(29),Object::String("Leonel Creger".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(30),Object::String("Barbar Opie".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(31),Object::String("Charita Boatman".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(32),Object::String("Marlen Delgado".into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(33),Object::String("Shane Betters".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(34),Object::String("Patrica Mccallie".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(35),Object::String("Greg Hollie".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(36),Object::String("Willa Roa".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(37),Object::String("Ora Spadaro".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(38),Object::String("Holley Mcdonalds".into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(39),Object::String("Jana Leonetti".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(40),Object::String("Cole Zoll".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(41),Object::String("Jenniffer Fazzino".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(42),Object::String(NAME.into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(43),Object::String("Dominique Pinnell".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(44),Object::String("Lori Moeckel".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(45),Object::String("Elida Carvalho".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(46),Object::String("Cortez Gertsch".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(47),Object::String("Marc Dunkle".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(48),Object::String("Maple Linsley".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(49),Object::String("Birdie Pence".into()),Object::Boolean(false),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(50),Object::String("Patty Antilla".into()),Object::Boolean(true),]).unwrap();
    db.get_table_mut("list".into()).unwrap().add_record(vec![Object::Integer(51),Object::String("Ethan Bowdoin".into()),Object::Boolean(false),]).unwrap();


    db.set_current_table("list".into()).unwrap();

    db
}


lazy_static!{
    pub static ref GLOBAL_DATABASE: Mutex<Database> = Mutex::new(get_default_db());
}

pub fn get_naughty() -> i64{
    let mut count = 0;
    let database = GLOBAL_DATABASE.lock().unwrap();

    for i in database.get_records().unwrap() {
        if i[2] == Object::Boolean(true) {
            count+=1;
        }
    }
    count
}

pub fn builtin_db_columns(_scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError>{
    let database = GLOBAL_DATABASE.lock().unwrap();

    Ok(Object::List(Rc::new(RefCell::new(
        database.columns
            .iter()
            .map(|i| Object::String(i.clone()))
            .collect()
    ))))
}

pub fn builtin_db_get(scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError> {
    let database = GLOBAL_DATABASE.lock().unwrap();

    if let Some(Object::String(column)) = scope.borrow().get_variable(&"column".into()) {
        if database.columns.contains(&column) {
            if let Some(value) = scope.borrow().get_variable(&"value".into()) {
                Ok(vec_to_list(database.get_first(column, value)?.clone()))
            } else {
                Err(SantaError::InvalidOperationError {cause: "No value found in parameters".into()})
            }
        } else {
            Err(SantaError::DatabaseError {cause: format!("No column with name {}", column)})
        }
    } else {
        Err(SantaError::InvalidOperationError {cause: format!(
            "column {} parameter not a string", scope.borrow().get_variable(&"column".into()).expect("no column parameter"))})
    }
}

pub fn builtin_db_set(scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError> {
    let mut database = GLOBAL_DATABASE.lock().unwrap();

    if let Some(Object::String(column)) = scope.borrow().get_variable(&"column".into()) {
        if let Some(value) = scope.borrow().get_variable(&"value".into()) {
            if let Some(Object::String(newcolumn)) = scope.borrow().get_variable(&"newcolumn".into()) {
                if let Some(newvalue) = scope.borrow().get_variable(&"newvalue".into()) {
                    if database.columns.contains(&column){
                        if database.columns.contains(&newcolumn){
                            database.set_first(column, value, newcolumn, newvalue)?;
                            Ok(Object::None)
                        } else {
                            Err(SantaError::DatabaseError {cause: format!("new column '{}' not found", newcolumn)})
                        }
                    } else {
                        Err(SantaError::DatabaseError {cause: format!("column '{}' not found", column)})
                    }
                } else {
                    Err(SantaError::DatabaseError {cause: "parameter newvalue not found".into()})
                }
            } else {
                Err(SantaError::DatabaseError {cause: "parameter newcolname must be a string".into()})
            }
        } else {
            Err(SantaError::DatabaseError {cause: "parameter colvalue not found".into()})
        }
    } else {
        Err(SantaError::DatabaseError {cause: "parameter colname must be a string".into()})
    }
}

pub fn builtin_db_get_all(_scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError> {
    let database = GLOBAL_DATABASE.lock().unwrap();

    let mut res = vec![];
    let records: &Vec<Vec<Object>> = database.get_records()?;

    for i in records {
       res.push(vec_to_list(i.clone()));
    }

    Ok(vec_to_list(res))
}


pub fn builtin_db_records(_scope: Rc<RefCell<Scope>>) -> Result<Object, SantaError> {
    let database = GLOBAL_DATABASE.lock().unwrap();

    let records: usize = database.get_records()?.len();

    Ok(Object::Integer(records as i64))
}



pub fn get_db_builtins(scope: &mut Scope) {

    scope.add_builtin_fn(
        "db_columns".into(),
        ParameterList::empty(),
        builtin_db_columns ,
    );

    scope.add_builtin_fn(
        "db_get".into(),
        ParameterList::new(vec!["column".into(), "value".into()]),
        builtin_db_get
    );

    scope.add_builtin_fn(
        "db_set".into(),
        ParameterList::new(vec!["column".into(), "value".into(), "newcolumn".into(), "newvalue".into()]),
        builtin_db_set
    );

    scope.add_builtin_fn(
        "db_get_all".into(),
        ParameterList::new(vec![]),
        builtin_db_get_all
    );

    scope.add_builtin_fn(
        "db_records".into(),
        ParameterList::new(vec![]),
        builtin_db_records
    );
}

#[cfg(test)]
mod tests{
    use crate::database::{get_default_db, get_naughty, ACCESSED_DB};
    use crate::object::Object;
    use crate::manual::NAME;
    use crate::parser::parse_string_or_panic;
    use crate::eval::{Scope, eval_with_scope};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_db_1() {
        let mut db = get_default_db();

        db.add_record(vec![
            Object::Integer(100),
            Object::String("test".into()),
            Object::Boolean(true),
        ]).unwrap();

        assert_eq!(
            db.get_first("id".into(), Object::Integer(100)).unwrap(),
            &vec![
               Object::Integer(100),
               Object::String("test".into()),
               Object::Boolean(true),
           ]
        );
    }

    #[test]
    fn test_db_2() {
        let db = get_default_db();

        assert_eq!(
            db.get_first("id".into(), Object::Integer(42)).unwrap()[1],
            Object::String(NAME.into())
        );
    }

    #[test]
    fn test_db_3() {
        let ast = parse_string_or_panic(
            "
a = db_columns();
            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::List(Rc::new(RefCell::new(vec![
            Object::String("id".into()),
            Object::String("name".into()),
            Object::String("isnaughty".into()),
        ])))));
    }

    #[test]
    fn test_db_4() {
        let ast = parse_string_or_panic(
            "
a = db_get(\"id\", 42);
            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::List(Rc::new(RefCell::new(vec![
            Object::Integer(42),
            Object::String("Tim Anema".into()),
            Object::Boolean(true),
        ])))));
    }

    #[test]
    fn test_db_5() {
        let ast = parse_string_or_panic(
            "
a = db_get_all()[42];
            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::List(Rc::new(RefCell::new(vec![
            Object::Integer(42),
            Object::String(NAME.into()),
            Object::Boolean(true),
        ])))));
    }

    #[test]
    fn test_db_6() {
        let ast = parse_string_or_panic(
            "
a = db_records();
            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::Integer(52)));
    }

    #[test]
    fn test_db_7() {
        assert_eq!(get_naughty(), 12);
    }

    #[test]
    fn test_db_8() {
        let ast = parse_string_or_panic(
            "
a = db_get_all();
length = len(a);

index = 0;
count = 0;

while index < length {
    if a[index][2] == true {
        count = count + 1;
    }

    index = index + 1;
}

print(count);
            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());

        assert_eq!(scope.borrow().get_variable(&"count".into()), Some(Object::Integer(get_naughty())));

        assert!(unsafe {ACCESSED_DB});
    }


    #[test]
    fn test_db_9() {
        let ast = parse_string_or_panic(
            "
db_set(\"id\", 3, \"name\", \"yeet\");
db_set(\"id\", 3, \"isnaughty\", true);
a = db_get(\"id\", 3);
            ",
        );

        let scope = Scope::new();
        eval_with_scope(ast, scope.clone());


        assert_eq!(scope.borrow().get_variable(&"a".into()), Some(Object::List(Rc::new(RefCell::new(vec![
            Object::Integer(3),
            Object::String("yeet".into()),
            Object::Boolean(true),
        ])))));
    }
}