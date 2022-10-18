use pasetors::{keys::{SymmetricKey, AsymmetricPublicKey, AsymmetricSecretKey}, version4::{V4, LocalToken, PublicToken}, token::{UntrustedToken, TrustedToken}, Local, errors, Public};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn v4_local_token_encrypt(secret_key: &SymmetricKey<V4>, message: &[u8], footer: Option<&[u8]>, implicit_assert: Option<&[u8]>) -> Result<String, errors::Error> {
  LocalToken::encrypt(secret_key, message, footer, implicit_assert)
}

#[wasm_bindgen]
pub fn v4_local_token_decrypt(secret_key: &SymmetricKey<V4>, token: &UntrustedToken<Local, V4>, footer: Option<&[u8]>, implicit_assert: Option<&[u8]>) -> Result<TrustedToken, errors::Error> {
  LocalToken::decrypt(secret_key, token, footer, implicit_assert)
}

#[wasm_bindgen]
pub fn v4_public_token_sign(secret_key: &AsymmetricSecretKey<V4>, message: &[u8], footer: Option<&[u8]>, implicit_assert: Option<&[u8]>) -> Result<String, errors::Error> {
  PublicToken::sign(secret_key, message, footer, implicit_assert)
}

#[wasm_bindgen]
pub fn v4_public_token_verify(public_key: &AsymmetricPublicKey<V4>, token: &UntrustedToken<Public, V4>, footer: Option<&[u8]>, implicit_assert: Option<&[u8]>) -> Result<TrustedToken, errors::Error> {
  PublicToken::verify(public_key, token, footer, implicit_assert)
}
