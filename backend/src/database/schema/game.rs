#[derive(Debug, toasty::Embed)]
pub struct GameId(uuid::Uuid);

#[derive(Debug, toasty::Model)]
pub struct Game {
    #[key]
    #[auto(uuid(v7))]
    pub game_id: GameId,

    pub name: String,

    pub slug: String,

    pub download_count: u32,

    pub project_count: u32,
}
