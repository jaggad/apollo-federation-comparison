use accounts::mutation::Mutation;
use accounts::query::Query;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::graphql;
use std::convert::Infallible;
use warp::{Filter, Reply};

/// Export the schema and serve graphql api via warp
#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, Mutation, EmptySubscription);

    warp::serve(graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, Mutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(warp::reply::json(&schema.execute(request).await).into_response())
        },
    ))
    .run(([0, 0, 0, 0], 4001))
    .await;
}
