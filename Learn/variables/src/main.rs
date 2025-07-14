fn main() {
    let x = 32; //immutable 

    //x = 6; //error

    println!("{}",x);

    let mut x = 34;//shadowing

    println!("{}",x);

    const PI : f32 = 3.141;

    println!("{}", PI);

    const READABLE_CONST : i32 = 10_00_00_00;

    println!("{}", READABLE_CONST);

    //Shadowing

    let y = 3;
    println!("y = {}", y);

    let y = "World";
    println!("y = {}", y);

    //integer overflow
    let num : u8 = 255;
    println!("{}", num);

    let float_num = 1.21;

    let c = 'c';

    //tuples 

    let tup1 = ("Hello", 123);
    println!("{:?}", tup1);

    //destructuring a tuple 
    let (tup1_val1, tup1_val2) = tup1;
    println!("{}, {}", tup1_val1, tup1_val2);

    let tup1_val1 = tup1.0;
    let tup1_val2 = tup1.1;
    println!("{}, {}", tup1_val1, tup1_val2);

    //Arrays 
    let arr1 = [1, 2, 3];
    println!("{}", arr1[0]);

    //to create a array of 8 zeros 
    let arr2 = [0; 8];
    println!("{:?}", arr2);

}
