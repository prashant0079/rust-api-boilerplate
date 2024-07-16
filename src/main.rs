#![allow(unused)] //For beginning

pub use self::error::{Error, Result};


use axum::extract::{Path, Query};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{Method, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Json, Router, Server};
use ctx::Ctx;
use log::log_request;
use model::ModelController;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;
mod log;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Model Controller
    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(hello_routes())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(mc.clone(), web::auth::mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("->> LISTENING on {addr}\n");
    Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
    // region:  --- Start Server
}

async fn main_response_mapper(ctx: Option<Ctx>,
	uri: Uri,
	req_method: Method,
	res: Response,) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let uuid = Uuid::new_v4();

    // Get the potential response error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // If client error, build the new response
    let error_response = client_status_error.as_ref()
                                        .map(|(status_code, client_error)| {
                                            let client_error_body = json!({
                                                "error": {
                                                    "type": client_error.as_ref(),
                                                    "req_uuid": uuid.to_string()
                                                }
                                            });

                                            println!("   ->> client_error_body {client_error_body}");
                                            // Build the new response from client error body
                                            return (*status_code, Json(client_error_body)).into_response();
                                        });
    
 	let client_error = client_status_error.unzip().1;
	log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

	println!();
	error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// compartmentalisation
fn hello_routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hellopath)) //a router expression
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// region:    --- Handler Hello
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!!!</strong>"))
}
// endregion: --- Handler Hello

// region:    --- Handler HelloPath
async fn handler_hellopath(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}!!!</strong>"))
}
// endregion: --- Handler HelloPath
