use regex::Regex;

use crate::utils::regex_utils::{contains_chars_re, not_contains_chars_re};
use super::errors::DerivationPathError;

const MASK_HARD_DERIVATION: u32 = 0x80000000;
const MASK_SOFT_DERIVATION: u32 = 0x00000000;

const VALID_INDEX_CHARACTERS_RE: &str = "0-9";
const VALID_IS_HARD_CHARACTERS_RE: &str = "'h";

#[derive(PartialEq, Debug)]
pub struct DerivationJunction {
  index: u32,
  is_hard: bool,
}

impl DerivationJunction {
  pub fn new(index: u32, is_hard: bool) -> DerivationJunction {
    DerivationJunction { index, is_hard }
  }

  pub fn from(string: &str) -> Result<DerivationJunction, DerivationPathError> {
    if string.len() == 0 {
      return Err(DerivationPathError::EmptyJunction)
    }

    let invalid_regex = Regex::new(not_contains_chars_re(&[VALID_INDEX_CHARACTERS_RE, VALID_IS_HARD_CHARACTERS_RE]).as_str())
      .or(Err(DerivationPathError::unknown("could not check derivation junction, invalid regular expression")))?;

    if invalid_regex.is_match(string) {
      let invalid = invalid_regex.find_iter(string).map(|m| &string[m.start()..m.end()]).collect();
      return Err(DerivationPathError::invalid_character(invalid))
    }

    let is_hard_regex = Regex::new(contains_chars_re(&[VALID_IS_HARD_CHARACTERS_RE]).as_str())
      .or(Err(DerivationPathError::unknown("could not create derivation junction, invalid `is_hard` regular expression")))?;

    let len = string.len();
    let is_hard = is_hard_regex.is_match(string);
    let index_end = if is_hard {
      len - 1
    } else {
      len
    };

    let index = &string[..index_end];
    let index = index.parse::<u32>().or(Err(DerivationPathError::unknown("could not parse derivation junction index")))?;

    Ok(DerivationJunction::new(index, is_hard))
  }

  pub fn value(&self) -> u32 {
    let mask = if self.is_hard {
      MASK_HARD_DERIVATION
    } else {
      MASK_SOFT_DERIVATION
    };

    self.index | mask
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn creates_derivation_junction_from_index_and_hard_flag() {
    let hard = DerivationJunction::new(44, true);
    let soft = DerivationJunction::new(0, false);

    assert_eq!(hard, DerivationJunction { index: 44, is_hard: true });
    assert_eq!(soft, DerivationJunction { index: 0, is_hard: false});
  }

   #[test]
   fn creates_derivation_junction_from_valid_string() {
      let hard_44 = DerivationJunction::from("44'").unwrap();
      let hard_0 = DerivationJunction::from("0h").unwrap();

      let soft_0 = DerivationJunction::from("0").unwrap();

      assert_eq!(hard_44, DerivationJunction { index: 44, is_hard: true });
      assert_eq!(hard_0, DerivationJunction { index: 0, is_hard: true });

      assert_eq!(soft_0, DerivationJunction { index: 0, is_hard: false});
   }

   #[test]
   fn fails_with_empty_junction_error_if_empty_string() {
      let empty = DerivationJunction::from("");

      assert_eq!(empty, Err(DerivationPathError::EmptyJunction));
   }
   
   #[test]
   fn fails_with_invalid_character_error_if_string_contains_illegal_characters() {
     let negative = DerivationJunction::from("-44");
     let not_a_number = DerivationJunction::from("abc");

     assert_eq!(negative, Err(DerivationPathError::invalid_character(vec!["-"])));
     assert_eq!(not_a_number, Err(DerivationPathError::invalid_character(vec!["a", "b", "c"])));
   }
}