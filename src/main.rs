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

        if frequency.contains_key(&i.to_string()) {
            current = frequency.get(&i.to_string()).unwrap();
            frequency.insert(i.to_string(), current + 1);
        }
        else {
            frequency.insert(i.to_string(), 1);
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

    // let mut sorting: Vec<(char, i32)> = Vec::new();

    // for (keys, values) in &frequency {
    //     sorting.push((*keys, *values));
    // }

    // sorting.sort_by(|a,b| b.1.cmp(&a.1));
    // println!("{:?}", sorting);

    let mut node_list: Vec<Node> = Vec::new();

    // create tree

    for (i, j) in &frequency {
        node_list.push(Node::new(i, *j));
    }

    let mut left: Node;
    let mut right: Node;
    let mut sum: i32;

    while node_list.len() > 1 {
        node_list.sort_by(|a,b| a.freq.cmp(&b.freq));

        left = node_list.pop().unwrap();
        right = node_list.pop().unwrap();
        sum = left.freq + right.freq;

        node_list.push(Node::new("", sum).left(left).right(right));
    }

    // // create look up table
    // let mut table = HashMap::new();

    // for (i, v) in sorting.iter().enumerate() {
    //     table.insert(v.0, i);
    // }
    // println!("{:?}", table);

}
pub struct Node<'a> {
    symbol: &'a str,
    freq: i32,
    l: Option<Box<Node<'a>>>,
    r: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn new(symbol: &'a str, freq: i32) -> Self {
        Node { symbol: (symbol), freq: (freq), l: (None), r: (None) }
    }

    pub fn left(mut self, node: Node<'a>) -> Self {
        self.l = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: Node<'a>) -> Self {
        self.r = Some(Box::new(node));
        self
    }
}