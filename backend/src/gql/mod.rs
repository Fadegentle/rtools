pub mod mutations;
pub mod queries;

use actix_web::{web, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use self::queries::QueryRoot;

use super::dbs::mysql::my_pool;
use super::util::constant::CFG;

type ActixSchema =
    Schema<queries::QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub async fn build_schema() -> ActixSchema {
    let my_pool = my_pool().await;
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(my_pool).finish()
}

pub async fn graphql(schema: web::Data<ActixSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(playground_source(
        GraphQLPlaygroundConfig::new(CFG.get("GQL_VER").unwrap())
            .subscription_endpoint(CFG.get("GQL_VER").unwrap()),
    )))
}
