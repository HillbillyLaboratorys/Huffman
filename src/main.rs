use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    
    let pairs = ["th", "he", "in", "en", "nt", "re", "er", "an", "ti", "es", "on", "at", "se", "nd", "or", "ar", "al", "te", "co", "de", "to", "ra", "et", "ed", "it", "sa", "em", "ro",
    "Th", "He", "In", "En", "Nt", "Re", "Er", "An", "Ti", "Es", "On", "At", "Se", "Nd", "Or", "Ar", "Al", "Te", "Co", "De", "To", "Ra", "Et", "Ed", "It", "Sa", "Em", "Ro",
    ];

    // read file contents to string
    let mut file = File::open("C:\\work\\Huffman\\test.txt")
        .expect("File Not Found");

    let mut input = String::new();

    file.read_to_string(&mut input)
        .expect("Error");

    // record frequency of charactors in input
    let frequency = count_freq(&input);
    //println!("{:?}", frequency);

    // create tree
    let node_list: Vec<Node> = generate_tree(frequency);

    // create look up table
    let mut table: HashMap<String, i64> = HashMap::new();

    let mut code: i64 = 0;

    node_list[0].codes_recursive(&mut code, &mut table);
    //println!("{:?}", table);

    // calculate and print code bit lengths 
    let mut lens: HashMap<String,i32> = HashMap::new();

    let mut val: i64;

    let mut count: i32 = 0;

    let mut tab_total: i32 = 0;

    for (i, j) in &table {
        val = *j;
        while val > 0 {
            val = val >> 1;
            count += 1;
        }
        lens.insert(i.to_string(), count);
        tab_total += count;
        count = 0;
    }
    println!("table size: {}", tab_total);

    let mut output: String = String::new();

    // put codes in output string
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

    // calculate bit length total of encodded input
    let mut encodelen: i32 = 0;
    
    for i in output.chars() {
        match lens.get(&i.to_string()) {
            Some(x) => { 
                encodelen += *x;
            }
            None => (),
        }
        // encodelen += lens.get(&i.to_string());
    }
    println!("message size: {}", encodelen);
    println!("message and table size: {}", encodelen + tab_total);
    

    //create output file
    let mut file = match File::create("C:\\work\\Huffman\\encoded.txt") {
        Err(why) => panic!("couldn't create file: {}", why),
        Ok(file) => file,
    };

    // write table to file
    match file.write_all(output.as_bytes()) {
        Err(why) => panic!("couldn't write to file: {}", why),
        Ok(_) => println!("successfully wrote to file"),
    }

    // frequency of charactor pairs 
    let freq_pair = count_freq_pair(&input, pairs);
    println!("{:?} /n", freq_pair);

    // create tree for pairs
    let node_list_pair: Vec<Node> = generate_tree(freq_pair);

    // create look up table
    let mut table_pair: HashMap<String, i64> = HashMap::new();

    let mut code_pair: i64 = 0;

    node_list_pair[0].codes_recursive(&mut code_pair, &mut table_pair);
    //println!("{:?}", table_pair);

    // calculate and print code bit lengths 
    let mut lens_pair: HashMap<String,i32> = HashMap::new();

    let mut val_pair: i64;

    let mut count_pair: i32 = 0;

    let mut tab_total_pair: i32 = 0;

    for (i, j) in &table_pair {
        val_pair = *j;
        while val_pair > 0 {
            val_pair = val_pair >> 1;
            count_pair += 1;
        }
        lens_pair.insert(i.to_string(), count_pair);
        tab_total_pair += count_pair;
        count_pair = 0;
    }
    println!("pair table size: {}", tab_total_pair);

    let mut output_pair: String = String::new();

    // put codes in output string
    for (i, j) in &table_pair {
        output_pair.push_str(&i);
        output_pair.push(' ');
        output_pair.push_str(&j.to_string());
        output_pair.push(' ');
    }
    output_pair.push('\n');
    
    // encode input and add to output
    let mut last_let = "".to_string();

    let mut vec_count = 0;
    let mut pair;

    let mut huff_pair: Vec<i64> = Vec::new();

    for  j in input.chars() {
        // add code for char in string to temp vector
        huff_pair.push(*table_pair.get(&j.to_string()).unwrap());
        vec_count += 1;
        
        pair = false;

        let mut tmp = last_let.to_string();
        tmp.push(j);
        for k in pairs {
            // if a charactor pair is found remove current code and replace previous with pair code
            if k.to_string() == tmp {
                huff_pair.pop();
                vec_count -= 1;
                huff_pair[vec_count - 1] = *table_pair.get(&tmp).unwrap();
                pair = true;
                break;
            }
        }
        if pair {
            last_let = tmp;
        }
        else {
            last_let = j.to_string();
        }
    }

    // calculate bit length total of encodded input
    let mut encodelen_pair: i32 = 0;
    
    for i in output_pair.chars() {
        match lens_pair.get(&i.to_string()) {
            Some(x) => { 
                encodelen_pair += *x;
            }
            None => (),
        }
        // encodelen += lens.get(&i.to_string());
    }
    println!("pair message size: {}", encodelen_pair);
    println!("pair message and table size: {}", encodelen_pair + tab_total_pair);

    println!("ASKII size: {}", output_pair.len() * 8);
}

fn count_freq (input: &String) -> HashMap<String, i32> {

    // takes input string and counts how many times each charactor is used 

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
    frequency
}

fn generate_tree (frequency_list: HashMap<String, i32>) -> Vec<Node> {
    
    // takes in charactors and their associated frequency's, and builds a binary tree
    // organized by frequency 

    let mut node_list: Vec<Node> = Vec::new();

    for (i, j) in frequency_list {
        node_list.push(Node::new(i, j));
    }

    let mut left: Node;
    let mut right: Node;
    let mut sum: i32;

    while node_list.len() > 1 {
        node_list.sort_by(|a,b| b.freq.cmp(&a.freq));

        left = node_list.pop().unwrap();
        right = node_list.pop().unwrap();
        sum = left.freq + right.freq;

        node_list.push(Node::new("".to_string(), sum).left(left).right(right));
    }
    node_list
}

fn count_freq_pair (input: &String, pairs: [&str; 56]) -> HashMap<String, i32> {

    // takes input string and counts how many times each charactor or charactor pair is used 

    let mut frequency = HashMap::new();

    let mut last_let: String = "".to_string();

    let mut tmp;

    let mut pair;

    for i in input.chars() {

        pair = false;

        tmp = last_let.to_string();
        tmp.push(i);

        for j in pairs {
            if j.to_string() == tmp {
                
                if frequency.contains_key(&tmp) {
                    
                    inc_dec(&mut frequency, &tmp, 1);                    
                }
                else {
                    frequency.insert(tmp.clone(), 1);
                }
                
                inc_dec(&mut frequency, &last_let, -1);
                pair = true;
                break;
            }
        }

        if !pair {

            if frequency.contains_key(&i.to_string()) {
                
                inc_dec(&mut frequency, &i.to_string(), 1);
            }
            else {

                frequency.insert(i.to_string(), 1);
            }
            last_let = i.to_string();
        }
        else {
            last_let = tmp;
        }
        
    }

    fn inc_dec (map: &mut HashMap<String, i32>,letter: &String , p_m: i32) {

        let current = map.get(letter).unwrap();
        map.insert(letter.clone(), current + p_m);
    }
    frequency
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
    pub fn codes_recursive (&self, code: &i64, table: &mut HashMap<String,i64>) { 
        match self.l {
            Some(ref node) => {
                let nx_code = *code << 1;
                node.codes_recursive(&nx_code, table)
            }
            None => (),
        }
        match self.r {
            Some(ref node) => {
                let nx_code = (*code << 1) + 1;
                node.codes_recursive(&nx_code, table)
            }
            None => (),
        }
        if self.symbol != "" {
            table.insert(self.symbol.clone(), *code);
        }
        return;
    }
}