use crate::database::schema::user::{User, UserId};

#[derive(Debug, toasty::Embed)]
pub struct OrganisationId(uuid::Uuid);

#[derive(Debug, toasty::Model)]
pub struct Organisation {
    #[key]
    #[auto(uuid(v7))]
    pub organisation_id: OrganisationId,

    pub organisation_name: String,

    #[unique]
    pub organisation_slug: String,

    #[auto]
    pub created_at: jiff::Timestamp,

    #[auto]
    pub updated_at: jiff::Timestamp,

    #[has_many]
    pub members: toasty::Deferred<Vec<OrganisationMember>>,
}

#[derive(Debug, toasty::Embed)]
pub enum OrganisationMemberRole {
    #[column(variant = "owner")]
    Owner,

    #[column(variant = "admin")]
    Admin,

    #[column(variant = "maintainer")]
    Maintainer,
}

#[derive(Debug, toasty::Model)]
pub struct OrganisationMember {
    #[key]
    pub user_id: UserId,

    #[key]
    pub organisation_id: OrganisationId,

    pub role: OrganisationMemberRole,

    pub tag: Option<String>,

    #[auto]
    pub created_at: jiff::Timestamp,

    #[auto]
    pub updated_at: jiff::Timestamp,

    #[belongs_to(key = user_id, references = user_id)]
    pub user: toasty::Deferred<User>,

    #[belongs_to(key = organisation_id, references = organisation_id)]
    pub organisation: toasty::Deferred<Organisation>,
}

#[derive(Debug, toasty::Model)]
pub struct OrganisationInvite {
    #[key]
    pub user_id: UserId,

    #[key]
    pub organisation_id: OrganisationId,

    #[auto]
    pub created_at: jiff::Timestamp,

    #[auto]
    pub updated_at: jiff::Timestamp,
}
