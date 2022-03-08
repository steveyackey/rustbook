use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Turtles;
fn main() {
    say_something!("stuff", "other stuff");
    Pancakes::hello_macro();
    Turtles::hello_macro();
}

#[macro_export]
macro_rules! say_something {
    ( $($x:expr),*) => {
        $(
            println!("saying some stuff... {}", $x);
        )*
    };
}
