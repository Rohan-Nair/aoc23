use std::fs; 
fn main() {
    // read the file from the file system 
    let contents = fs::read_to_string("./src/assets/done1.txt")
        .expect("Something went wrong reading the file");


    println!("With text:\n{contents}");

    
}
