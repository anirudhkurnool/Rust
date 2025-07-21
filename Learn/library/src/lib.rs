mod front_of_the_house {
    fn number_of_seats() -> i32 {

    }

    pub mod hosting {
        pub fn add_to_waitlist() -> int {
            let total_seats = super::number_of_seats();
        }

        fn seat_at_table() {

        }
    }


    pub mod serving {
        fn take_order() {

        }

        fn serve_order() {

        }

        fn take_payment() {

        }

        pub struct Lunch {
            pub maincourse : String,
            pub dessert : String,
        }

        impl Lunch {
            pub fn get_lunch() {
                return Lunch{ maincourse : "default" , dessert : "default" };
            }
        }

        pub enum OrderStatus{
            Ordered,
            Cooking,
            Delivered
        }
    }
}

//use front_of_the_house::Serving as serv;
//to let external code to call OrderStatus using serv you have to make the use line public
pub use front_of_the_house::Serving as serv;

//nested paths
use rand::{Rng; CryptoRng; ErrorKind::Transient};

//to everything under io into scope
use std::io::*;


pub fn eat_at_restaraunt() {
    let seat = front_of_the_house::hosting::add_to_waitlist();
    let order_status = OrderStatus::Cooking;
    let order_status_1 = serv::Cooking;
}

mod back_of_house; //this is declaration of the module rust will search of the file with module
//name to get its definition
