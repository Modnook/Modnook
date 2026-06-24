use crate::{
    api::AppContext,
    database::schema::user::{User, UserId},
    error::ModnookResult,
};

struct UserRepository;

impl UserRepository {
    pub async fn user_by_id(user_id: UserId, context: &mut AppContext) -> ModnookResult<User> {
        let user = User::get_by_user_id(&mut context.database, user_id).await?;

        Ok(user)
    }
}
