/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::num::ParseIntError;

use hmac::{Hmac, Mac};
use sha2::Sha256;
use thiserror::Error;

#[derive(Debug)]
pub struct EventSignature {
    bytes: Box<[u8]>,
}

impl EventSignature {
    pub fn from_signature_header(
        sha256_signature_header: &str,
    ) -> Result<EventSignature, SignatureConversionError> {
        let hex_string = sha256_signature_header
            .strip_prefix("sha256=")
            .unwrap_or(sha256_signature_header);

        let bytes = (0..hex_string.len())
            .step_by(2)
            .map(|i| {
                let hex_tuple =
                    hex_string
                        .get(i..i + 2)
                        .ok_or(SignatureConversionError::InvalidInput(
                            hex_string.to_string(),
                        ))?;

                if !hex_tuple.is_ascii() {
                    return Err(SignatureConversionError::InvalidInput(
                        hex_string.to_string(),
                    ));
                }

                u8::from_str_radix(hex_tuple, 16).map_err(|error| {
                    SignatureConversionError::NonHexCharacter(hex_tuple.to_string(), error)
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(EventSignature { bytes })
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

pub struct EventVerifier {
    secret: Box<[u8]>,
}

impl EventVerifier {
    pub fn new(secret: &str) -> Self {
        EventVerifier {
            secret: secret.as_bytes().into(),
        }
    }

    pub fn payload_is_valid(&self, payload: &[u8], signature: &EventSignature) -> bool {
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&self.secret).expect("HMAC can take key of any size");

        mac.update(payload);

        mac.verify_slice(signature.bytes()).is_ok()
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum SignatureConversionError {
    #[error("Hex string must contain an even amount of ASCII characters: {0}")]
    InvalidInput(String),

    #[error("Hex tuple could not be parsed: {0}")]
    NonHexCharacter(String, ParseIntError),
}

#[cfg(test)]
mod signature_parsing_tests {
    use super::*;

    #[test]
    fn can_parse_signature_header_without_prefix() {
        let signature = EventSignature::from_signature_header("000F10FF").unwrap();

        assert_eq!(signature.bytes(), &[0, 15, 16, 255]);
    }

    #[test]
    fn can_parse_lower_case_signature_headers() {
        let signature = EventSignature::from_signature_header("000f10ff").unwrap();

        assert_eq!(signature.bytes(), &[0, 15, 16, 255]);
    }

    #[test]
    fn can_parse_signature_header_prefix() {
        let signature = EventSignature::from_signature_header("sha256=000F10FF").unwrap();

        assert_eq!(signature.bytes(), &[0, 15, 16, 255]);
    }

    #[test]
    fn returns_an_error_for_hex_strings_of_odd_length() {
        let signature = EventSignature::from_signature_header("AAB");

        assert_eq!(
            signature.unwrap_err().to_string(),
            "Hex string must contain an even amount of ASCII characters: AAB"
        );
    }

    #[test]
    fn returns_an_error_for_strings_that_contain_non_ascii_characters() {
        let signature = EventSignature::from_signature_header("BDÄBCD");

        assert_eq!(
            signature.unwrap_err().to_string(),
            "Hex string must contain an even amount of ASCII characters: BDÄBCD"
        );
    }

    #[test]
    fn returns_an_error_for_strings_that_contain_non_hex_characters() {
        let signature = EventSignature::from_signature_header("CDAGBC");

        assert_eq!(
            signature.unwrap_err().to_string(),
            "Hex tuple could not be parsed: AG"
        );
    }
}

#[cfg(test)]
mod verifier_tests {
    use super::*;

    const EXAMPLE_SECRET: &str = "It's a Secret to Everybody";
    const EXAMPLE_PAYLOAD: &[u8] = b"Hello, World!";
    const EXAMPLE_SIGNATURE: &str =
        "757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17";

    #[test]
    fn returns_true_for_the_example_provided_by_github() {
        let verifier = EventVerifier::new(EXAMPLE_SECRET);
        let signature = EventSignature::from_signature_header(EXAMPLE_SIGNATURE).unwrap();

        let result = verifier.payload_is_valid(EXAMPLE_PAYLOAD, &signature);

        assert!(result);
    }

    #[test]
    fn returns_false_for_invalid_signature() {
        let verifier = EventVerifier::new(EXAMPLE_SECRET);
        let signature = EventSignature::from_signature_header("12ab35").unwrap();

        let result = verifier.payload_is_valid(EXAMPLE_PAYLOAD, &signature);

        assert!(!result);
    }

    #[test]
    fn returns_false_for_invalid_payload() {
        let verifier = EventVerifier::new(EXAMPLE_SECRET);
        let payload = b"invalid";
        let signature = EventSignature::from_signature_header(EXAMPLE_SIGNATURE).unwrap();

        let result = verifier.payload_is_valid(payload, &signature);

        assert!(!result);
    }
}
