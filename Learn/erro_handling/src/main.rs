use::std::fs::{self, File};
use::std::io::{ self, Read, ErrorKind};


fn read_file_best_way() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn read_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello2.txt")?.read_to_string(&mut s);
    Ok(s)
}

fn main() {
    //panic!("PANIC!!!");
    
    let file = File::open("hello.txt");
    match file {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(err) => panic!("Problem creating a file : {:?}", err)
            }

            other_error => {
                panic!("Problem opening file {:?}", other_error);
            }
        }
    };

    //you can do the above things using expect method
    let f2 = File::open("hello1.txt").expect("file not found");

    read_file();
    let f3 = read_file_best_way();
}
