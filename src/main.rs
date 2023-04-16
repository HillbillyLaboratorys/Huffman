use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("C:\\work\\Huffman_comp\\test.txt").expect("File Not Found");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Error");

    let mut frequency = HashMap::new();

    let mut current;

    for i in input.chars() {

        if frequency.contains_key(&i) {
            current = frequency.get(&i).unwrap();
            frequency.insert(i, current + 1);
        }
        else {
            frequency.insert(i, 1);
        }
    }
    println!("{:?}", frequency);
}