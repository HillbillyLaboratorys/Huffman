use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("C:\\work\\Huffman\\test.txt")
        .expect("File Not Found");

    let mut input = String::new();

    file.read_to_string(&mut input)
        .expect("Error");

    // record frequency of charactors in input
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

    // sort by frequecy
    // let mut sorted = Vec::new();

    // for (_keys, values) in &frequency {
    //     sorted.push(values);
    // }
    // sorted.sort();
    // println!("{:?}", sorted);

    // // create lookup table with huffman codes
    // let mut table = HashMap::new();

    // for (keys, values) in &frequency {
    //     table.insert( keys, sorted.binary_search(&values).expect("miss match of frequency"));
    // }
    // println!("{:?}", table);

    // sort by frequecy

    let mut sorting: Vec<(char, i32)> = Vec::new();

    for (keys, values) in &frequency {
        sorting.push((*keys, *values));
    }

    sorting.sort_by(|a,b| b.1.cmp(&a.1));
    println!("{:?}", sorting);

    // create look up table
    let mut table = HashMap::new();

    for (i, v) in sorting.iter().enumerate() {
        table.insert(v.0, i);
    }
    println!("{:?}", table);

}