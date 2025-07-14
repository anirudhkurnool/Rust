fn add(a : i32, b : i32) -> i32{
    a + b //naked return
}

fn mul(a : i32, b : i32) -> i32 {
    return a * b;
}

fn even_or_odd(num : i32) {
    if num % 2 == 0{
        println!("Even");
    } else{
        println!("Odd");
    }
}

fn is_even(num : i32) -> bool {
    if num % 2 == 0 {
        return true
    } else {
        return false
    }
}

fn loop_demo() {
    let mut num = 0;
    loop {
        if num == 10 {
            break;
        }

        println!("{}", num);
        num += 1;
    }
}

fn while_demo() {
    let mut num = 1;
    while num < 10{
        println!("{}", num);
        num += 1;
    } 
}

fn for_demo() {
    let arr1 = [1, 2, 3, 4, 5];

    for i in arr1.iter() {
        println!("{}", i);
    }

    for i in 1..10{
        //range from 1 to 9
        println!("{}", i);
    }
}

fn main() {
    /*
    
        multi line
        comment    
     */
    println!("{}", add(1, 2));
    println!("{}", mul(2, 3));
    even_or_odd(2);
    even_or_odd(3);

    let num = if is_even(2*3 + 5) { 2 * 3 + 5 } else { 5 }; 
    println!("num = {}", num);
    loop_demo();
    while_demo();
    for_demo();
}
