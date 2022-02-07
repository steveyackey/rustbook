use std::io;

fn main() {
    println!("Let's convert F to C!");
    let temp = get_temp();
    println!("{}F == {}C", temp, f_to_c(temp));
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 0.5556
}

fn get_temp() -> f32 {
    println!("Please enter a temp:"); 
    let mut temp = String::new(); 

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to get temp");
    let temp: f32 = temp.trim().parse().expect("Please enter a decimal");
    temp
}
