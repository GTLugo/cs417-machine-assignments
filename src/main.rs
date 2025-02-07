/*
  I do realize that this entire program is massively over-engineered, but I wanted
  to try and stuff as many of my favorite techniques in here as possible.
*/

use clap::Parser;

use decto::prelude::*;

// Program args
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  base: usize,
  #[arg(allow_negative_numbers(true), num_args(1..))]
  numbers: Vec<Number>,
}

fn main() {
  let args = Args::parse();
  let decimals: Vec<Number> = args.numbers.clone();
  let targets: Vec<Number> = args.numbers.iter().map(|x| x.to_base(args.base).unwrap()).collect();
  format_output(args.base, decimals, targets);
}

fn format_output(base: usize, decimals: Vec<Number>, targets: Vec<Number>) {
  println!("| Base 10 | Base {base} |\n| :------ | :----- |");
  for (decimal, binary) in decimals.iter().zip(targets) {
    let d = decimal.to_dec_string();
    let b = binary.to_string();
    println!("| {d:8.8}| {b:7.7}|");
  }
}
