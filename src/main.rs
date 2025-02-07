/*
  I do realize that this entire program is massively over-engineered, but I wanted
  to try and stuff as many of my favorite techniques in here as possible.
*/

use std::{collections::HashSet, fmt::Display, marker::PhantomData, str::FromStr};

use clap::Parser;
use fraction::{error::ParseError, BigDecimal, One};

// Program args
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long, num_args(1..))]
  numbers: Vec<Number<Decimal>>,
}

fn main() {
  let args = Args::parse();
  let decimals: Vec<Number<Decimal>> = args.numbers.clone();
  let binaries: Vec<Number<Binary>> = args.numbers.iter().map(Number::to_binary).collect();
  format_output(decimals, binaries);
}

fn format_output(decimals: Vec<Number<Decimal>>, binaries: Vec<Number<Binary>>) {
  println!("| Base 10 | Base 2 |\n| :------ | :----- |");
  for (decimal, binary) in decimals.iter().zip(binaries) {
    let d = decimal.to_string();
    let b = binary.to_string();
    println!("| {d:8.8}| {b:7.7}|");
  }
}

// Storage for both sides of the decimal
#[derive(Debug, Default, Clone)]
struct Number<Base> {
  _base: PhantomData<Base>,
  num: BigDecimal,
}

// Markers for current base
#[derive(Debug, Default, Clone)]
struct Decimal;
#[derive(Debug, Default, Clone)]
struct Binary;

// Required global constant
const MAX_DIGITS: u64 = 8;

// Implementing only for specific bases prevents "reconversion" at compile-time
impl Number<Decimal> {
  #[must_use]
  fn integral_part(&self) -> BigDecimal {
    self.num.trunc()
  }

  #[must_use]
  fn fractional_part(&self) -> BigDecimal {
    self.num.fract()
  }

  #[must_use]
  fn to_binary(&self) -> Number<Binary> {
    // integral portion
    let two = BigDecimal::from(2);
    let mut integer = self.integral_part();
    let mut integral_digits = "0".to_owned();
    while integer >= BigDecimal::one() {
      let rem = &integer % &two;
      integer /= 2; // div_assign isn't implemented for BigDecimal upon BigDecimal for some reason...
      match rem >= BigDecimal::one() {
        // rem is a decimal value because this library decided to do it that way for some reason...
        true => integral_digits.push('0'),
        false => integral_digits.push('1'),
      }
    }

    // fractional portion
    let fraction = self.fractional_part();
    let mut fractional_digits = "".to_owned();
    let mut seen = HashSet::new();
    let mut buffer = fraction * 2;
    for _ in 0..MAX_DIGITS {
      if seen.contains(&buffer) {
        break;
      }
      match buffer >= BigDecimal::one() {
        true => {
          fractional_digits.push('1');
          buffer -= BigDecimal::one();
        }
        false => fractional_digits.push('0'),
      }
      seen.insert(buffer.clone());
      buffer *= 2;
    }

    let digits = format!("{}.{}", integral_digits, fractional_digits);

    Number {
      _base: PhantomData,
      num: digits.parse().unwrap(),
    }
  }
}

// Extra conversion if I get around to implementing it
// impl Number<Binary> {
//   #[must_use]
//   fn to_decimal(&self) -> Number<Decimal> {
//     let [integer, decimal] = self.num.clone();

//     todo!()

//     Number {
//       _base: PhantomData,
//       num: [integer, decimal],
//     }
//   }
// }

// Basic output formatting
impl<Base> Display for Number<Base> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.num)
  }
}

// String parsing of number for reading from stdin
impl FromStr for Number<Decimal> {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      _base: PhantomData,
      num: s.parse()?,
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse_number() -> Result<(), ParseError> {
    let n = Number::from_str("Hello, Prof. Kennedy!");
    assert!(n.is_err());

    let n = Number::from_str("6.9")?;
    assert_ne!(BigDecimal::from(34), n.integral_part());
    assert_eq!(BigDecimal::from(6), n.integral_part());
    assert_eq!(BigDecimal::from_str("0.9")?, n.fractional_part());

    let n = Number::from_str("0.69")?;
    assert_eq!(BigDecimal::from(0), n.integral_part());
    assert_eq!(BigDecimal::from_str("0.69")?, n.fractional_part());

    let n = Number::from_str("034.69")?;
    assert_eq!(BigDecimal::from(34), n.integral_part());
    assert_eq!(BigDecimal::from_str("0.69")?, n.fractional_part());

    let n = Number::from_str("0.69")?;
    assert_eq!(BigDecimal::from(0), n.integral_part());
    assert_eq!(BigDecimal::from_str("0.69")?, n.fractional_part());

    let n = Number::from_str("69")?;
    assert_eq!(BigDecimal::from(69), n.integral_part());
    assert_eq!(BigDecimal::from_str("0")?, n.fractional_part());

    let n = Number::from_str("69.")?;
    assert_eq!(BigDecimal::from(69), n.integral_part());
    assert_eq!(BigDecimal::from_str("0")?, n.fractional_part());

    let n = Number::from_str("69.0")?;
    assert_eq!(BigDecimal::from(69), n.integral_part());
    assert_eq!(BigDecimal::from_str("0")?, n.fractional_part());

    let n = Number::from_str("69.00")?;
    assert_eq!(BigDecimal::from(69), n.integral_part());
    assert_eq!(BigDecimal::from_str("0")?, n.fractional_part());
    Ok(())
  }

  #[test]
  fn display_number() -> Result<(), ParseError> {
    let n = Number::from_str("12.34")?;
    assert_eq!("12.34", format!("{n}"));
    Ok(())
  }

  #[test]
  fn convert() -> Result<(), ParseError> {
    let n = Number::from_str("0.2")?;
    assert_eq!("0.0011", format!("{}", n.to_binary()));

    let n = Number::from_str("0.1")?;
    assert_eq!("0.00011", format!("{}", n.to_binary()));

    let n = Number::from_str("1/7")?;
    assert_eq!("0.001", format!("{}", n.to_binary()));
    Ok(())
  }
}
