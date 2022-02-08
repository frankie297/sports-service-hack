#[derive(serde::Serialize)]
pub struct Standings {
    pub name: String,
    pub updated: String,
    pub players: Vec<Player>
}

#[derive(serde::Serialize)]
pub struct Player {
    pub rank: u16,
    pub name: String,
    pub country : String,
    pub points: String
}