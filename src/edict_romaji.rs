extern crate regex;
extern crate wana_kana;

use regex::Regex;
use std::io::prelude::*;
use wana_kana::to_romaji::*;

fn parse_line(s: &str) -> &str {
    let r = Regex::new(r"^([^ ]+) \[(.+)\] ").unwrap();
    let caps = r.captures(s);
    if caps.is_some() {
      caps.unwrap().get(2).unwrap().as_str()
    } else {
        let r = Regex::new(r"^([^ ]+) /").unwrap();
	r.captures(s).unwrap().get(1).unwrap().as_str()
    }
}

fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    for line in stdin.lines() {
        let line = line.unwrap();
        println!("{} [{}]", line, to_romaji(parse_line(&line)));
    }
}
