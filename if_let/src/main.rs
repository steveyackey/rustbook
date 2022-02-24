fn main() {
    let config_max = Some(4u8);
    if let Some(max) = config_max {
        println!("The max is configured to be {}", max);
    } else {
        println!("The max is not configured.");
    }

}
