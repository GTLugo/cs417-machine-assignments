use fraction::{error::ParseError, BigDecimal, BigUint, One, Sign, ToPrimitive};
use std::{
  collections::{HashSet, VecDeque},
  fmt::Display,
  str::FromStr,
};

use crate::base::Base;

// Storage for both sides of the decimal
#[derive(Debug, Clone)]
pub struct Number {
  base: Base,
  sign: Sign,
  int: VecDeque<usize>,
  dec: VecDeque<usize>,
}

impl Default for Number {
  fn default() -> Self {
    Self {
      base: Base::default(),
      sign: Sign::Plus,
      int: vec![0].into(),
      dec: vec![0].into(),
    }
  }
}

// Required global constant
const MAX_DIGITS: u64 = 8;
// loss of precision is sadly required during comparison.
// This is because the value is being converted to and from BigDecimal, causing it
// to lose its finer precision when doing math.
// If I had more time, I would find a better way to stay in BigDecimal, but I've already spent
// several days trying to figure this out and now the assignment is overdue!
const PRECISION: usize = 128;

// Implementing only for specific bases prevents "reconversion" at compile-time
impl Number {
  ///
  /// Returns None if the number was already in the correct base.
  ///
  #[must_use]
  pub fn to_base(&self, base: impl Into<Base>) -> Option<Number> {
    let base = base.into();
    if self.base == base {
      return None;
    }

    let stringified = self.to_dec_string();
    let big_dec = BigDecimal::from_str(&stringified)
      .unwrap()
      .abs()
      .set_precision(PRECISION * 2);
    // eprintln!("{big_dec}");

    // integral portion
    let mut integer = big_dec.trunc();
    let mut integral_digits = VecDeque::new();
    while integer >= BigDecimal::one() {
      let rem = &integer % base.0; // rem is a decimal value because this library decided to do it that way for some reason...
      integer /= base.0; // div_assign isn't implemented for BigDecimal upon BigDecimal for some reason...
      let uint: BigUint = rem.trunc().try_into().expect("Could not convert to BigUint");
      integral_digits.push_front(uint.to_usize().expect("Value too large"));
    }
    if integral_digits.is_empty() {
      integral_digits.push_back(0);
    }

    // fractional portion
    let mut fractional_digits = VecDeque::new();
    let mut seen = HashSet::new();
    let mut buffer = big_dec.fract().set_precision(PRECISION) * base.0;
    for _ in 0..MAX_DIGITS {
      eprintln!("{buffer}");
      let integral_portion = buffer.trunc();
      let big_uint: BigUint = integral_portion
        .clone()
        .try_into()
        .expect("Could not convert to BigUint");

      if seen.contains(&buffer) {
        break;
      }

      let fractional_portion = buffer - integral_portion;
      seen.insert(fractional_portion.clone());

      fractional_digits.push_back(big_uint.to_usize().expect("Value too large"));
      buffer = fractional_portion * base.0;
    }

    Some(Number {
      base,
      sign: self.sign,
      int: integral_digits,
      dec: fractional_digits,
    })
  }

  pub fn to_dec_string(&self) -> String {
    let int: String = self.int.iter().map(usize::to_string).collect();
    let dec: String = self.dec.iter().map(usize::to_string).collect();
    format!("{}{}.{}", self.sign, int, dec)
  }
}

// Basic output formatting
impl Display for Number {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let int: String = itertools::intersperse(self.int.iter().map(usize::to_string), String::from(";")).collect();
    let dec: String = itertools::intersperse(self.dec.iter().map(usize::to_string), String::from(";")).collect();

    write!(f, "{}{}.{}", self.sign, int, dec)
  }
}

// String parsing of number for reading from stdin
impl FromStr for Number {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    const BASE: u32 = 10;
    let big_dec: BigDecimal = s.parse()?;
    let op = |s: &str| {
      let mut vec = s
        .chars()
        .map(|c| c.to_digit(BASE).unwrap() as usize)
        .collect::<VecDeque<usize>>();
      if vec.is_empty() {
        vec.push_back(0);
      }
      vec
    };

    let int = big_dec
      .abs()
      .trunc()
      .set_precision(PRECISION * 2)
      .to_string()
      .split(".")
      .map(op)
      .next()
      .unwrap();
    let dec = big_dec
      .abs()
      .fract()
      .set_precision(PRECISION * 2)
      .to_string()
      .split(".")
      .map(op)
      .last()
      .unwrap();

    Ok(Self {
      base: Base::default(),
      sign: big_dec.sign().unwrap(),
      int,
      dec,
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
    assert_ne!(VecDeque::from(vec![3, 4]), n.int);
    assert_eq!(VecDeque::from(vec![6]), n.int);
    assert_eq!(VecDeque::from(vec![9]), n.dec);

    let n = Number::from_str("0.69")?;
    assert_eq!(VecDeque::from(vec![0]), n.int);
    assert_eq!(VecDeque::from(vec![6, 9]), n.dec);

    let n = Number::from_str("034.69")?;
    assert_eq!(VecDeque::from(vec![3, 4]), n.int);
    assert_eq!(VecDeque::from(vec![6, 9]), n.dec);

    let n = Number::from_str("0.69")?;
    assert_eq!(VecDeque::from(vec![0]), n.int);
    assert_eq!(VecDeque::from(vec![6, 9]), n.dec);

    let n = Number::from_str("69")?;
    assert_eq!(VecDeque::from(vec![6, 9]), n.int);
    assert_eq!(VecDeque::from(vec![0]), n.dec);

    let n = Number::from_str("69.")?;
    assert_eq!(VecDeque::from(vec![6, 9]), n.int);
    assert_eq!(VecDeque::from(vec![0]), n.dec);

    let n = Number::from_str("69.0")?;
    assert_eq!(VecDeque::from(vec![6, 9]), n.int);
    assert_eq!(VecDeque::from(vec![0]), n.dec);

    let n = Number::from_str("69.00")?;
    assert_eq!(VecDeque::from(vec![6, 9]), n.int);
    assert_eq!(VecDeque::from(vec![0]), n.dec);
    Ok(())
  }

  #[test]
  fn display_number() -> Result<(), ParseError> {
    let n = Number::from_str("12.34")?;
    assert_eq!("12.34", format!("{}", n.to_dec_string()));
    assert_eq!("1;2.3;4", format!("{n}"));
    Ok(())
  }

  #[test]
  fn convert() -> Result<(), ParseError> {
    let n = Number::from_str("0.2")?;
    let c = n.to_base(10);
    assert!(c.is_none());

    let n = Number::from_str("0.2")?;
    let c = n.to_base(2);
    assert!(c.is_some());
    assert_eq!("0.0;0;1;1", c.unwrap().to_string());

    let n = Number::from_str("0.1")?;
    let c = n.to_base(2);
    assert!(c.is_some());
    assert_eq!("0.0;0;0;1;1", c.unwrap().to_string());

    let n = Number::from_str("1/7")?;
    let c = n.to_base(2);
    assert!(c.is_some());
    assert_eq!("0.0;0;1", c.unwrap().to_string());

    let n = Number::from_str("0.5")?;
    let c = n.to_base(60);
    assert!(c.is_some());
    assert_eq!("0.30", c.unwrap().to_string());

    let n = Number::from_str("0.25")?;
    let c = n.to_base(60);
    assert!(c.is_some());
    assert_eq!("0.15", c.unwrap().to_string());

    let n = Number::from_str("0.75")?;
    let c = n.to_base(60);
    assert!(c.is_some());
    assert_eq!("0.45", c.unwrap().to_string());

    let n = Number::from_str("0.8")?;
    let c = n.to_base(60);
    assert!(c.is_some());
    assert_eq!("0.48", c.unwrap().to_string());

    let n = Number::from_str("0.16666")?;
    assert_eq!("0.16666", format!("{}", n.to_dec_string()));
    let c = n.to_base(60);
    assert!(c.is_some());
    assert_eq!("0.9;59;58;33;36", c.unwrap().to_string()); // I can't get perfect precision here.

    Ok(())
  }
}
