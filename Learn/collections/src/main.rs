use std::collections::HashMap;

fn main() {
    let arr = [1, 2, 3];
    let mut v1 : Vec<i32> = Vec::new();

    v1.push(1);
    v1.push(2);
    v1.push(3);

    println!("{:?}", v1);

    let mut v2 : Vec<i32> = vec![2, 4, 6];//vec! is a macro
    println!("{:?}", v2);

    println!("v2[1] = {}", v2[1]);

    let default_null_element = -1;

    let second_element_of_v2 = match v2.get(2) {
        Some(v) => v,
        None => &default_null_element
    };

    println!("{}", second_element_of_v2);

    let third_element_of_v2 = match v2.get(3) {
        Some(v) => v,
        None => &default_null_element
    };

    println!("{}", third_element_of_v2);

    //iterating through a vector

    for i in &v2{
        println!("{}",i);
    }

    //iterating a mutating a element in a vector
    //


    for i in &mut v2 {
        *i *= 2;
    }
    
    println!("--------------------------");

    for i in &v2{
        println!("{}",i);
    }

    // a work around to store hetergenous data in a vector 
    
    #[derive(Debug)]
    enum HeteroGeneousData {
        Int(i32),
        Str(String),
        Float(f32),
    }

    let heterogeneous_vector = vec![ HeteroGeneousData::Int(1), HeteroGeneousData::Str(String::from("Hello")), HeteroGeneousData::Float(3.14)];

    println!("{:?}", heterogeneous_vector);

    //String vs string literal
    let string_literal = String::from("Hello").to_string();
    println!("{}",string_literal);

    //string concatenation

    let s1 = String::from("Hello");
    let s2 = String::from("World");

    let s = s1 + " , " + &s2;
    //the data of s2 is copied to s1
    //the data of s1 is moved to s and thus are invalidated
    println!("{}", s);
    //println!("{}", s1);//should panic
    //another way where s1 and s2 are not invalidated is 
    //

    let s1 = String::from("Hello");
    let s2 = String::from("World");

    let s = format!("{} , {}", s1, s2);
    //format macro does not take the ownership of s1 and s2
    //
    println!("{}", s);

    //to iterate over a string literal
    for c in "hello".chars(){
        println!("{}", c);
    }

    for c in s2.chars(){
        println!("{}", c);
    }
   
    let mut h1 = HashMap::new();
    h1.insert('A', 1);
    h1.insert('B', 2);
    h1.insert('C', 3);
    
    //ownership of both key and value is moved to the hashmap

    println!("{:#?}", h1);

    match h1.get(&'A') {
        Some(v) => println!("{}", v),
        None    => println!("key not present")
    };

    h1.entry('A').or_insert(2);//if 'A' is already do nothing else insert ('A', 1)
}
