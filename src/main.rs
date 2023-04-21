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
    //println!("{:?}", frequency);

    // create tree

    let mut node_list: Vec<Node> = Vec::new();

    for (i, j) in &frequency {
        node_list.push(Node::new(i.to_string(), *j));
    }

    let mut left: Node;
    let mut right: Node;
    let mut sum: i32;

    while node_list.len() > 1 {
        node_list.sort_by(|a,b| a.freq.cmp(&b.freq));

        left = node_list.pop().unwrap();
        right = node_list.pop().unwrap();
        sum = left.freq + right.freq;

        node_list.push(Node::new("".to_string(), sum).left(left).right(right));
    }

    // create look up table
    let mut table: HashMap<String, i64> = HashMap::new();

    let mut code: i64 = 0;

    node_list[0].codes_recursive(&mut code, &mut table);

    //println!("{:?}", table);

    //create output file
    let mut file = match File::create("C:\\work\\Huffman\\encoded.txt") {
        Err(why) => panic!("couldn't create file: {}", why),
        Ok(file) => file,
    };

    let mut output: String = String::new();

    // put codes in output 
    for (i, j) in &table {
        output.push_str(&i);
        output.push(' ');
        output.push_str(&j.to_string());
        output.push(' ');
    }
    output.push('\n');
    
    // encode input and add to output
    for i in input.chars() {
        output.push_str(table.get(&i.to_string()).unwrap().to_string().as_ref());
        output.push(' ');
    }

    // write table to file
    match file.write_all(output.as_bytes()) {
        Err(why) => panic!("couldn't write to file: {}", why),
        Ok(_) => println!("successfully wrote to file"),
    }

}
pub struct Node {
    symbol: String,
    freq: i32,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

impl<'a> Node {
    pub fn new(symbol: String, freq: i32) -> Self {
        Node { symbol: (symbol), freq: (freq), l: (None), r: (None) }
    }

    pub fn left(mut self, node: Node) -> Self {
        self.l = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: Node) -> Self {
        self.r = Some(Box::new(node));
        self
    }
    pub fn codes_recursive (&mut self, code: &mut i64, table: &mut HashMap<String,i64>) {
        match self.l {
            Some(ref mut node) => {
                *code = *code << 1;
                node.codes_recursive(code, table)
            }
            None => (),
        }
        match self.r {
            Some(ref mut node) => {
                *code = (*code << 1) + 1;
                node.codes_recursive(code, table)
            }
            None => (),
        }
        if self.symbol != "" {
            table.insert(self.symbol.clone(), *code);
        }
        return;
    }
}