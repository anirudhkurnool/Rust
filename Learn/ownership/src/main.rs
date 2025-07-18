fn takes_ownership(s : String) { 
    println!("{}", s);
}

fn takes_ownership1(s : String) -> String {
    println!("{}", s);
    return s;
}

fn reference_demo(s : &String) {
    println!("{}", s);
}

fn mutable_reference_demo(s : &mut String) {
    s.push_str("World");
    println!("{}", s);
}


fn main() {
    println!("Hello, World");

    let x = 1;
    let y = x;//deep copy
    //for scalar data types a deep copy happens automatically without any problems 
    //to be more precise for the types which implement the copy trait a deep copy happens
    //automatically  

    println!("{}", x);
    println!("{}", y);

    let str1 = String::from("Hello");
    let str2 = str1; //here the onwership of the data of str1 is "moved" to str2.
    //thus str1 is invalidated and cannot be used further 
    
    //println!("{}", str1);//should panic
    println!("{}", str2);

    //to avoid the above phenomenon by doing a deep copy we can write
    let str3 = str2.clone();
    println!("{}", str3);

    takes_ownership(str3);
    //whenever a variable whose type does not implement the copy trait is passed to a function
    //directly its ownership is moved to the functions parameter and thus once the function execution is done
    //that variable goes out of scope and the value is discarded
    //println!("{}", str3); // Should panic

    //to avoid the above phenomenon the function can the return the value back
    
    let mut str3 = str2.clone();

    str3 = takes_ownership1(str3);
    println!("{}", str3);

    //another way to avoid the above phenomenon is to pass the value by reference(borrowing)
    //reference are immutable by default i.e. reference_demo can't change the value of the str3
    reference_demo(&str3);
    println!("{}", str3);

    let mut str4 = str3.clone();
    mutable_reference_demo(&mut str4);
    println!("{}", str4);


    //there can only be one mutable reference on a value in a scope
    //there can be any number of immutable reference on a value 
    //we cannot have a mutable reference if a immutable reference already exists in a scope 
    //
    //we can have a mutable refernce once immutable reference is out of scope 
    //
    
    let mut str5 = str4.clone();
    let imt_ref_str5 = &str5;
    let imt_ref_str5_1 = &str5;

    println!("{} ,  {}", imt_ref_str5, imt_ref_str5_1);// scope of immutable refs end 
    //
    // so a mutable ref is possible now 
    //
    let mut mut_ref_str5 = &mut str5;
    mut_ref_str5.push_str(" ..Worlds???");

    println!("{}", str5);

    //slices 
    //
    let str6 = String::from("Hello, Galaxy");
    let str7 = &str6[..6];
    let str8 = &str6[7..];
    println!("{}, {}", str7, str8);
}
