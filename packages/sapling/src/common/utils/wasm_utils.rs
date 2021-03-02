use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::JsValue;

use crate::common::traits::Serializable;

pub fn js_serialize<S, E>(value: S) -> Result<Vec<u8>, JsValue> 
    where S: Serializable<Vec<u8>, E>,
          E: ToString {

    value.serialize().map_err(|err| JsValue::from(err.to_string()))
}

pub fn js_serialize_res<S, E>(value: Result<S, E>) -> Result<Vec<u8>, JsValue>
    where S: Serializable<Vec<u8>, E>,
          E: ToString {

    value
        .and_then(|s| s.serialize())
        .map_err(|err| JsValue::from(err.to_string()))
}

pub fn js_deserialize<S, E>(bytes: &[u8]) -> Result<S, JsValue>
    where S: Serializable<Vec<u8>, E>,
          E: ToString {

    S::deserialize(bytes.to_vec()).map_err(|err| JsValue::from(err.to_string()))
}

pub fn js_result_from<O, E: ToString>(error: E) -> Result<O, JsValue> {
    Err(js_error_from(error))
}

pub fn js_error_from<E: ToString>(error: E) -> JsValue {
    JsValue::from(error.to_string())
}

pub fn js_reference<T>(object: T) -> u32 {
    let ref_cell = WasmRefCell::new(object);
    let boxed = Box::new(ref_cell);

    Box::into_raw(boxed) as u32
}

pub unsafe fn js_dereference<'a, T>(pointer: u32) -> &'a mut T {
    let pointer = pointer as *mut WasmRefCell<T>;
    let ref_cell = &mut *pointer;
    ref_cell.get_mut()
}

pub unsafe fn js_drop_reference<T>(pointer: u32) {
    let pointer = pointer as *mut WasmRefCell<T>;
    (*pointer).borrow_mut(); // ensure no active borrows
    drop(Box::from_raw(pointer));
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use wasm_bindgen_test::*;

    use super::*;

    #[derive(Debug, PartialEq, Copy, Clone)]
    struct TestSerializable([u8; 3]);

    impl Serializable<Vec<u8>, String> for TestSerializable {
        fn deserialize(serialized: Vec<u8>) -> Result<Self, String> {
            {
               if serialized.len() != 3 {
                   Err("invalid length")
               } else {
                   Ok(())
               }
            }?;

            let mut bytes_copy = [0u8; 3];
            bytes_copy.copy_from_slice(&serialized);
            bytes_copy.reverse();

            Ok(TestSerializable(bytes_copy))
        }

        fn serialize(&self) -> Result<Vec<u8>, String> {
            let mut bytes = [0u8; 3];
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
                let actual = js_serialize_res(Ok(*result)).unwrap();

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
                let actual = js_result_from::<(), &Box<dyn fmt::Display>>(err).unwrap_err();

                (actual, js_value)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(actual, *expected)
        }
    }
}