use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Option<String> {
    match File::open(path) {
        Ok(mut file) => {
            let mut out_put = String::new();
            file.read_to_string(&mut out_put);
            Some(out_put)
        }
        Err(_) => None,
    }
}

fn main() {
    if let Some(result) = read_file("/mnt/d/Project/devfest/rust/_1.rs") {
        println!("{result}");
    } else {
        println!("nothing");
    }
}
