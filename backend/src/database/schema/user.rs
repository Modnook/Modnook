#[derive(Debug, toasty::Embed)]
pub struct UserId(uuid::Uuid);

#[derive(Debug, toasty::Embed)]
pub enum UserRole {
    #[column(variant = "user")]
    User,

    #[column(variant = "helper")]
    Helper,

    #[column(variant = "moderator")]
    Moderator,

    #[column(variant = "admin")]
    Admin,
}

#[derive(Debug, toasty::Model)]
pub struct User {
    #[key]
    #[auto(uuid(v7))]
    user_id: UserId,

    kratos_identity_id: String,

    #[default(UserRole::User)]
    role: UserRole,

    #[unique]
    username: String,

    display_name: String,

    email: Option<String>,

    avatar_url: Option<String>,

    #[auto]
    created_at: jiff::Timestamp,

    #[auto]
    updated_at: jiff::Timestamp,
}
