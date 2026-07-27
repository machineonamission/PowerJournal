use crate::database;
use crate::database::entity::prelude::*;
use crate::database::init_db;
use anyhow::{anyhow, Context, Result};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::{Datelike, Utc};
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::{ActiveValue, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::io::{Read, Seek};
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
    note_title: Option<String>,
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

fn ms_to_datetime(ms: i64) -> Result<chrono::DateTime<Utc>> {
    chrono::DateTime::from_timestamp_millis(ms).context("epic datetime fail")
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
    println!("beginning import");
    // parse zip
    let mut masterzip = ZipArchive::new(file)?;
    // {
    //     for name in masterzip.file_names() {
    //         dbg!(name);
    //     }
    // }

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
    println!("file decoded, initing db");

    let db = init_db().await?;

    let txn = db.begin().await?;

    println!("mapping daylio IDs");

    // map FROM daylio IDs TO powerjournal IDs
    let mut tag_map: HashMap<i32, i64> = HashMap::new();

    for tag in json.tags {
        let model = activities::ActiveModel::builder()
            .set_name(tag.name)
            .set_emoji("❤️")
            .insert(&txn)
            .await?;

        // dbg!(&model);
        tag_map.insert(tag.id, model.id);
    }

    // from daylio id to mood valence (apple journal style -1 to 1 f64)
    let mut mood_map: HashMap<i32, f64> = HashMap::new();

    for mood in json.custom_moods {
        mood_map.insert(mood.id, ((mood.mood_group_id as f64) - 3.0) / 2.0);
    }

    // key is checksum
    // value is zip index
    let mut checksum_to_zip_index_map: HashMap<String, usize> = HashMap::new();

    for i in 0..masterzip.len() {
        let mut file = masterzip.by_index(i)?;
        let buf = file.enclosed_name().context("zip enclosed name error")?;
        let name = buf.file_name().context("file name error")?;
        let strname = name
            .to_os_string()
            .into_string()
            .map_err(|_| anyhow!("osstring to string fail"))?;
        checksum_to_zip_index_map.insert(strname, i);
    }

    // key is daylio asset id
    // value is ZIP INDEX
    let mut asset_id_to_zip: HashMap<i32, usize> = HashMap::new();
    for asset in json.assets {
        asset_id_to_zip.insert(
            asset.id,
            checksum_to_zip_index_map
                .get(&asset.checksum)
                .cloned()
                .context("zip index lookup fail")?,
        );
    }

    for entry in json.day_entries {
        println!("entry {}", entry.id);
        let mut master_entry = entries::ActiveModel::builder()
            .set_datetime(entry.datetime / 1000) // daylio does ms, i do s like a NORMAL PERSON
            .set_title(entry.note_title.clone())
            .add_piece(
                piece::ActiveModel::builder().set_piece_1_mood(
                    piece_1_mood::ActiveModel::builder().set_pleasantness(
                        *mood_map
                            .get(&entry.mood)
                            .context("couldnt get valence for mood")?,
                    ),
                ),
            );

        if let Some(note) = entry.note {
            master_entry = master_entry.add_piece(
                piece::ActiveModel::builder().set_piece_0_text(
                    piece_0_text::ActiveModel::builder()
                        .set_title(entry.note_title.clone())
                        .set_content(note),
                ),
            );
        }

        if !entry.tags.is_empty() {
            let mut activity_piece = piece::ActiveModel::builder();
            for tag in entry.tags {
                activity_piece = activity_piece.add_piece_4_activity(
                    piece_4_activities::ActiveModel::builder()
                        .set_activity_id(tag_map.get(&tag).cloned().context("epic tag failure")?)
                        .set_value(1),
                )
            }
            master_entry = master_entry.add_piece(activity_piece);
        }

        if !entry.assets.is_empty() {
            for asset_id in entry.assets {
                let asset = asset_id_to_zip
                    .get(&asset_id)
                    .cloned()
                    .context("asset reference error")?;
                let mut file_in_zip = masterzip.by_index(asset)?;
                let mut buf: Vec<u8> = Vec::new();
                file_in_zip.read_to_end(&mut buf)?;

                master_entry = master_entry.add_piece(
                    piece::ActiveModel::builder().set_piece_2_blob(
                        piece_2_blob::ActiveModel::builder()
                            .set_data(buf)
                            .set_type(0), // assume they're all images for now
                    ),
                )
            }
        }

        master_entry.insert(&txn).await?;
    }

    // // daylio to pj id
    // let mut asset_map: HashMap<i32, i32> = HashMap::new();
    // // TODO: this shit brokey cause you cant insert assets without a fucking entry. need to change the order i do this
    // for asset in json.assets {
    //     let checksum = asset.checksum;
    //     let asset_dt = chrono::DateTime::from_timestamp_millis(asset.created_at).ok_or(anyhow!("epic fail"))?;
    //     let year = asset_dt.year();
    //     let month = asset_dt.month();
    //     dbg!(&checksum);
    //     let mut file_in_zip = masterzip.by_name(&format!("/assets/photos/{year}/{month}/{checksum}"))?;
    //     let mut buf: Vec<u8> = Vec::new();
    //     file_in_zip.read_to_end(&mut buf)?;
    //     let model = database::entity::piece_2_blob::ActiveModel {
    //         r#type: Set(0),
    //         data: Set(buf),
    //         ..Default::default()
    //     }
    //         .insert(&db)
    //         .await?;
    //     asset_map.insert(asset.id, model.id as i32);
    // }
    txn.commit().await?;
    println!("finished!");
    Ok(())
}

pub async fn main() {
    import_daylio(fs::File::open("ignore/backup_2026_07_23 (1).daylio").expect("file open error"))
        .await
        .unwrap()
}
