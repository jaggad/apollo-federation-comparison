use async_graphql::*;

#[derive(Debug, SimpleObject)]
pub struct User {
    pub id: ID,
    pub username: String,
    pub password: String,
}

fn user_data() -> std::array::IntoIter<User, 4> {
    let users = [
        User {
            id: "1".into(),
            username: "@apple".to_string(),
            password: "password".to_string(),
        },
        User {
            id: "2".into(),
            username: "@orange".to_string(),
            password: "password".to_string(),
        },
        User {
            id: "3".into(),
            username: "@pear".to_string(),
            password: "password".to_string(),
        },
        User {
            id: "4".into(),
            username: "@kiwi".to_string(),
            password: "password".to_string(),
        },
    ];

    users.into_iter()
}

pub async fn find_user_by_id(id: ID) -> Option<User> {
    user_data().find(|user| user.id == id)
}

pub async fn find_user_by_username(username: String) -> Option<User> {
    user_data().find(|user| user.username == username)
}
