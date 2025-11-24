use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

pub fn jwt_encode<T: Serialize>(claims: &T, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref())
    )
}

pub fn jwt_decode<T: for<'de> Deserialize<'de>>(
    token: &str, 
    secret: &str
) -> Result<T, jsonwebtoken::errors::Error> {
    let token_data = decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )?;
    Ok(token_data.claims)
}