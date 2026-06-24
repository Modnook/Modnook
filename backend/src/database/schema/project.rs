use crate::database::schema::game::{Game, GameId};
use crate::database::schema::organisation::{Organisation, OrganisationId};
use crate::database::schema::user::{User, UserId};

#[derive(Debug, toasty::Embed)]
pub struct ProjectId(uuid::Uuid);

#[derive(Debug, toasty::Model)]
pub struct Project {
    #[key]
    #[auto(uuid(v7))]
    project_id: ProjectId,

    /// A non unique display name for a project
    ///
    /// HACK: toasty doesn't currently allow for custom indexes in model
    /// defintions, these are instead added as a custom addition to the
    /// normal migration scripts
    ///
    /// see: https://github.com/tokio-rs/toasty/issues/673
    ///
    /// TODO: add trgm indexes
    ///
    project_name: String,

    /// A markdown description of the mod
    ///
    description: String,

    /// A unique string identifier for a project made up of only lowercase
    /// alphanumeric charachters as well as "_"
    ///
    /// For example "extended_cogwheels"
    ///
    #[unique]
    project_slug: String,

    #[index]
    game_id: GameId,

    #[default(0)]
    download_count: u32,

    #[default(0)]
    follow_count: u32,

    #[auto]
    created_at: jiff::Timestamp,

    #[auto]
    updated_at: jiff::Timestamp,

    #[belongs_to(key = game_id, references = game_id)]
    game: Game,
}

#[derive(Debug, toasty::Model)]
pub struct ProjectFollow {
    #[key]
    pub project_id: ProjectId,

    #[key]
    pub user_id: UserId,

    #[auto]
    pub created_at: jiff::Timestamp,

    #[auto]
    pub updated_at: jiff::Timestamp,

    #[belongs_to(key = project_id, references = project_id)]
    pub project: toasty::Deferred<Project>,

    #[belongs_to(key = user_id, references = user_id)]
    pub user: toasty::Deferred<User>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, toasty::Embed)]
pub enum ProjectRole {
    #[column(variant = "maintainer")]
    Maintainer,

    #[column(variant = "admin")]
    Admin,

    #[column(variant = "owner")]
    Owner,
}

#[derive(Debug, toasty::Model)]
pub struct ProjectUserMember {
    #[key]
    pub project_id: ProjectId,
    #[key]
    pub user_id: UserId,
    pub role: ProjectRole,
    /// Optional freeform label shown on the project page, e.g. "Art & textures"
    pub tag: Option<String>,
    #[auto]
    pub created_at: jiff::Timestamp,
    #[auto]
    pub updated_at: jiff::Timestamp,
    #[belongs_to(key = project_id, references = project_id)]
    pub project: toasty::Deferred<Project>,
    #[belongs_to(key = user_id, references = user_id)]
    pub user: toasty::Deferred<User>,
}

#[derive(Debug, toasty::Model)]
pub struct ProjectOrganisationMember {
    #[key]
    pub project_id: ProjectId,
    #[key]
    pub organisation_id: OrganisationId,
    /// The ceiling role granted to org members on this project.
    pub role: ProjectRole,
    #[auto]
    pub created_at: jiff::Timestamp,
    #[auto]
    pub updated_at: jiff::Timestamp,
    #[belongs_to(key = project_id, references = project_id)]
    pub project: toasty::Deferred<Project>,
    #[belongs_to(key = organisation_id, references = organisation_id)]
    pub organisation: toasty::Deferred<Organisation>,
}

#[derive(Debug, toasty::Embed)]
pub struct ReleaseId(uuid::Uuid);

#[derive(Debug, toasty::Model)]
pub struct Release {
    #[key]
    #[auto(uuid(v7))]
    release_id: ReleaseId,

    project_id: ProjectId,

    #[default(0)]
    download_count: u32,
}
