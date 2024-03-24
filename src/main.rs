#![allow(unused)]
use std::io;
use rand::Rng; //ability to generate values
use std::io::{Write,BufReader,BufRead,ErrorKind}; //method to import all file at once
use std::fs::File; //file handling
use std::cmp::Ordering; //for comparing values

fn main() {
    const ONE_MIL:u32=1_000_000;
    const PI:f32= 3.23;
    let age= "34";
    //shadowing - same variable  name but different type
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age= age + 1;
    println!("I'm {} and I want  ${}", age, ONE_MIL);
}
