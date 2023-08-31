use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_PATH: &str = "src/cherry.txt";
fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file").to_lowercase();
    println!("First cherry: {:?}", contents.find("cherry").expect("Couldn't find first occurrence"));
    println!("Second cherry: {:?}", contents.match_indices("cherry").collect::<Vec<_>>()[1].0);
    println!("Third cherry: {:?}", search_vector(txt_to_array(FILE_PATH), "cherry")[2].expect("Couldn't find third occurrence"));
    println!("Fourth cherry: {:?}", contents.rmatch_indices("cherry").collect::<Vec<_>>()[1].0);
    println!("Last cherry: {:?}", contents.rfind("cherry").expect("Couldn't find last occurrence"));
}

fn txt_to_array(file_path: &str) -> Vec<Vec<char>> {
    let reader = BufReader::new(File::open(file_path).unwrap());
    let mut array: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let char_line: Vec<char> = line.unwrap().to_lowercase().chars().collect();
        array.push(char_line);
    }
    return array;
}

fn search_vector(vector: Vec<Vec<char>>, sequence: &str) -> Vec<Option<(usize, usize)>> {
    let mut result: Vec<Option<(usize, usize)>> = Vec::new();
    for (i, vec) in vector.iter().enumerate() {
        let windows: Vec<&[char]> = vec.windows(sequence.len()).collect();
        for (j, window) in windows.iter().enumerate() {
            if window.iter().collect::<String>() == sequence {
                result.push(Some((j+1, i+1)));
            }
        }
    }
    return result;
}