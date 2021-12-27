#[derive(Clone, Copy)]
pub enum Pin {
  Positive,
  Negative,
}

impl Pin {
  pub fn nand(&self, another: Pin) -> Pin {
    match self {
      Self::Positive => match another {
        Self::Positive => Self::Negative,
        Self::Negative => Self::Positive,
      },
      Self::Negative => Self::Positive,
    }
  }
}

mod tests {
  use super::Pin;

  #[test]
  fn nand_works() {
    assert!(matches!(Pin::Positive.nand(Pin::Positive), Pin::Negative));
    assert!(matches!(Pin::Positive.nand(Pin::Negative), Pin::Positive));
    assert!(matches!(Pin::Negative.nand(Pin::Positive), Pin::Positive));
    assert!(matches!(Pin::Negative.nand(Pin::Negative), Pin::Positive));
  }
}
