# cait-sith
A Rust implementation of a Steam Deck library suggester


## Purpose
Create a simple Rust program that generates a list of games to play that are Steam Deck Verified/Playable, and can be sorted by HowLongToBeat times

## Libraries
### Major Features
https://crates.io/crates/rsteam
https://crates.io/crates/proton-api-rs
https://crates.io/crates/howlongtobeat

### Minor Features
https://crates.io/crates/libsqlite3-sys
https://crates.io/crates/ratatui
https://crates.io/crates/serde
https://crates.io/crates/serde_json


## TODO

### Technical
- [ ] Data Structure for storing a game and its info
- [ ] API calls to Steam to retrieve library data
- [ ] API calls to Proton to retrieve compatibility data
- [ ] API calls to HLTB to retrieve playtime data

### Sorting
- [ ] HLTB
- [ ] Steam Deck Proton Status
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
