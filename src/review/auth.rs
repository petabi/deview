use std::sync::{LazyLock, RwLock};

use anyhow::{anyhow, Result};
use chrono::{NaiveDateTime, TimeDelta};
use dioxus::{
    hooks::{use_context, use_resource},
    prelude::*,
    signals::Signal,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use review_database::types::Account;
use serde::{Deserialize, Serialize};

use super::Review;

pub fn SignIn(user: String, password: String) -> Element {
    let review = use_context::<Signal<Review>>();
    let user = use_signal(|| user);
    let password = use_signal(|| password);

    let res = use_resource(move || async move { review().sign_in(&user(), &password()).await });
    rsx! {
        match &*res.read() {
            Some(Ok((account, token))) => {
                // if it is, render the account details
                let username = account.username.to_string();
                rsx! {
                    h1 { "{username}" }
                    p {
                        "{token}"
                    }
                }
            }
            Some(Err(err)) => {
                // if there was an error, render the error
                rsx! {"An error occurred while signing in {err}"}
            }
            None => {
                // if the future is not resolved yet, render a loading message
                rsx! {"Signing in"}
            }
        }
    }
}

impl Review {
    async fn sign_in(&self, user: &str, password: &str) -> Result<(Account, String)> {
        let store = self.store.read().await;
        let account_map = store.account_map();
        let reason = if let Some(mut account) = account_map.get(user)? {
            if account.verify_password(password) {
                let (token, _expiration_time) =
                    create_token(account.username.clone(), account.role.to_string())?;
                account.update_last_signin_time();
                account_map.put(&account)?;
                store.access_token_map().insert(user, &token)?;
                return Ok((account, token));
            }
            "incorrect password"
        } else {
            &format!("user {user} doesn't exist")
        };
        Err(anyhow!("sign in failed: {reason}"))
    }
}

static JWT_EXPIRES_IN: LazyLock<RwLock<u32>> = LazyLock::new(|| RwLock::new(3600));
static JWT_SECRET: LazyLock<RwLock<Vec<u8>>> = LazyLock::new(|| RwLock::new(vec![]));

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
}

impl Claims {
    fn new(sub: String, role: String, exp: i64) -> Self {
        Self { sub, role, exp }
    }
}

/// Creates a JWT token with the given username and role.
///
/// # Errors
///
/// Returns an error if the JWT locks are poisoned or if the JWT secret cannot be read.
pub fn create_token(username: String, role: String) -> Result<(String, NaiveDateTime)> {
    let expires_in = *JWT_EXPIRES_IN.read().map_err(|e| anyhow!(e.to_string()))?;
    let Some(delta) = TimeDelta::try_seconds(expires_in.into()) else {
        unreachable!("`JWT_EXPIRES_IN` is greather than 0 and less than 2^32")
    };
    let exp = chrono::Utc::now() + delta;

    let claims = Claims::new(username, role, exp.timestamp());
    let jwt_secret = JWT_SECRET.read().map_err(|e| anyhow!(e.to_string()))?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&jwt_secret),
    )?;
    let expiration_time = NaiveDateTime::new(exp.date_naive(), exp.time());

    Ok((token, expiration_time))
}
