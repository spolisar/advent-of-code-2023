use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Read input file
// for each line, take the first and last digit
//   from those digits form a two digit number '1' '2' -> '12'
// sum those values

fn main() {
    let mut total = 0;
    let filename = "input.txt";
    //check and show where we're running
    println!("{}", std::env::current_dir().unwrap().display());
    println!("{}", Path::new(filename).exists());
    if let Ok(lines) = read_lines(filename) {
        //consumes iterator, returns (Optional String)
        for line in lines {
            //Ok ensures that we have a valid string to work with
            if let Ok(text) = line {
                //get index of first digit and last digit of line
                let first_ind = text.find(char::is_numeric);
                let last_ind = text.rfind(char::is_numeric);

                if first_ind.is_some() {
                    let mut num_str = String::with_capacity(2);
                    let first = text.chars().nth(first_ind.unwrap()).unwrap();
                    let last = text.chars().nth(last_ind.unwrap()).unwrap();
                    num_str.push(first);
                    num_str.push(last);
                    let num: i32 = num_str.parse().unwrap();
                    total += num;
                }
                
            }
        }
    }
    println!("the sum of the constructed digits is {}", total)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}