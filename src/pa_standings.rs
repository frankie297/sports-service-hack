#[derive(serde::Deserialize)]
pub struct PaStandings {
    pub id: u32,
    pub name: String,
    pub sport: Sport,
    pub updated: String,
    pub entries: Vec<Entry>,
    pub links: Vec<Links>
}

#[derive(serde::Deserialize)]
pub struct Sport {
    pub id: u16,
    pub name: String
}

#[derive(serde::Deserialize)]
pub struct Entry {
    pub id: u32,
    pub rank: u16,
    pub participant: Participant,
    pub standings: Standings
}

#[derive(serde::Deserialize)]
pub struct Participant {
    pub id: u32,
    pub name: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub gender: String,
    #[serde(rename = "type")]
    pub type_of: String,
    pub country: Country,
}

#[derive(serde::Deserialize)]
pub struct Country {
    pub id: u16,
    pub name: String
}

#[derive(serde::Deserialize)]
pub struct Standings {
    pub points: Points
}

#[derive(serde::Deserialize)]
pub struct Points {
    pub code: String,
    pub name: String,
    pub value: String
}

#[derive(serde::Deserialize)]
pub struct Links {
    pub rel: String,
    pub href: String
}