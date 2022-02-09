#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };


    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can1 hold 2? {}", rect1.can_hold(&rect2));
    println!("Can1 hold 3? {}", rect1.can_hold(&rect3));


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?}", rect1);

    dbg!(&rect1);

    let rect4 = Rectangle::square(20);
    dbg!(&rect4);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area_tuples(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


