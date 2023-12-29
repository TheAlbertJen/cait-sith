use argh::{self, FromArgs};
pub mod data;

#[derive(FromArgs)]
#[argh(description = "...")]
struct Params {
    #[argh(option, short = 'u', description = "steam id (SteamID64)")]
    id: u64,

    #[argh(option, short = 'k', description = "steam api key")]
    key: String,
}

fn main() {
    // let _result: String = data::get_protondb_info().await?;

    let params: Params = argh::from_env();
    let manager: data::DataManager = data::DataManager::new();

    println!("SteamID64: {}", params.id);
    println!("API Key: {}", params.key);

    let _ = manager.build_local_steam_db(params.id, params.key);
}
