use axum::http::StatusCode;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use crate::utils;

#[derive(Serialize,Deserialize)]
pub struct Claims{
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn encode_jwt(email:String)-> Result<String,StatusCode>{
    let now: DateTime<Utc> = Utc::now();
    let expire:Duration = Duration::hours(24);
    
    let claim = Claims{iat:now.timestamp() as usize,exp:(now+expire).timestamp() as usize,email };
    let secret = (*utils::constants::TOKEN).clone();
    
    encode(&Header::default(),&claim,&EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_|{StatusCode::INTERNAL_SERVER_ERROR})
}

pub fn decode_jwt(jwt:String) -> Result<TokenData<Claims>,StatusCode>{
    let secret = (*utils::constants::TOKEN).clone();
    let res = decode(&jwt,&DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map_err(|_|{StatusCode::INTERNAL_SERVER_ERROR});
    res
}