use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::{DateTime, Utc};

pub const REFRESH_TOKEN_NAME: &str = "refreshToken";

pub fn create_refresh_token_cookie(
    jar: CookieJar,
    token: &str,
    max_age: &DateTime<Utc>,
) -> CookieJar {
    let expires =
        time::OffsetDateTime::now_utc() + time::Duration::days(max_age.timestamp() / 86400);
    let max_age = time::Duration::days(max_age.timestamp() / 86400);

    let refresh_cookie = Cookie::build((REFRESH_TOKEN_NAME, token.to_string()))
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(max_age)
        .expires(expires)
        .build();

    let jar = jar.add(refresh_cookie);

    jar
}

pub fn remove_refresh_token_cookie(jar: CookieJar) -> CookieJar {
    jar.remove(REFRESH_TOKEN_NAME)
}
