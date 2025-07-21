use std::fmt::Display;

struct Rectangle {
    length : i32,
    breadth : i32
}

impl CalcArea for Rectangle{ 
    fn Area(&self) -> i32 {
        self.length * self.breadth
    }
}

struct Square {
    side_length : i32
}

impl CalcArea for Square{
    fn Area(&self) -> i32{
        self.side_length * self.side_length
    }

    fn Display(&self) {
        println!("Square; Side Length : {}", self.side_length);
    }
}

pub trait CalcArea {
    fn Area(&self) -> i32;
    
    fn Display(&self) {
        //default implementation
        println!("default : cannot print but Area : {}", self.Area());
    }
}

fn Shape_Area_Calculator(shape : &impl CalcArea) { 
    println!("Shape Area : {}", shape.Area());
}

//the above fn is syntactic sugar for

fn Shape_Area_Calculator_1<T: CalcArea>(shape : &T) {
    println!("Shape Area : {}", shape.Area());
}

/*
fn Shape_Or_Rectangele(square : bool) -> impl CalcArea() {
    //this will panic in a function only one type which implements CalcArea can be returned
    //not two unlike Here
    if (square == true) {
        return Square{side_length : 3};
    } else {
        return Rectangle{ length : 3, breadth : 4};
    }
}
*/ 

//trait bounds 

struct Square1<T>{
    side_length : T
}

impl<T : std::ops::Mul<Output = ()> + Copy> Square1<T> {
    fn Area(&self)  {
        self.side_length * self.side_length
    }
}

//trait bound 
//impl block for type which implement Display 

impl<T : Display + std::ops::Mul<Output = ()> + Copy> Square1<T> {
    fn Area1(&self) {
        self.side_length * self.side_length
    }
}



fn main() {
    let r1 = Rectangle{length : 2, breadth : 3};
    let s1 = Square{side_length : 3};
    println!("Sqaure Area : {}", s1.Area());
    s1.Display();
    println!("Rectangel Area : {}", r1.Area());
    r1.Display();
    Shape_Area_Calculator(&s1);
    Shape_Area_Calculator_1(&r1);
} 
