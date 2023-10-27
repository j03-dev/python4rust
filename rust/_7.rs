#[derive(Debug)]
struct Person {
    name: String,
    prenom: String,
    age: i32,
}

impl Person {
    fn new(name: &str, prenom: &str, age: i32) -> Self {
        Self {
            name: name.into(),
            prenom: prenom.into(),
            age,
        }
    }

    fn get_age(&self) -> i32 {
        self.age
    }

//     fn grow(&self) {
//         self.age += 1
//     }
}

fn main() {
    let person = Person::new("FITAHIANA", "Nomeniavo", 22);
    println!("{person:?}");

//     person.grow();
// 
//     println!("{person:?}");
}
