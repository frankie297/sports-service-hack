use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

mod standings;
use standings::{Standings, Player};

mod pa_standings;
use pa_standings::{PaStandings, Entry};

fn load_standings() -> Result<Standings, serde_json::Error> {
    serde_json::from_str(include_str!("../standings.json")).map(|pa_data: PaStandings|
        Standings { 
            name: pa_data.name, 
            updated: pa_data.updated, 
            players: pa_data.entries.into_iter().map(get_player).collect()
        }
    )
}

fn get_player(entry: Entry) -> Player {
    Player {
        rank: entry.rank,
        name: entry.participant.name,
        country: entry.participant.country.name,
        points: entry.standings.points.value
    }
}

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    match req.get_method() {
        // Allow GET and HEAD requests.
        &Method::GET | &Method::HEAD => (),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    // Pattern match on the path...
    match req.get_path() {
        "/pa-proxy" => {
            req.with_ttl(60);
            Ok(Response::from_status(StatusCode::OK)
                .with_header(header::CACHE_CONTROL, "max-age=60")
                .with_content_type(mime::APPLICATION_JSON)
                .with_body(include_str!("../standings.json")))
        }

        "/simplified-output" => {
            req.with_ttl(120);
            load_standings().and_then(|s|
                Response::from_status(StatusCode::OK)
                    .with_header(header::CACHE_CONTROL, "max-age=120")
                    .with_content_type(mime::APPLICATION_JSON)
                    .with_body_json(&s)
            ).map_err(|err| Error::new(err))
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
