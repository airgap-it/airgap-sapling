use wasm_bindgen::JsValue;

use crate::common::traits::Serializable;

pub fn js_serialize<S, E>(value: Result<S, E>) -> Result<Vec<u8>, JsValue>
    where S: Serializable<E>,
          E: ToString {

    value
        .and_then(|s| s.to_bytes())
        .map_err(|err| JsValue::from(err.to_string()))
}

pub fn js_deserialize<S, E>(bytes: &[u8]) -> Result<S, JsValue>
    where S: Serializable<E>,
          E: ToString {

    S::from_bytes(bytes).map_err(|err| JsValue::from(err.to_string()))
}

pub fn js_error_from<O, E: ToString>(error: E) -> Result<O, JsValue> {
    Err(JsValue::from(error.to_string()))
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use wasm_bindgen_test::*;

    use super::*;

    #[derive(Debug, PartialEq, Copy, Clone)]
    struct TestSerializable([u8; 3]);

    impl Serializable<String> for TestSerializable {
        fn from_bytes(bytes: &[u8]) -> Result<Self, String> where Self: Sized {
            {
               if bytes.len() != 3 {
                   Err("invalid length")
               } else {
                   Ok(())
               }
            }?;

            let mut bytes_copy: [u8; 3] = [0; 3];
            bytes_copy.copy_from_slice(bytes);
            bytes_copy.reverse();

            Ok(TestSerializable(bytes_copy))
        }

        fn to_bytes(&self) -> Result<Vec<u8>, String> {
            let mut bytes: [u8; 3] = [0; 3];
            bytes.clone_from_slice(&self.0);
            bytes.reverse();

            Ok(bytes.to_vec())
        }
    }

    #[wasm_bindgen_test]
    fn serializes_result() {
        let test_data = vec![
            (TestSerializable([1, 2, 3]), [3, 2, 1]),
        ];

        let actual_expected = test_data.iter()
            .map(|(result, bytes)| {
                let actual = js_serialize(Ok(*result)).unwrap();

                (actual, bytes)
            });

        for (actual, expected) in actual_expected {
            println!("{:?}, {:?}", actual, expected);
            assert_eq!(actual, expected);
        }
    }

    #[wasm_bindgen_test]
    fn deserializes_bytes() {
        let test_data = vec![
            ([3, 2, 1], TestSerializable([1, 2, 3])),
        ];

        let actual_expected = test_data.iter()
            .map(|&(bytes, expected)| {
                let actual = js_deserialize::<TestSerializable, _>(&bytes).unwrap();

                (actual, expected)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(actual, expected);
        }
    }

    #[wasm_bindgen_test]
    fn creates_js_value_from_errors() {
        let test_data: Vec<(Box<dyn fmt::Display>, JsValue)> = vec![
            (Box::new("error"), JsValue::from("error")),
        ];

        let actual_expected = test_data.iter()
            .map(|(err, js_value)| {
                let actual = js_error_from::<(), &Box<dyn fmt::Display>>(err).unwrap_err();

                (actual, js_value)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(actual, *expected)
        }
    }
}