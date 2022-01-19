use crate::user::user::*;
use async_graphql::*;

#[derive(Debug, SimpleObject)]
pub struct AuthToken {
    pub token: String,
}

pub async fn login_user(username: String, password: String) -> Result<AuthToken, Error> {
    let token = "1234";

    let user = find_user_by_username(username).await;

    if user.unwrap().password != password {
        let err = Error::new("Password is incorrect")
            .extend_with(|_, e| e.set("details", "CAN_NOT_FETCH"));
        return Err(err);
    }

    Ok(AuthToken {
        token: token.to_string(),
    })
}
