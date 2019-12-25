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

pub const NAME: &'static str = "Tim Anema";

pub const BASICS: usize = 0;
pub const CONDITIONALS: usize = 1;
pub const LOOPS: usize = 2;
pub const DATABASES: usize = 3;
pub const DATABASES_TEST2: usize = 4;
pub const DATABASES_TEST3: usize = 5;
pub const FUNCTIONS: usize = 6;
pub const NETWORKING_1: usize = 7;
pub const NETWORKING_2: usize = 8;
pub const FINISHED: usize = 9;

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

    if unsafe{MANUAL_ID} == FINISHED {
        webbrowser::open("https://www.napkinshop.co.uk/wp-content/uploads/2019/07/423560.jpg");
        println!("High Tech! you finished the secret santa challenge! Presents coming soon!");
        return;
    }

    println!("{}", "High Tech! You have advanced to the next version of the Santa programming language. Check your manual!".red());
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
        i if i >= CONDITIONALS => format!("### Complex expressions

#### Booleans

Booleans can be used in combination with high tech complex expressions (if/while) to create control flow.

Booleans can have the high tech values `true` and `false` and will act like the integers `1` and `0` respectively in mathmatical operations.
The `-` operator to negate a number (`5` --> `-5`) is used to invert booleans.

#### Comparison

You can compare numbers and other data types using the following high tech operators:

```
a == b;
a != b;
a >= b;
a <= b;
a > b;
a < b;
```

You can compare floats and integers together. Watch out for floating point errors! Booleans act as the integers
0 and 1 under comparison with an integer. Strings can be compared for equality only.

### Conditionals

Using the comparison operators, you can now build programs that conditionally execute code.
This is done with an if statement. A high tech example of if statements is given here:

```
if a > b {{
    print(\"a greater than b\");
}} else {{
    print(\"a not greater than b\");
}}

```

If statements can be used as ternary operators by assigning them to a variable.
The result of the last statement in the branch of the if statement that is executed
will be yeeted back.

{loops}",
loops=match i {
    j if j >= LOOPS => "
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


    let mut job = String::new();

    if unsafe { MANUAL_ID } >= CONDITIONALS {
        job.push_str("

E.L.F inc is obliged to tell you that if at any point you find a bug in the high tech language, it's not a bug but a feature and you should simply ignore it.
Failure to do so will get you personally yeeted out of the company by Santa.

");
    }
    if unsafe { MANUAL_ID } >= DATABASES {
        job.push_str("

When you finish your training, you will become our lead database engineer.
But first we will test your capabilities a bit.
Check the Tests section as it will update with your new challenges.

");
    }
    if unsafe { MANUAL_ID } >= FINISHED {
        job.push_str(&format!("

=================================================================================

High tech! you have completed all tests! you are hired as our new database engineer.
And with that, {}, Santa wishes you a merry christmas. Presents coming soon :tm:

=================================================================================

", NAME));
    }

    let database = match unsafe{ MANUAL_ID } {
        i if i >= DATABASES => "### Database

Built into the language is a database. This database is regenerated every time you start a program to
avoid accidental corruption.

To read from the database, you can use the following high tech interface:


#### Get data by column value

```
db_get(\"colname\", \"colvalue\"); // this would get the first record where colname == colvalue
```

#### Set data by column value

```
// this would get the first record where colname == colvalue,
// and set the column identified by \"newcolname\" to newvalue
db_set(\"colname\", \"colvalue\", \"newcolname\", \"newvalue\");
```

#### Get column names

```
db_columns() // yeets back a list of column names
```


#### Get number of records

```
db_records() // yeets back an integer counting the number of records in the table
```

#### Get all records

```
db_get_all() // yeets back a list of all records in the table. A record is a list of values.
```
",
        _ => ""
    };

    let basics = format!("### Basics

The santa language supports many common high tech syntax patterns such as
variable assignment, addition (`+`), subtraction (`-`), multiplication (`*`) and division (`/`).

Each statement or expression is terminated with a semicolon (`;`).
Below is an example of a simple Santa program.

```
a = 3 + 5;
b = 8;
print(a + b); // prints 16. High tech!
```

Suggestion: It's useful to try out examples given in this manual as they improve your understanding of the subject,
which aid you in becoming our new engineer at E.L.F inc.

#### Printing

As you can see in the example above, the print function is used to display information.
The print function accepts any number of arguments of any type, and will print them to the standard output.

#### Data representation

The Santa language is dynamically typed. Types are automatically converted. An example of this is division.
Any number, under division, will be converted to a float.

#### Comments

Comments can be added to code by prefixing them with a double slash (`//`) or by enclosing the code with `/* ... */`

#### Built in functions

##### len

len gets the length of a datatype. This operation is defined for strings, lists and maps.

```
print(len([1,2,3]));
print(len(\"High Tech\"));
```

{assertion}
#### Datatypes

The datatypes available in the Santa language are:

* String{boolean}{function}
* List
* Map
* Integer
* Float
* None

##### Strings

Strings can be created with single or double quotes surrounding any text.
Strings can be indexed by an integer and yeet back the character at that position. Characters are strings of length 1.
An object, string or other, can be appended to a string by using the `+` operator.

##### Maps

A Map can be created by using the following syntax: `{{key: value, key: value, ...}}`. Keys can be any type apart from lists and maps themselves as they are mutable.
Indexing a map with a key yeets back it's value.

example:

```
a = {{\"a\":3, 4:\"x\"}};

print(a[\"a\"]);

```

##### Lists

A List is made using the following syntax: `[1,2,3,4]`. Indexing a List with an integer index yeets back the item at this index.
Lists can be concatenated using the `+` operator and repeated using the `*` operator.


",
        boolean = if unsafe { MANUAL_ID } >= CONDITIONALS {"\n* Boolean"} else {""},
        function = if unsafe { MANUAL_ID } >= FUNCTIONS {"\n* Function"} else {""},
        assertion = if unsafe {MANUAL_ID} >= DATABASES_TEST2 {"
##### assert

The Santa language includes a high tech testing framework by including the `assert` function.
The assert function requires one boolean parameter, and the program is exited with an AssertionError
if this value is false.

```

assert(1 == 1); // passes
assert(1 == 2); // fails

```

"} else {""},
    );

    let functions = match unsafe { MANUAL_ID } {
        i if i >= FUNCTIONS => {
            "### Functions

The santa language pioneers a high tech feature it calls \"functions\".
You make a function using the `function` keyword:

```
function a(x) {
    yeet x + 1 back;
}

assert(a(3) == 4);

```

Alternatively, you can use functions inline like this:

```
a = function(x) {
    yeet x + 1 back;
};

assert(a(3) == 4);
```

A variable number of parameters can be specified by prefixing the last parameter with a `*` like this:

```

function sum(*values) {
    length = len(values);
    total = 0;
    index = 0;

    while index < length {
        total = total + values[index];
        index = index + 1;
    }

    yeet total back;
}

assert(sum(1,2) == 3);
assert(sum(1,2,3) == 6);
assert(sum() == 0);

```

Functions can be nested and form closures over their outer scope. High tech!

"
        }
        _ => "",
    };


    let mut tests = String::new();

    if unsafe {MANUAL_ID} >= DATABASES {
        tests.push_str("
## Tests

In this section, we will give you some tests. When you completed them all you will be accepted as our new database engineer at E.L.F inc.

### Test 1: The naughty ones

Find the number of naughty people in santa's database. Print this value.

");

    };
    if unsafe {MANUAL_ID} >= DATABASES_TEST2 {
        tests.push_str("

### Test 2: Test your tests!

Assert that the answer to the previous test was 12 with the assert keyword.

");
    }
    if unsafe {MANUAL_ID} >= DATABASES_TEST3 {
        tests.push_str("

### Test 3: Who's the naughty one here!

As our new employee, Santa is prepared to pardon the numerous major crimes you committed
(eg. saying High Tech more than 5 times a  week).

Change your own status in the database from naughty to nice.

");
    }
    if unsafe {MANUAL_ID} >= FUNCTIONS {
        tests.push_str("

### Test 4: A better test framework

With your new knowledge of functions, create a new function called `assert_eq` which takes two parameters
and uses the built-in assert function to assert their equality. The function should always yeet back the value `42`.
");
    }
    if unsafe {MANUAL_ID} >= NETWORKING_1 {
        tests.push_str("

### Test 5: What's that I hear?

Listen carefully! Santa is sending you some messages over the internet. Register a network handler and look at the data coming in.
");
    }
    if unsafe {MANUAL_ID} >= NETWORKING_2 {
        tests.push_str("

### Test 6: The final countdown!

This is your final test. It's the hardest one sofar. Listen to the incoming messages from Santa.
Parse the queries and update your database accordingly.

Suggestion: Creating something like a string split method and a string to int method might be useful for this.
");
    }

    let mut networking = String::new();

    if unsafe {MANUAL_ID} >= NETWORKING_1 {
        networking.push_str("

### Networking

To support updates from Santa, the Santa language also supports networking. To use this feature, simply register a function:

```

register_network_handler(function (data){
    print(data);
});

```

After you registered a network hadler, you can call `listen()` to stop and wait for incoming messages.

");
    }

    mainfile.write_all(format!("

# Welcome to the Santa programming language manual version 1.2.{version}

This programming language is the main system used by E.L.F incorporated.
This system makes lists, checks them twice, and sends them to Santa Claus to see who's naughty or nice, whenever he comes
to town.

You - our new trainee - will be learning the ins and outs of this language next week. Read this manual very carefully.

{job}

## Features

{basics}
{blocks}

{database}
{functions}
{networking}

{tests}

", version=unsafe{MANUAL_ID},
        basics=basics,
        blocks=blocks,
        job=job,
        database=database,
        functions=functions,
        tests=tests,
        networking=networking,
    ).as_bytes()).expect("Couldn't write");
}
