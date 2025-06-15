fn even_or_odd(num: i32) -> String {
    if num % 2 == 0 {
        String::from("even")
    } else {
        String::from("odd")
    }
}

fn is_prime(num: i32) -> bool {
    let end: i32 = (num as f64).sqrt().ceil() as i32;
    for i in 2..=end {
        if num % i == 0 {
            return true;
        }
    }
    false
}

fn sieve_of_eratosthenes(num: usize) -> Vec<i32> {
    let mut bool_arr: Vec<bool> = vec![true; num];
    //bool_arr.fill(true);
    // for i in 0..num {
    //     bool_arr.push(true)
    // }

    bool_arr[0] = false;
    let mut res: Vec<i32> = Vec::new();
    for i in 1..num {
        if bool_arr[i] == true {
            let mut j: usize = 2 * i + 1;
            while j < num {
                bool_arr[j] = false;
                j += i + 1
            }

            res.push((i + 1) as i32);
        }
    }

    return res;
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: i32, b: i32) -> i32 {
    assert!(b != 0);
    a / b
}

fn main() {
    println!("Hello, World!!!");
    println!("1 + 2 = {}", add(1, 2));
    println!("2 - 3 = {}", sub(2, 3));
    println!("3 * 4 = {}", mul(3, 4));
    println!("5 / 2 = {}", div(5, 2));
    println!("3 is {}", even_or_odd(3));
    println!("4 is {}", even_or_odd(4));
    println!("is 3 prime : {}", is_prime(3));
    println!("is 4 prime : {}", is_prime(4));
    println!("{:?}", sieve_of_eratosthenes(100));
}
