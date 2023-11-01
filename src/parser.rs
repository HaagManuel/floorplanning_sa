use crate::definitions::*;
use std::fs::File;
use std::path::Path;
use std::io::{self, prelude::*, BufReader};

fn parse_ints(s: &String) -> Vec<Int> {
    s.split(" ").map(|x|  x.parse::<Int>().unwrap()).collect()
}

pub fn parse_file<P>(file_path: P) -> io::Result<(Vec<Rectangle>, Vec<Net>)>
where P: AsRef<Path>, {
    let file = BufReader::new(File::open(file_path).unwrap());
    let mut lines: Vec<String> = Vec::new();
    let mut blocks: Vec<Rectangle> = Vec::new();
    let mut nets: Vec<Net> = Vec::new();
    for line in file.lines() {
        let s = line?;
        if !s.starts_with("#") {
            lines.push(s);
        }
    }
    let (n, m) = lines[0].split_once(' ').unwrap();
    let num_blocks = n.parse::<usize>().unwrap();
    let num_nets = m.parse::<usize>().unwrap();
    for i in 0..num_blocks {
        let width_height = parse_ints(&lines[i + 1]);
        blocks.push(Rectangle::new(width_height[0], width_height[1]));
    }
    for i in 0..num_nets {
        let pins = parse_ints(&lines[i + 1 + num_blocks]).iter().map(|&x| x as usize).collect();
        nets.push(Net::new(pins, i));
    }
    Ok((blocks, nets))
}