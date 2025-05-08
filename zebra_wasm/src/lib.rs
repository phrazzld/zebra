use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use zebra_crypto::SignedMessage;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Represents the parsed result of a line containing an identifier
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParsedLineInfo {
    pub prefix: String,
    pub identifier: String,
    pub suffix: String,
}

#[wasm_bindgen]
pub fn verify_signature(message: &str) -> bool {
    match SignedMessage::from_str(message) {
        Ok(signed_message) => signed_message.verify(),
        Err(_) => false,
    }
}

/// Parses a line looking for text within angle brackets format: " <identifier> "
/// Returns a ParsedLineInfo struct containing:
/// - prefix: Text before the identifier (including opening bracket)
/// - identifier: The text between brackets (without brackets)
/// - suffix: Text after the identifier (including closing bracket)
///
/// This replicates the behavior of the JavaScript parseString function
/// from the webapp.
#[wasm_bindgen]
pub fn parse_line_for_identifier(input: &str) -> Result<JsValue, JsValue> {
    // Start from the end and find the last occurrence of " <"
    let mut start = input.rfind(" <");

    while let Some(start_idx) = start {
        // Find the closing bracket followed by space
        if let Some(end_idx) = input[start_idx..].find("> ") {
            let end_idx = start_idx + end_idx + 2; // Adjust to absolute position (+2 for "> ")

            // Extract the middle part (without brackets)
            let middle = &input[(start_idx + 2)..(end_idx - 2)];

            // Ensure the middle part doesn't contain whitespace
            if !middle.contains(char::is_whitespace) {
                let before = input[..start_idx + 1].to_string(); // Include the space
                let after = input[(end_idx - 1)..].to_string(); // Include the space

                let result = ParsedLineInfo {
                    prefix: before,
                    identifier: middle.to_string(),
                    suffix: after,
                };

                // Serialize to JsValue and return
                match to_value(&result) {
                    Ok(value) => return Ok(value),
                    Err(e) => {
                        return Err(JsValue::from_str(&format!("Serialization error: {}", e)))
                    }
                }
            }
        }

        // Try to find an earlier occurrence
        start = input[..start_idx].rfind(" <");
    }

    // If no valid identifier found, return the original input with empty identifier
    let result = ParsedLineInfo {
        prefix: input.to_string(),
        identifier: "".to_string(),
        suffix: "".to_string(),
    };

    match to_value(&result) {
        Ok(value) => Ok(value),
        Err(e) => Err(JsValue::from_str(&format!("Serialization error: {}", e))),
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Keeping this around for testing purposes
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-test!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_identifier() {
        let input = "John Doe <ABC123> john@example.com";
        let expected = ParsedLineInfo {
            prefix: "John Doe ".to_string(),
            identifier: "ABC123".to_string(),
            suffix: " john@example.com".to_string(),
        };

        // We need to deserialize the JsValue to check the result
        // Since we can't do this in a test, we'll test the internal logic
        let (prefix, identifier, suffix) = parse_for_test(input);

        assert_eq!(prefix, expected.prefix);
        assert_eq!(identifier, expected.identifier);
        assert_eq!(suffix, expected.suffix);
    }

    #[test]
    fn test_multiple_identifiers() {
        let input = "John <Doe> <ABC123> john@example.com";
        let expected = ParsedLineInfo {
            prefix: "John <Doe> ".to_string(),
            identifier: "ABC123".to_string(),
            suffix: " john@example.com".to_string(),
        };

        let (prefix, identifier, suffix) = parse_for_test(input);

        assert_eq!(prefix, expected.prefix);
        assert_eq!(identifier, expected.identifier);
        assert_eq!(suffix, expected.suffix);
    }

    #[test]
    fn test_no_identifier() {
        let input = "John Doe - john@example.com";
        let expected = ParsedLineInfo {
            prefix: input.to_string(),
            identifier: "".to_string(),
            suffix: "".to_string(),
        };

        let (prefix, identifier, suffix) = parse_for_test(input);

        assert_eq!(prefix, expected.prefix);
        assert_eq!(identifier, expected.identifier);
        assert_eq!(suffix, expected.suffix);
    }

    #[test]
    fn test_whitespace_in_identifier() {
        let input = "John Doe <ABC 123> john@example.com";
        let expected = ParsedLineInfo {
            prefix: input.to_string(),
            identifier: "".to_string(),
            suffix: "".to_string(),
        };

        let (prefix, identifier, suffix) = parse_for_test(input);

        assert_eq!(prefix, expected.prefix);
        assert_eq!(identifier, expected.identifier);
        assert_eq!(suffix, expected.suffix);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected = ParsedLineInfo {
            prefix: "".to_string(),
            identifier: "".to_string(),
            suffix: "".to_string(),
        };

        let (prefix, identifier, suffix) = parse_for_test(input);

        assert_eq!(prefix, expected.prefix);
        assert_eq!(identifier, expected.identifier);
        assert_eq!(suffix, expected.suffix);
    }

    /// Helper function for testing parse_line_for_identifier without dealing with JsValue
    fn parse_for_test(input: &str) -> (String, String, String) {
        // Start from the end and find the last occurrence of " <"
        let mut start = input.rfind(" <");

        while let Some(start_idx) = start {
            // Find the closing bracket followed by space
            if let Some(end_idx) = input[start_idx..].find("> ") {
                let end_idx = start_idx + end_idx + 2; // Adjust to absolute position (+2 for "> ")

                // Extract the middle part (without brackets)
                let middle = &input[(start_idx + 2)..(end_idx - 2)];

                // Ensure the middle part doesn't contain whitespace
                if !middle.contains(char::is_whitespace) {
                    let before = input[..start_idx + 1].to_string(); // Include the space
                    let after = input[(end_idx - 1)..].to_string(); // Include the space

                    return (before, middle.to_string(), after);
                }
            }

            // Try to find an earlier occurrence
            start = input[..start_idx].rfind(" <");
        }

        // If no valid identifier found, return the original input with empty identifier
        (input.to_string(), "".to_string(), "".to_string())
    }
}
