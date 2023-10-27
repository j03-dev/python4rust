fn main() {
    let ages = vec![14, 19];

    for age in ages {
        if age >= 18 {
            println!("{age} is Major");
        } else {
            println!("{age} is Minor");
        }
    }
}
