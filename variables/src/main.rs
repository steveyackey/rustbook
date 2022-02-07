use std::io;

fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    let x = 6;
    println!("The value of x is {}", x);

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {}", x);
    }

    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, z) = tup;

    println!("The value of y is {}", y);
    println!("The value of z is {}", z);

    let a = [1, 2, 3, 4, 5];
    println!("{}", a[2]);
    let a: [i32; 5] = [0,1,2,5,3];
    println!("The value of a.1 = {}", a[1]);

    // Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
