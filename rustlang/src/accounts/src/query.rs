use crate::user::user::*;
use async_graphql::*;

pub struct Query;

#[Object(extends)]
impl Query {
    async fn me(&self, id: ID) -> Result<Option<User>> {
        Ok(find_user_by_id(id).await)
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> Result<Option<User>> {
        Ok(find_user_by_id(id).await)
    }
}
