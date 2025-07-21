#[derive(Debug)]
struct HPoint<T> {
    x : T,
    y : T
}

impl<X> HPoint<X> {
    fn do_smthg(&self) -> &X {
        &self.x
    }
}

impl HPoint<f64> {
    //methods only for Point of f64 type
    fn do_smthg1(&self) -> f64 {
        self.x
    }

}

#[derive(Debug)]
struct HtPoint<T, U> {
    x : T,
    y : U
}


fn largest<T : PartialOrd + Copy>(v : Vec<T>) -> T {
    let mut largest = v[0];
    for i in &v {
        if *i > largest {
            largest = *i;
        }
    }

    largest

}

fn main() {
    println!("Hello, world!");
    let v = vec![5, 1, 12, 42, 12, 31];
    println!("{}", largest(v));

    let v1 = vec!['b', 'c', 'q', 'a'];
    println!("{}", largest(v1));

    let p1 = HPoint{x : 2, y : 3};
    println!("{:?}", p1);

    let p2 = HPoint{x : 3.1, y : 2.31};
    println!("{:?}", p2);

    let p3 = HtPoint { x : 1, y : 2.1231};
    println!("{:?}", p3);
}
