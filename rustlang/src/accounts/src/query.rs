use async_graphql::{Object, SimpleObject, ID};

#[derive(SimpleObject)]
pub struct User {
    pub id: ID,
    pub username: String,
}

pub struct Query;

#[Object(extends)]
impl Query {
    // Get current User
    async fn me(&self) -> User {
        User {
            id: "1234".into(),
            username: "Me".to_string(),
        }
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> User {
        let username = if id == "1234" {
            "Me".to_string()
        } else {
            format!("User {:?}", id)
        };
        User { id, username }
    }
}
