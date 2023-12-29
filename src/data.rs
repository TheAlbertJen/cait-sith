use rusqlite::{Connection, Result, named_params};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::blocking::Client;
// use howlongtobeat::search;

// TODO: scrape store page for game to get deck compatibility
// enum SteamDeckStatus {
//     Verified,
//     Playable,
//     Unsupported
// }

#[derive(Debug, Deserialize)]
struct ProtonDbInfo {
    // confidence: String,
    // score: f32,
    tier: String,
}

#[derive(Debug, Deserialize)]
struct SteamGameInfo {
    #[serde(rename = "appId")]
    app_id: u32,
    name: String,
    playtime_forever: u32,
    playtime_2weeks: u32,
    rtime_last_played: u32,
}

#[derive(Debug, Deserialize)]
struct SteamOwnedGamesResponse {
    games: Vec<SteamGameInfo>,
}

pub struct LocalGameInfo {
    app_id: u32,
    name: String,
    playtime_forever: u32,
    playtime_2weeks: u32,
    rtime_last_played: u32,
    proton_status: String,
    // steam_deck_status: SteamDeckStatus,
    // hltb_main: i32,
    // hltb_completionist: i32,
    last_fetch: u64,
}

impl LocalGameInfo {
    fn new(
        app_id: u32,
        name: String,
        playtime_forever: u32,
        playtime_2weeks: u32,
        rtime_last_played: u32,
        proton_tier: String, /*, steam_deck_status: SteamDeckStatus, hltb_main: i32, hltb_completionist: i32 */
    ) -> Self {
        Self {
            app_id: app_id,
            name: name,
            playtime_forever: playtime_forever,
            playtime_2weeks: playtime_2weeks,
            rtime_last_played: rtime_last_played,
            // hltb_main: hltb_main,
            // hltb_completionist: hltb_completionist,
            proton_status: proton_tier,
            last_fetch: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Timestamp failure")
                .as_secs(),
        }
    }
}

pub struct DataManager {
    http_client: Client,
    db: Connection
}

impl DataManager {
    pub fn new() -> Self {
        let connection = Connection::open("./games.db").unwrap();

        Self {
            http_client: Client::new(),
            db: connection
        }
    }

    fn get_protondb_info(&self, app_id: u32) -> ProtonDbInfo {
        self.http_client.get(format!(
            "https://www.protondb.com/api/v1/reports/summaries/{}.json",
            app_id
        )).send().unwrap()
        .json::<ProtonDbInfo>()
        .unwrap()
    }
    
    pub fn build_local_steam_db(&self, steam_id: u64, api_key: String) -> Result<(), reqwest::Error> {
        // get owned games
        let response = self.http_client.get(
            format!("http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json",
                        api_key,
                        steam_id
                )
            )
        .send().unwrap();

        // response is gzipped (duh)

        let owned_games = response.json::<SteamOwnedGamesResponse>()
        .unwrap()
        .games;
    
        self.db.execute("create table if not exists game_info (
            app_id integer primary key,
            name text not null unique,
            playtime_forever integer default 0,
            playtime_2weeks integer default 0,
            rtime_last_played integer default 0,
            proton_tier text default \"UNKNOWN\"
        )", []).unwrap();

        let local_infos = owned_games.iter().map(|game_info| {
            let proton_info = self.get_protondb_info(game_info.app_id);
            LocalGameInfo::new(
                game_info.app_id,
                game_info.name.to_owned(),
                game_info.playtime_forever,
                game_info.playtime_2weeks,
                game_info.rtime_last_played,
                proton_info.tier.to_uppercase(),
            )
        }).collect::<Vec<LocalGameInfo>>();

        for game_info in local_infos {
            self.update_local_db_entry(&game_info).unwrap();
        }
    
        Ok(())
    }
    
    fn update_local_db_entry(&self, game_info: &LocalGameInfo) -> Result<usize, rusqlite::Error> {
        let upsert_entry_sql = "
            INSERT INTO game_info(
                    app_id,
                    name,
                    playtime_forever,
                    playtime_2weeks,
                    rtime_last_played,
                    proton_tier,
                    last_fetch
                )
                VALUES(
                    :app_id,
                    :name,
                    :playtime_forever,
                    :playtime_2weeks,
                    :rtime_last_played,
                    :proton_tier,
                    :last_fetch
                )
                ON CONFLICT(app_id) DO UPDATE SET
                    name=:name,
                    playtime_forever=:playtime_forever,
                    playtime_2weeks=:playtime_2weeks,
                    rtime_last_played=:rtime_last_played,
                    proton_tier=:proton_tier,
                    last_fetch=:last_fetch
        ";
        self.db.execute(upsert_entry_sql, named_params! {
            ":app_id": game_info.app_id,
            ":name": game_info.name,
            ":playtime_forever": game_info.playtime_forever,
            ":playtime_2weeks": game_info.playtime_2weeks,
            ":rtime_last_played": game_info.rtime_last_played,
            ":proton_tier": game_info.proton_status,
            ":last_fetch": game_info.last_fetch
        })
    }
}
