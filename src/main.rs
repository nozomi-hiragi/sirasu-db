use actix_web::{
    get,
    http::header::HeaderMap,
    post,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;

mod gql;
mod utils;

use crate::{
    gql::schema::{build_schema, ApiSchema},
    utils::params::Params,
};

fn get_token_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| String::from(s)).ok())
}

#[post("/")]
async fn index(
    schema: web::Data<ApiSchema>,
    req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_req.into_inner();
    if let Some(token) = get_token_from_headers(req.headers()) {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

#[get("/")]
async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let params = Params::new();
    let hostname = format!("127.0.0.1:{}", params.port);
    println!("listen: {}", hostname);
    HttpServer::new(move || {
        let app = App::new()
            .app_data(Data::new(build_schema()))
            .service(index);
        if cfg!(debug_assertions) {
            app.service(index_playground)
        } else {
            app
        }
    })
    .bind(hostname)?
    .run()
    .await
}
