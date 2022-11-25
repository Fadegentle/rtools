use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder, Result};
use async_graphql::{
    extensions::{ApolloTracing, Logger},
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use self::queries::QueryRoot;
use crate::{
    configs::{Configs, GraphQlConfig},
    State,
};

pub mod mutations;
pub mod queries;

pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;
pub type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn build_schema(state: Arc<State>, config: &GraphQlConfig) -> ServiceSchema {
    let builder =
        Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(state).extension(Logger);
    if config.tracing.unwrap_or_default() {
        builder.extension(ApolloTracing).finish()
    } else {
        builder.finish()
    }
}

pub async fn graphql(schema: web::Data<ServiceSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql(config: web::Data<Arc<Configs>>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(playground_source(
        GraphQLPlaygroundConfig::new(&config.graphql.path)
            .subscription_endpoint(&config.graphql.path),
    )))
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(r#"{"status": "up"}"#)
}
