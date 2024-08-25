fn main() {
    // read the file from the file system 
    let contents = std::fs::read_to_string("./src/assets/done1.txt")
        .expect("Something went wrong reading the file");

    // initalize the sum
    let mut sum = 0; 

    // iterate over the lines of the file
    for line in contents.lines() {
        let mut firstnumber = 0;
        let mut secondnumber = 0;
        for character in line.chars() {
            if character.is_digit(10) {
                firstnumber = character.to_string().parse::<i32>().unwrap();
                break;
            }
        }
        // iterate from end to start of the line
        for character in line.chars().rev() {
            if character.is_digit(10) {
            secondnumber = character.to_string().parse::<i32>().unwrap();
            break;
            }
        }

        // perform some operations using irstnumber and secondnumber
        // let result: String = firstnumber.to_string() + &secondnumber.to_string();
        // using &secondnumber.to_string() because the + operator does not support String + String but rather String + &String so it is not neccessary to use & on the firstnumber but it is necessary to use & on the secondnumber 


        // a better way to format the result is to use the format! macro
        let result = format!("{}{}" , firstnumber, secondnumber);
        sum += result.parse::<i32>().unwrap();
    }

    // print the sum
    println!("{sum}"); 
}
