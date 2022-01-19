use crate::user::login::*;
use async_graphql::*;

pub struct Mutation;

#[Object(extends)]
impl Mutation {
    async fn login(&self, username: String, password: String) -> Result<AuthToken> {
        login_user(username, password).await
    }
}
