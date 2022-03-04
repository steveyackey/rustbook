use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("Hello, world!");

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // let address = 0x012345usize;
    // let r = address as *const i32;
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 6;
        println!("r2 is: {}", *r2);

        dangerous();

        println!("Abs of -3 according to C: {}", abs(-3));
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

unsafe trait Foo {}

unsafe impl Foo for i32 {}
