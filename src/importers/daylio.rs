use crate::database;
use crate::database::entity::prelude::*;
use crate::database::init_db;
use anyhow::{anyhow, Result};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use sea_orm::ActiveValue;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::io::{Read, Seek};
use chrono::Datelike;
use zip::ZipArchive;

#[derive(Serialize, Deserialize, Debug)]
struct Mood {
    id: i32,
    mood_group_id: i32, // 1-5
}

#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Entry {
    id: i32,
    datetime: i64,
    time_zone_offset: i64,
    mood: i32,
    tags: Vec<i32>,
    assets: Vec<i32>,
    note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Asset {
    id: i32,
    checksum: String,
    r#type: i32,
    created_at: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DaylioBackup {
    version: i32,
    custom_moods: Vec<Mood>,
    tags: Vec<Tag>,
    day_entries: Vec<Entry>,
    assets: Vec<Asset>,
}

///
///
/// # Arguments
///
/// * `file`: file-like object that is a .daylio BACKUP.
/// More > Backup & Restore > Advanced Options > Export
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub async fn import_daylio<R: Read + Seek + Debug>(file: R) -> Result<()> {
    // parse zip
    let mut masterzip = ZipArchive::new(file)?;
    {
        for name in masterzip.file_names() {
            dbg!(name);
        }
    }


    let mut buf: String = String::new();
    {
        // grab main file
        let mut master = masterzip.by_name("backup.daylio")?;
        // read to string
        master.read_to_string(&mut buf)?;
    }
    // get rid of any newlines
    buf = buf.replace("\n", "");
    // the file is base 64 for some confounding reason, decode
    let bytes = BASE64_STANDARD.decode(buf)?;
    // bytes to utf-8
    let decoded_string = String::from_utf8(bytes)?;
    // fs::write("ignore/decoded.json", &decoded_string)?;
    // string to json
    let json: DaylioBackup = serde_json::from_str(&*decoded_string)?;
    // dbg!(json);
    // init db
    let db = init_db().await?;

    // map FROM daylio IDs TO powerjournal IDs
    let mut tag_map: HashMap<i32, i32> = HashMap::new();

    for tag in json.tags {
        let model = database::entity::activities::ActiveModel {
            name: Set(tag.name),
            emoji: Set("❤️".into()),
            ..Default::default()
        }
        .insert(&db)
        .await?;
        // dbg!(&model);
        tag_map.insert(tag.id, model.id as i32);
    }

    // from daylio id to mood valence (apple journal style -1 to 1 f64)
    let mut mood_map: HashMap<i32, f64> = HashMap::new();

    for mood in json.custom_moods {
        mood_map.insert(mood.id, ((mood.mood_group_id as f64) - 3.0) / 2.0);
    }

    // daylio to pj id
    let mut asset_map: HashMap<i32, i32> = HashMap::new();
    // TODO: this shit brokey cause you cant insert assets without a fucking entry. need to change the order i do this
    for asset in json.assets {
        let checksum = asset.checksum;
        let asset_dt = chrono::DateTime::from_timestamp_millis(asset.created_at).ok_or(anyhow!("epic fail"))?;
        let year = asset_dt.year();
        let month = asset_dt.month();
        dbg!(&checksum);
        let mut file_in_zip = masterzip.by_name(&format!("/assets/photos/{year}/{month}/{checksum}"))?;
        let mut buf: Vec<u8> = Vec::new();
        file_in_zip.read_to_end(&mut buf)?;
        let model = database::entity::piece_2_blob::ActiveModel {
            r#type: Set(0),
            data: Set(buf),
            ..Default::default()
        }
            .insert(&db)
            .await?;
        asset_map.insert(asset.id, model.id as i32);
    }
    println!("finished!");
    Ok(())
}

pub async fn main() {
    import_daylio(fs::File::open("ignore/backup_2026_07_23 (1).daylio").expect("file open error"))
        .await
        .unwrap()
}
