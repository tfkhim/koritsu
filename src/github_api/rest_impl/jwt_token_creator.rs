/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::time::{SystemTime, UNIX_EPOCH};

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use rsa::pkcs1::{self, DecodeRsaPrivateKey};
use rsa::sha2::Sha256;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::{RsaPrivateKey, pkcs1v15::SigningKey};
use serde_json::{Error, json};

pub struct JwtTokenCreator {
    header: String,
    client_id: String,
    signing_key: SigningKey<Sha256>,
}

impl JwtTokenCreator {
    pub fn new(client_id: String, private_key_pem: &str) -> Result<Self, pkcs1::Error> {
        let header = BASE64_URL_SAFE_NO_PAD.encode("{ \"alg\": \"RS256\", \"typ\": \"JWT\" }");
        let private_key = RsaPrivateKey::from_pkcs1_pem(private_key_pem)?;
        let signing_key = SigningKey::<Sha256>::new(private_key);
        Ok(Self {
            header,
            signing_key,
            client_id,
        })
    }

    pub fn build_token(&self) -> Result<String, Error> {
        let header_and_payload = format!("{}.{}", self.header, self.build_payload()?);

        let mut rng = rand::thread_rng();
        let signature = self
            .signing_key
            .sign_with_rng(&mut rng, header_and_payload.as_bytes());

        Ok(format!(
            "{header_and_payload}.{}",
            BASE64_URL_SAFE_NO_PAD.encode(signature.to_bytes())
        ))
    }

    fn build_payload(&self) -> Result<String, Error> {
        let seconds_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Unix epoch is always in the past")
            .as_secs();

        let payload = serde_json::to_vec(&json!({
            "iat": seconds_since_epoch - 60,
            "exp": seconds_since_epoch + 120,
            "iss": self.client_id,
        }))?;

        Ok(BASE64_URL_SAFE_NO_PAD.encode(payload))
    }
}
