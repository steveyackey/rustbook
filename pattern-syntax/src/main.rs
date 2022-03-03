fn main() {
    let x = 1;
    let y = 'c';

    match x {
        1 | 2 => println!("one or two!"),
        3..=5 => println!("three through five!"),
        _ => println!("anything else!"),
    }

    match y {
        'a'..='j' => println!("a to j"),
        'k'..='z' => println!("late ascii letter"),
        _ => println!("anything else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
