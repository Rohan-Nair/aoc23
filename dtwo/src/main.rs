fn main() {
   // reading the contents of the input file
   // let contents = std::fs::read_to_string("./src/assets/dtwotest.txt").expect("Something went wrong while reading the file");

   // the above method reads the contents of the file at runtime but the below method reads the contents of the file at compile time.   
   const CONTENTS: &str= include_str!("assets/dtwo.txt");
   
   let res: i32 = CONTENTS.lines().map(|line| {
    let line = line.strip_prefix("Game ").unwrap();
    let (num, single_game) = line.split_once(": ").unwrap();
    let num = num.parse::<i32>().unwrap();
    let is_possible = single_game
        .split("; ")
        .map(|each_pair| 
            {
                each_pair.split(", ")
                .map(|pair| 
                {
                    let (count, colour) = pair.split_once(" ").unwrap();
                    (count.parse::<i32>().unwrap(), colour)
                })
                .collect::<Vec<_>>() 
            }).all(|subset| 
            {
                subset.iter().all(|&(count, colour)| 
                {
                    count
                            <= match colour {
                                "red" => 12,
                                "green" => 13,
                                "blue" => 14,
                                _ => unreachable!(),
                            }
                    })
                    
            }); 
    if is_possible {
        num
    } else {
        0
    }
   }).sum();

   println!("{}", res);

}
