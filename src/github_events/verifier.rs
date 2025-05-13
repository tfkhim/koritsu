/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use hmac::{Hmac, Mac};
use sha2::Sha256;

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

pub struct EventSignature {
    bytes: Box<[u8]>,
}

impl EventSignature {
    pub fn from_signature_header(sha256_signature_header: &str) -> EventSignature {
        let hex_string = sha256_signature_header
            .strip_prefix("sha256=")
            .unwrap_or(sha256_signature_header);

        assert!(hex_string.len() % 2 == 0);

        let bytes = (0..hex_string.len())
            .step_by(2)
            .map(|i| {
                let hex_tuple = hex_string
                    .get(i..i + 2)
                    .expect("Hex string must only single byte characters");
                u8::from_str_radix(hex_tuple, 16).expect("Invalid Hex characters")
            })
            .collect();

        EventSignature { bytes }
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_signature_header_without_prefix() {
        let signature = EventSignature::from_signature_header("000F10FF");

        assert_eq!(signature.bytes(), &[0, 15, 16, 255]);
    }

    #[test]
    fn can_parse_signature_header_prefix() {
        let signature = EventSignature::from_signature_header("sha256=000F10FF");

        assert_eq!(signature.bytes(), &[0, 15, 16, 255]);
    }

    #[test]
    fn returns_true_for_the_example_provided_by_github() {
        let verifier = EventVerifier::new("It's a Secret to Everybody");
        let payload = b"Hello, World!";
        let signature = EventSignature::from_signature_header(
            "757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17",
        );

        let result = verifier.payload_is_valid(payload, &signature);

        assert!(result);
    }

    #[test]
    fn can_handle_signature_with_sha256_prefix() {
        let verifier = EventVerifier::new("It's a Secret to Everybody");
        let payload = b"Hello, World!";
        let signature = EventSignature::from_signature_header(
            "sha256=757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17",
        );

        let result = verifier.payload_is_valid(payload, &signature);

        assert!(result);
    }

    #[test]
    fn returns_false_for_invalid_signature() {
        let verifier = EventVerifier::new("It's a Secret to Everybody");
        let payload = b"Hello, World!";
        let signature = EventSignature::from_signature_header("12ab35");

        let result = verifier.payload_is_valid(payload, &signature);

        assert!(!result);
    }

    #[test]
    fn returns_false_for_invalid_payload() {
        let verifier = EventVerifier::new("It's a Secret to Everybody");
        let payload = b"invalid";
        let signature = EventSignature::from_signature_header(
            "sha256=757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17",
        );

        let result = verifier.payload_is_valid(payload, &signature);

        assert!(!result);
    }
}
