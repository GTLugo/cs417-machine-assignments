// Markers for current base
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Base(pub usize);

impl Default for Base {
  fn default() -> Self {
    Self::DECIMAL
  }
}

impl Base {
  const DECIMAL: Self = Self(10);
}

impl From<usize> for Base {
  fn from(value: usize) -> Self {
    Base(value)
  }
}
