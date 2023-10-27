fn introduce(name: String) {
    println!("My name is {name}");
}

fn main() {
    let name: String = "Joe".to_string();
    introduce(name); // owner 
//     introduce(name);
}
