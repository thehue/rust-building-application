use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input);

    borrow_string(&input);
    own_string(input);

    // println!("Input: {}", input);
    // let mut mars_weight = caculate_weight_on_mars(100.0);
    // println!("Weight on Mars: {}g", mars_weight)
}

fn borrow_string(s: &String) {
    println!("{}", s);
}

fn own_string(s: String) {
    println!("{}", s);
}

fn caculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
