#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8000")?;

    hc.do_get("/hello?name=prashant").await?.print().await?;
    hc.do_get("/hello2/muskan").await?.print().await?;

    // hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!(
            {
                "username": "demo1",
                "pwd": "welcome"
            }
        ),
    );
    // req_login.await?.print().await?;

    // Ticket testing
    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!(
            {
                "title": "Learn Rust"
            }
        ),
    );

    req_create_ticket.await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;
    //hc.do_delete("/api/tickets/1").await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
