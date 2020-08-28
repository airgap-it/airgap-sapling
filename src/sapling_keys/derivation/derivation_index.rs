use regex::Regex;
use zcash_primitives::zip32::ChildIndex;

use crate::utils::regex_utils::{contains_chars_re, not_contains_chars_re};
use super::errors::DerivationPathError;

const MASK_HARD_DERIVATION: u32 = 0x80000000;
const MASK_SOFT_DERIVATION: u32 = 0x00000000;

const VALID_INDEX_CHARACTERS_RE: &str = "0-9";
const VALID_IS_HARD_CHARACTERS_RE: &str = "'h";

pub type DerivationIndex = ChildIndex;

pub fn create_derivation_index(string: &str) -> Result<DerivationIndex, DerivationPathError> {
    if string.len() == 0 {
        return Err(DerivationPathError::EmptyIndex)
    }

    let invalid_regex = Regex::new(not_contains_chars_re(&[VALID_INDEX_CHARACTERS_RE, VALID_IS_HARD_CHARACTERS_RE]).as_str())
        .or(Err(DerivationPathError::unknown("could not check derivation index, invalid regular expression")))?;

    if invalid_regex.is_match(string) {
        let invalid = invalid_regex.find_iter(string).map(|m| &string[m.start()..m.end()]).collect();
        return Err(DerivationPathError::invalid_character(invalid))
    }

    let is_hard_regex = Regex::new(contains_chars_re(&[VALID_IS_HARD_CHARACTERS_RE]).as_str())
        .or(Err(DerivationPathError::unknown("could not create derivation index, invalid `is_hard` regular expression")))?;

    let len = string.len();
    let is_hard = is_hard_regex.is_match(string);
    let index_end = if is_hard {
        len - 1
    } else {
        len
    };

    let index = &string[..index_end];
    let index = index.parse::<u32>().or(Err(DerivationPathError::unknown("could not parse derivation index")))?;

    let mask = if is_hard {
        MASK_HARD_DERIVATION
    } else {
        MASK_SOFT_DERIVATION
    };

    Ok(DerivationIndex::from_index(index | mask))
}

#[cfg(test)]
mod test {
  use super::*;

   #[test]
   fn creates_derivation_index_from_valid_string() {
      let hard_44 = create_derivation_index("44'").unwrap();
      let hard_0 = create_derivation_index("0h").unwrap();

      let soft_0 = create_derivation_index("0").unwrap();

      assert_eq!(hard_44, DerivationIndex::Hardened(44));
      assert_eq!(hard_0, DerivationIndex::Hardened(0));

      assert_eq!(soft_0, DerivationIndex::NonHardened(0));
   }

   #[test]
   fn fails_with_empty_index_error_if_empty_string() {
      let empty = create_derivation_index("");

      assert_eq!(empty, Err(DerivationPathError::EmptyIndex));
   }
   
   #[test]
   fn fails_with_invalid_character_error_if_string_contains_illegal_characters() {
       let negative = create_derivation_index("-44");
       let not_a_number = create_derivation_index("abc");

       assert_eq!(negative, Err(DerivationPathError::invalid_character(vec!["-"])));
       assert_eq!(not_a_number, Err(DerivationPathError::invalid_character(vec!["a", "b", "c"])));
   }

    #[test]
    fn fails_with_parsing_error_if_index_is_too_big() {
        let too_big = create_derivation_index(&(std::u64::MAX).to_string()[..]);

        assert_eq!(too_big, Err(DerivationPathError::unknown("could not parse derivation index")))
    }
}