use crate::database::entity::prelude::*;
use crate::database::init_db;
use anyhow::{anyhow, Context, Error, Result};
use lightningcss::properties::Property;
use lightningcss::stylesheet::{ParserOptions, StyleAttribute};
use lightningcss::values::color::CssColor;
use scraper::{Html, Selector};
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::{ActiveValue, TransactionTrait};
use std::fmt::Debug;
use std::fs;
use std::io::{Read, Seek};
use zip::ZipArchive;

fn selector(s: &str) -> Result<Selector> {
    Selector::parse(s).map_err(|e| anyhow!("{e:?}"))
}

use glam::DVec3;

/// THIS FUNCTION WAS WRITTEN BY GEMINI
pub fn calculate_mood_valence(point: (f64, f64, f64)) -> f64 {
    let p = DVec3::from(point);

    let gvertices = [
        DVec3::new(254.96, 238.04, 230.99),
        DVec3::new(248.67, 219.51, 177.07),
        DVec3::new(214.55, 233.19, 179.14),
        DVec3::new(225.79, 230.95, 231.47),
        DVec3::new(215.30, 223.77, 235.97),
        DVec3::new(229.34, 235.03, 255.33),
        DVec3::new(217.02, 209.98, 243.02),
    ];

    let valences = [1.0, 2.0 / 3.0, 1.0 / 3.0, 0.0, -1.0 / 3.0, -2.0 / 3.0, -1.0];

    gvertices
        .windows(2) // Iterate over adjacent pairs of points (segments)
        .zip(valences.windows(2)) // Pair them with their corresponding valences
        .map(|(v_win, val_win)| {
            let (a, b) = (v_win[0], v_win[1]);
            let (val_a, val_b) = (val_win[0], val_win[1]);

            // Direction vector of the segment, and vector from 'a' to our point
            let dir = b - a;
            let w = p - a;

            // Project point onto the segment line.
            // We clamp between 0.0 and 1.0 to ensure it falls strictly within the segment endpoints.
            let t = (w.dot(dir) / dir.length_squared()).clamp(0.0, 1.0);

            // Calculate exact squared distance from the input point to this segment
            let closest_point_on_segment = a + dir * t;
            let dist_sq = p.distance_squared(closest_point_on_segment);

            // Interpolate the valence based on 't'
            let valence = val_a + t * (val_b - val_a);

            (dist_sq, valence)
        })
        // Find the segment that is mathematically closest to the input point
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .unwrap() // Unwrap is safe here as there are guarantees of valid floats & length > 0
        .1 // Return just the valence
}

pub async fn import_apple_journal<R: Read + Seek + Debug>(file: R) -> Result<()> {
    println!("beginning import");
    // parse zip
    let mut masterzip = ZipArchive::new(file)?;

    // init db
    let db = init_db().await?;
    let txn = db.begin().await?;

    // some selectors we use for html
    let assets_s = selector(".assetGrid")?;
    let item_s = selector(".gridItem")?;
    let photo_s = selector(".assetType_photo")?;
    let video_s = selector(".assetType_video, .assetType_audio")?;
    let som_s = selector(".assetType_stateOfMind")?;
    let location_s = selector(".assetType_multiPinMap")?;
    let img = selector("img")?;
    let source = selector("source")?;

    // for every file in the zip
    for i in 0..masterzip.len() {
        // read file contents if it is an entry
        // inside block because getting a ZipFile object is a mutable borrow on masterzip,
        // and you can only do one of those at a time, so we gotta let file go out of scope
        let mut buf: String = String::new();
        {
            let mut file = masterzip.by_index(i)?;
            let path = file.enclosed_name().context("zip enclosed name error")?;
            if !path.starts_with("AppleJournalEntries/Entries") {
                //
                continue;
            }
            println!("entry {:?}", &path);
            file.read_to_string(&mut buf)?;
        }
        // for every entry in the zip:

        // init entry db object
        let mut master_entry = entries::ActiveModel::builder();
        // .set_datetime(entry.datetime / 1000) // daylio does ms, i do s like a NORMAL PERSON
        // .set_title(entry.note_title.clone());

        let document = Html::parse_document(&*buf);

        // grid is where apple journal stores "assets", basically pieces for us
        let grid = document.select(&assets_s).next().context("grid fail")?;

        // handle blob assets (img, video, audio)
        let mut asset_srcs: Vec<&str> = vec![];

        // because its html, photos are img and video/audio is source

        // get images from img
        for element in grid.select(&photo_s) {
            let img = element.select(&img).next().context("img fail")?;
            let src = img.value().attr("src").context("src fail")?;
            asset_srcs.push(src);
        }

        // get media from video/audio
        for element in grid.select(&video_s) {
            let source = element.select(&source).next().context("source fail")?;
            let src = source.attr("src").context("src fail")?;
            asset_srcs.push(src);
        }

        // for each asset, get the blob data from the zip and add it to the entry
        for src in asset_srcs {
            let path = format!("AppleJournalEntries/Entries/{src}");
            let canon_path = path_clean::clean(path);
            dbg!(&canon_path);
            let mut buf: Vec<u8> = Vec::new();

            {
                let mut file = masterzip.by_path(canon_path)?;
                file.read_to_end(&mut buf)?;
            }
            // add piece with blob
            master_entry = master_entry.add_piece(
                piece::ActiveModel::builder().set_piece_2_blob(
                    piece_2_blob::ActiveModel::builder()
                        .set_data(buf)
                        .set_blob_type(0), // assume they're all images for now
                ),
            )
        }

        for element in grid.select(&som_s) {
            let color = element.value().attr("style").context("color fail")?;

            let options = ParserOptions::default();
            if let Ok(style) = StyleAttribute::parse(color, options) {
                // 2. Iterate through all standard CSS declarations in the style tag
                for declaration in style.declarations.declarations {
                    // 3. Pattern match exactly on the BackgroundColor property
                    if let Property::BackgroundColor(color) = declaration {
                        let rgb = color
                            .to_rgb()
                            .map_err(|e| anyhow!("Failed to convert color to RGB: {:?}", e))?;
                        // 4. Extract the RGBA values from the color enum
                        if let CssColor::RGBA(rgba) = rgb {
                            // calculate valence from rgb
                            let valence = calculate_mood_valence((
                                rgba.red as f64,
                                rgba.green as f64,
                                rgba.blue as f64,
                            ));
                            dbg!(&rgba, &valence);
                            // add to entry
                            master_entry = master_entry.add_piece(
                                piece::ActiveModel::builder().set_piece_1_mood(
                                    piece_1_mood::ActiveModel::builder().set_pleasantness(valence),
                                ),
                            );
                        }
                    }
                }
            }
        }

        for element in grid.select(&location_s) {
            let id = element.value().attr("id").context("id fail")?;
            let mut buf: String = String::new();
            {
                let mut scfile =
                    masterzip.by_path(format!("AppleJournalEntries/Resources/{id}.json"))?;
                scfile.read_to_string(&mut buf)?;
            }
            // TODO: pick up here
        }
    }

    txn.commit().await?;
    println!("finished!");
    Ok(())
}

pub async fn main() {
    println!("debug: loading zip");
    import_apple_journal(fs::File::open("ignore/AppleJournalEntries.zip").expect("file open error"))
        .await
        .unwrap()
}
