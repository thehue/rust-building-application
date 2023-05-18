fn main() {
    let mut mars_weight = caculate_weight_on_mars(100.0);
    mars_weight *= 1000.0;
    println!("Weight on Mars: {}g", mars_weight)
}

fn caculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
