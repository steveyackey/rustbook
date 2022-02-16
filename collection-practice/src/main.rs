mod employee;

fn main() {
    println!("Hello, world!");
    let test = [0, 1, 2, 2];
    mode(&test);
    median(&test);

    let mut depts = employee::new_department_list();
    employee::parse_command( &mut depts, String::from("Add Steve to Platform"));
    let emp = depts.get("Platform").unwrap();
    println!("{}", emp.get(0).unwrap());
}

fn median(ints: &[i32]) {
    let mut sorted = Vec::new();
    sorted.extend_from_slice(ints);
    println!("median: {}", sorted[sorted.len() / 2]);
}

use ::std::collections::HashMap;
fn mode(ints: &[i32]) {
    let mut map = HashMap::new();

    for i in ints {
        let mut count = map.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut max_k = 0;
    let mut max_v = 0;
    for (k, v) in &map {
        if v > &max_v {
            max_v = *v;
            max_k = *k;
        }
        println!("{} {}", max_k, max_v);
    }
    println!("mode k: {}, max v {}", max_k, max_v);
}

