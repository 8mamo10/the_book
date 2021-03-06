use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    //panic!("crash and burn");
    let v = vec![1, 2, 3];
    //v[99];

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let f = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });

    //let f = File::open("hello3.txt").unwrap();
    //let f = File::open("hello3.txt").expect("Failed to open hello3.txt");

    let s = read_username_from_file();
    let name = match s {
        Ok(s) => s,
        Err(_) => String::from("XXX"),
    };
    //println!("username is {:?}", s);
    println!("username is {:?}", name);
    let s = read_username_from_file2();
    println!("username is {:?}", s);
    let s = read_username_from_file3();
    println!("username is {:?}", s);
    let s = read_username_from_file4();
    println!("username is {:?}", s);

    let g = Guess::new(1);
    println!("guess is {}", g.value());

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello4.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello4.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello4.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello4.txt")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value mast be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
