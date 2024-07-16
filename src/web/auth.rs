use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;
use crate::error::{Error, Result};
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;

// B means Body
// middleware for authentication layer
pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>, //extractor injected
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

// region:    --- Ctx resolver
pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelController>, //can be used later instead for db connections
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    // Compute Result<Ctx>
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Pattern matching is used keeping async utilites in mind
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
        {
            Ok((user_id, _exp, _signature)) => {
                // TODO: Token validation
                Ok(Ctx::new(user_id))
            }
            Err(e) => Err(e)
        };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie
    if result_ctx.is_err()
        && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie))
        {
           cookies.remove(Cookie::named(AUTH_TOKEN));
        }    

    // Store the ctx_result in the request extension
    // Has to be unique by type - otherwise last write wins
    req.extensions_mut().insert(result_ctx);
    Ok((next.run(req).await))

}

// endregion: --- Ctx resolver


// region:    --- Ctx extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(
		parts: &mut Parts,
		_state: &S,
	) -> std::result::Result<Self, Self::Rejection> {
		println!("->> {:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<Result<Ctx>>()
			.ok_or(Error::AuthFailCtxNotInRequestExtension)?
			.clone()
	}
}

// endregion: --- Ctx extractor

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
