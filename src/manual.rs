use colored::Colorize;
use regex::Regex;
use std::env::current_exe;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

pub static mut MANUAL_ID: usize = 0;
const MANUAL_MAIN_FILE: &'static str = "main.md";
const MANUAL_DIR: &'static str = ".manual";

fn get_manual_dir() -> PathBuf {
    let executable_file = current_exe().expect("Couln't find executable directory");

    let executable_dir = executable_file
        .parent()
        .expect("Couldn't access parent directory of executable");

    let manual_dir = executable_dir.join(MANUAL_DIR);
    std::fs::create_dir_all(&manual_dir).expect("Couldn't create manual director. Is it writable?");

    manual_dir
}

pub fn get_manual_id() {
    let manual_dir = get_manual_dir();

    let mainfile_path = manual_dir.join(MANUAL_MAIN_FILE);
    let manualid = match File::open(&mainfile_path) {
        Ok(mut mainfile) => {
            let mut buf = String::new();
            mainfile.read_to_string(&mut buf).expect("Couldn't read");

            let re = Regex::new(r"manual version \d.\d.(\d+)").unwrap();
            let mat = match re.captures(&buf) {
                Some(i) => match i.get(1) {
                    Some(m) => m.as_str(),
                    None => "0",
                },
                None => "0",
            };

            match mat.parse() {
                Ok(i) => i,
                Err(_) => 0,
            }
        }
        Err(_) => 0,
    };

    unsafe { MANUAL_ID = manualid };
}

pub fn increment_manual_id() {
    unsafe { MANUAL_ID += 1 };
    generate_manual();

    println!("{}", "Good job! You have advanced to the next version of the Santa programming language. Check your manual!".red());
}

pub fn set_manual_id(version: usize) {
    unsafe { MANUAL_ID = version };
    generate_manual();

    println!("{}", "Manual was reset.".red());
}

pub fn find_editor() -> String {
    std::env::var("EDITOR").unwrap_or("vim".into())
}

pub fn run_manual() {
    let editor = find_editor();

    println!("Starting manual");

    generate_manual();

    let manual_dir = get_manual_dir();

    let mainfile_path = manual_dir.join(MANUAL_MAIN_FILE);
    let _ = Command::new(&editor)
        .arg("-m")
        .arg(&mainfile_path)
        .status()
        .expect(&format!("Couldn't start editor: {}", &editor));
}

fn generate_manual() {
    let manual_dir = get_manual_dir();

    let mainfile_path = manual_dir.join(MANUAL_MAIN_FILE);
    let mut mainfile = File::create(&mainfile_path).expect("Couldn't create manual file");

    let blocks = match unsafe{ MANUAL_ID } {
        i if i > 0 => format!("### Complex expressions

#### Booleans

Booleans can be used in combination with complex expressions (if/while) to create control flow.

Booleans can have the value `true` and `false` and will act like the integers `1` and `0` respectively in mathmatical operations.
The `-` operator to negate a number (`5` --> `-5`) is used to invert booleans.

#### Comparison

You can compare numbers and other data types using the following operators:

```
a == b;
a != b;
a >= b;
a <= b;
a > b;
a < b;
```

You can compare floats and integers together. Watch out for floating point errors! Booleans act as the integers
0 and 1 under comparison with an integer. Strings can be compared for euqality only.

### Conditionals

Using the comparison operators, you can now build programs that conditionally execute code.
This is done with an if statement. If statements look like this:

```
if a > b {{
    print(\"a greater than b\");
}} else {{
    print(\"a not greater than b\");
}}

```

If statements can work as ternary operators by assigning them to a variable.
The result of the last statement in the branch of the if statement that is executed
will be returned.

{loops}

", loops=match i {
            j if j > 1 => "
#### Loops

Loop syntax is similar to that of if statements. To use them use the `while` keyword.

```

a = 0;
while a < 1000 {
    print(a);
    a = a + 1;
}

```

",
            _ => ""
        }),
        _ => "".into()
    };

    let job = match unsafe { MANUAL_ID } {
        i if i > 2 => "

You will become our main database engineer. Because you are still new here, we will first test your capabilities.
Check this manual periodically as it will update with tests.

",
        _ => "",
    };

    let database = match unsafe{ MANUAL_ID } {
        i if i > 2 => "### Database

Built into the language is a database. This database is regenerated every time you start a program to
avoid accidental corruption.

The database has a single table with columns (in order) <'id': int> <'name': string> <'naughty/nice': boolean>'

To read from the database, you can use the following interface:

```
row = 5;
colum = 10;
db_get_by_id(\"Tablename\", Column, row);
```


",
        _ => ""
    };

    let basics = "### Basics

The santa language supports many common syntax patterns such as
variable assignment, addition (`+`), subtraction (`-`), multiplication (`*`) and division (`/`).

Each statement or expression is terminated with a semicolon (`;`).
Below is an example of a simple Santa program.

```
a = 3 + 5;
b = 8;
print(a + b); // prints 16.
```

Suggestion: It's useful to try out examples given in this manual as they improve your understanding of the subject,
which you will need as our new employee at E.L.F inc.

#### Printing

As you can see in the example above, the print function is used to display information.
The print function accepts 1 argument of any type, and will print it to the standard output.

#### Data representation

The Santa language is dynamically typed. Types are automatically converted. An example of this is division.
Any number, under division, will be converted to a float.

#### Comments

Comments can be added to code by prefixing them with a double slash (`//`)

";

    let functions = match unsafe { MANUAL_ID } {
        i if i > 3 => {
            "### Functions


"
        }
        _ => "",
    };

    mainfile.write_all(format!("

# Welcome to the Santa programming language manual version 1.2.{version}

This programming language is the main system used by E.L.F incorporated.
This system makes lists, checks them twice, and sends them to Santaclaus to see who's naughty or nice, whenever he comes
to town.

You - our new employee - will be learning the ins and outs of this language next week. Read this manual very carefully.

{job}

## Features

{basics}
{blocks}

{database}
{functions}


", version=unsafe{MANUAL_ID},
        basics=basics,
        blocks=blocks,
        job=job,
        database=database,
        functions=functions,

    ).as_bytes()).expect("Couldn't write");
}
