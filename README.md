# cait-sith
A Rust implementation of a Steam Deck library suggester

## Purpose
Create a simple Rust program that generates a list of games to play that are Steam Deck Verified/Playable, and can be sorted by HowLongToBeat times. Calls ProtonDB directly to retreive JSON data for each owned appId.

## Usage
```
cait-sith STEAM_USER_ID API_KEY
```

## Libraries
### Major Features
https://crates.io/crates/reqwest
https://crates.io/crates/howlongtobeat
https://crates.io/crates/serde
https://crates.io/crates/serde_json

### Minor Features
https://crates.io/crates/libsqlite3-sys
https://crates.io/crates/ratatui


## TODO

### Technical
- [ ] Data Structure for storing a game and its info
- [ ] API calls to Steam to retrieve library data
- [ ] API calls to Proton to retrieve compatibility data
- [ ] API calls to HLTB to retrieve playtime data

### Sorting
- [ ] HLTB
- [ ] Proton Status
- [ ] Steam Deck Status
- [ ] Release Date
- [ ] Recently Played
- [ ] Steam Review Rating

### Input
SteamID (requires public library access)
Release Year Range (optional)

### Operations
- [ ] Create Game Library
- [ ] Update Game Library

### Output
- [ ] CSV/Spreadsheet
- [ ] Local Database
- [ ] Console TUI
