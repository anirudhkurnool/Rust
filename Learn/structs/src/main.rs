#[derive(Debug)]//this makes it possible for us pretty print a instance of this struct
struct User {
    username      : String,
    email         : String,
    active        : bool,
    sign_in_count : u8
}

#[derive(Debug)]
struct Rectangle {
    length  : u32,
    breadth : u32
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.length * self.breadth
    }
}

impl Rectangle {
    fn square(side_length : u32) ->  Rectangle {
        Rectangle { length: side_length, breadth: side_length }
    }
}

fn area(rect : &Rectangle) -> u32 {
    rect.length * rect.breadth
}

fn main() {
    println!("Hello, World");

    let usr1 = User{ username : String::from("john_doe"), email : String::from("john_doe@example.com"), active : true, sign_in_count : 1};

    //you can pretty print a struct 
    println!("{:?}", usr1);
    println!("{:#?}", usr1);

    //accessing fields of the user object 
    println!("username      : {}", usr1.username);
    println!("email         : {}", usr1.email);
    println!("active        : {}", usr1.active);
    println!("sign_in_count : {}", usr1.sign_in_count);

    let usr2 = User{
        email : String::from("john_doe1@example.com"),
        ..usr1
    }; //rest of the field are moved from usr1 and usr1 gets invalidated here

    //println!("{:#?}", usr1); // should panic


    //tuple structs 
    struct Color(i32, i32, i32);
    struct Point(i32, i32 ,i32);

    let rect1 = Rectangle{ length : 3, breadth : 4};
    println!("{:#?}", rect1);
    println!("area of rect1 = {}", area(&rect1));
    println!("area of rect1 = {}", rect1.area());

    let sq1 = Rectangle::square(3);//assosiated functions should be called using :: 
}
