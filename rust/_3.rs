fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = 2;
    let b = 4;

    let c = add(a, b);

    c += 1;

    println!("c = {c}");
}
