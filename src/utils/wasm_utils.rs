use wasm_bindgen::JsValue;

pub fn js_error_from<O, E: ToString>(error: E) -> Result<O, JsValue> {
    Err(JsValue::from(error.to_string()))
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use wasm_bindgen_test::*;
    use super::*;

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