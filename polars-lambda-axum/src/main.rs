use axum::{extract::Path, response::Json, routing::get, Router};
use lambda_http::{self, service_fn, Error, Request, RequestExt};
use polars_lambda_axum::calculate;
use serde_json::{json, Value};
use tracing_subscriber;

async fn root() -> &'static str {
    "Hello, Polars"
}

async fn get_filter(Path(value): Path<f64>) -> Json<Value> {
    let df = calculate(value).unwrap();
    let json = json!({
        "payload": format!("{}", df),
    });
    Json(json)
}

// Lambda handler
async fn lambda_handler(event: Request) -> Result<lambda_http::Response<String>, Error> {
    let value: f64 = event
        .query_string_parameters()
        .first("filter")
        .unwrap_or("5")
        .parse()
        .unwrap_or(5.0);

    let df = calculate(value).unwrap();
    let payload = json!({ "payload": format!("{}", df) });

    Ok(lambda_http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(payload.to_string())
        .unwrap())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let run_in_lambda = std::env::var("LAMBDA_RUNTIME").is_ok();

    if run_in_lambda {
        let handler = service_fn(lambda_handler);
        lambda_http::run(handler)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
    } else {
        let app = Router::new()
            .route("/", get(root))
            .route("/iris/filter/:value", get(get_filter));

        let addr = "127.0.0.1:3000".parse()?;
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }

    Ok(())
}
