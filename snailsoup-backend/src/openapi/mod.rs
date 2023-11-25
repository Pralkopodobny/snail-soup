mod api_doc;
mod security;

use std::sync::Arc;

use utoipa::OpenApi;
use utoipa_swagger_ui::Config;
use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Filter, Rejection, Reply,
};

pub use api_doc::ApiDoc;
pub use security::SecurityAddon;

const SWAGGER_UI_PATH: &str = "/swagger-ui";

pub fn openapi_filters(
    config: Arc<Config<'static>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    apidoc_filter().or(swaggerui_filter(config))
}

fn apidoc_filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()))
}

fn swaggerui_filter(
    config: Arc<Config<'static>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(swagger_handler)
}

async fn swagger_handler(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == SWAGGER_UI_PATH {
        return Ok(Box::new(warp::redirect::found(Uri::from_static(
            SWAGGER_UI_PATH,
        ))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
