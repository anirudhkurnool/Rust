enum IpAddrVersion {
    IPv4,
    IPv6
}

//enum field can store other data
enum IpAddr {
    IPv4(String),
    IPv6(String),
}

//just like structs instance methods and assosiated functions can be defined on the enum
impl IpAddrVersion {
    fn hello_world() {
        println!("Hello, World");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String)
}

impl Coin {
    fn int_value_of_coin (&self, state : String) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("quarter from state : {}", state);
                25
            }
        }
    }
}

fn optional_plus_one(num : Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn some_value(num : Option<i32>) -> i32 {
    match num {
        Some(i) => i,
        _ => 0//default case
    }
}



fn main() {
    let usr1_ip_vrsn = IpAddrVersion::IPv4;
    let usr2_ip_vrsn = IpAddrVersion::IPv6;

    let usr1_ip = IpAddr::IPv4(String::from("1.1.1.1"));
    let usr2_ip = IpAddr::IPv6(String::from("2.2.2.2"));

    //options enum gives the functionality to make some variable some times null
    let optional_x = Some(2);
    let x = 3;
    //we cannot add optional_x and x
    //let y = optional_x + x; //panic
    //to do the above op
    let y = x + optional_x.unwrap_or(0);//unwrap the optional and if its null use the default that is 0 here
    println!("{}", y);

    let c1 = Coin::Dime;

    println!("value of c1 is {}" , c1.int_value_of_coin(String::from("s1")));

    let num1 = optional_plus_one(optional_x);
    let optional_x_1 = None;
    let num2 = optional_plus_one(optional_x_1);
    let num3 = some_value(optional_x_1);

    if let Some(2) = optional_x_1 {
        println!("..."); //??
    }

}
