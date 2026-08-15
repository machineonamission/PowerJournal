#![feature(try_blocks)]
use crate::blob_utils::infer_mime_type;
use crate::database::entity::prelude::*;
use crate::database::init_db;
use crate::importers::common::ImporterArgs;
use crate::text;
use anyhow::{Context, Result, anyhow};
use bytes::Bytes;
use chrono::{Local, NaiveDate, TimeZone};
use dioxus::prelude::*;
use glam::DVec3;
use heic::{DecoderConfig, PixelLayout};
use image::RgbaImage;
use lightningcss::properties::Property;
use lightningcss::stylesheet::{ParserOptions, StyleAttribute};
use lightningcss::values::color::CssColor;
use scraper::{Html, Selector};
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use sea_orm::{DatabaseTransaction, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;
use std::io::{Cursor, Read, Seek};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Semaphore;
use webp::Encoder;
use zip::ZipArchive;

fn selector(s: &str) -> Result<Selector> {
    Selector::parse(s).map_err(|e| anyhow!("{e:?}"))
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Location {
    longitude: f64,
    city: String,
    type_of_place: Option<String>,
    latitude: f64,
    place_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LocationSidecar {
    visits: Vec<Location>,
}

/// THIS FUNCTION WAS WRITTEN BY GEMINI (with heavy prompting)
pub fn calculate_mood_valence(point: (f64, f64, f64)) -> f64 {
    // see https://github.com/machineonamission/applejournalmoodcolors
    let p = DVec3::from(point);
    // these represent a piecewise RGB gradient that is my best reconstruction of what the export uses internally
    // these were computed like a while ago
    let gvertices = [
        DVec3::new(254.96, 238.04, 230.99),
        DVec3::new(248.67, 219.51, 177.07),
        DVec3::new(214.55, 233.19, 179.14),
        DVec3::new(225.79, 230.95, 231.47),
        DVec3::new(215.30, 223.77, 235.97),
        DVec3::new(229.34, 235.03, 255.33),
        DVec3::new(217.02, 209.98, 243.02),
    ];

    // the mood valence each vertex represents
    let valences = [1.0, 2.0 / 3.0, 1.0 / 3.0, 0.0, -1.0 / 3.0, -2.0 / 3.0, -1.0];

    // so basically, apple journal somewhere has a function that, using a piecewise gradient, turns a valence value -1 to 1 into an RGB color.
    // my function is essentially the INVERSE of that, reconstructing the color from the value
    // the task of this code is to map `point` to somewhere along the gradient, and then compute the input valence that would have resulted in that color
    // this is the best way to reconstruct a valence value from a normal journal export since we aren't given the raw valence value directly.
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

/// WRITTEN BY GEMINI
/// Parses a date string like "Thursday, January 3, 2019" into a Unix timestamp
/// (seconds since the UNIX epoch) in the system's local timezone.
pub fn parse_date_to_unix_timestamp(date_str: &str) -> Result<i64> {
    // 1. Define the format string matching your input
    // %A = Full weekday name (Thursday)
    // %B = Full month name (January)
    // %d = Day of the month (3)
    // %Y = 4-digit year (2019)
    let format = "%A, %B %d, %Y";

    // 2. Parse the string into a NaiveDate (a date without time or timezone)
    let naive_date = NaiveDate::parse_from_str(date_str, format)?;

    // 3. Give it a time so we can convert it to a timestamp (assuming noon)
    // and_hms_opt takes (hour, minute, second)
    let naive_datetime = naive_date.and_hms_opt(12, 0, 0).context("hms opt fail")?;

    // 4. Bind it to the Local timezone
    // We use .earliest() to handle potential Daylight Saving Time edge cases
    // (e.g., if a DST shift happens exactly at midnight, making the time ambiguous)
    let local_datetime = Local
        .from_local_datetime(&naive_datetime)
        .earliest()
        .context("Failed to map local time due to timezone ambiguity")?;

    // 5. Extract the Unix timestamp (seconds since Jan 1, 1970)
    Ok(local_datetime.timestamp())
}

fn decode_heic_to_rgba(buf: &[u8]) -> Result<RgbaImage> {
    let decoded = DecoderConfig::new().decode(buf, PixelLayout::Rgba8)?;
    RgbaImage::from_raw(decoded.width, decoded.height, decoded.data)
        .context("decoder buffer size matches declared dimensions")
}

fn heic_to_png(buf: Vec<u8>) -> Result<Vec<u8>> {
    let img = decode_heic_to_rgba(&buf)?;

    let mut png_bytes = Vec::new();
    img.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png)?;

    Ok(png_bytes)
}

fn heic_to_lossless_webp(buf: Vec<u8>) -> Result<Vec<u8>> {
    let img = decode_heic_to_rgba(&buf)?;
    Ok(Encoder::from_rgba(&img, img.width(), img.height())
        .encode_lossless()
        .to_vec())
}

fn heic_to_lossy_webp(buf: Vec<u8>, quality: f32) -> Result<Vec<u8>> {
    let img = decode_heic_to_rgba(&buf)?;
    Ok(Encoder::from_rgba(&img, img.width(), img.height())
        .encode(quality)
        .to_vec())
}

// fn normalize_mov(buf: Vec<u8>) -> Result<Vec<u8>> {
//
// }

fn needs_a_normalization(buf: &Vec<u8>) -> bool {
    let mt = infer_mime_type(buf);
    matches!(
        mt,
        "image/heic" | "image/heif" | "image/heic-sequence" | "image/heif-sequence"
    )
}

// apple uses weird formats (eg heic, mov) that browsers (ie, dioxus) wont render. convert on import.
fn normalize_apple_media(buf: Vec<u8>, heic_codec: &str) -> Result<Vec<u8>> {
    match heic_codec {
        "Lossless WebP" => {
            debug!("converting heic to lossless webp");
            heic_to_lossless_webp(buf)
        }
        "Lossy WebP" => {
            debug!("converting heic to lossy webp");
            heic_to_lossy_webp(buf, 80.0)
        }
        "PNG" => {
            debug!("converting heic to png");
            heic_to_png(buf)
        }
        "HEIC" | _ => {
            // the no-op option lmao
            Ok(buf)
        }
    }
    // suprisingly, MOV + HEVC is actually fucking supported in my testing? leaving this here in
    // case it's not as universal, though video encoding is something i hope i wont have to do
    // https://caniuse.com/hevc
    // https://github.com/Fyrd/caniuse/issues/6086
    // "video/quicktime" => {
    //     log_str("converting mov");
    //     normalize_mov(buf)
    // },
}

async fn insert_blob(txn: &DatabaseTransaction, entry_id: i64, buf: Vec<u8>) -> Result<()> {
    let mt = infer_mime_type(&buf);
    piece::ActiveModel::builder()
        .set_piece_2_blob(
            piece_2_blob::ActiveModel::builder()
                .set_mime_type(mt) // assume they're all images for now
                .set_blob(blobs::ActiveModel::builder().set_data(buf)),
        )
        .set_piece_type(2)
        .set_entry_id(entry_id)
        .insert(txn)
        .await?;
    Ok(())
}

pub fn sanitize_apple_html(html: &str) -> Result<String> {
    // placeholder
    // TODO fix apple specific html
    text::sanitize_html(html)
}

/// file must be file-like object representing an Apple Journal export
/// - 3 dots in the top right of the Journal app > Export
/// - once it's exported to a folder, open the files app
/// - long press AppleJournalEntries > Compress
/// use the zip file generated by this!
pub async fn import_apple_journal(mut args: ImporterArgs<'_>) -> Result<()> {
    let ImporterArgs {
        file,
        db,
        mut log_signal,
        mut current_prog_signal,
        mut max_prog_signal,
        importer_options,
    } = args;

    let mut log = move |message: String| {
        log_signal.write().push(message);
    };

    let mut log_str = move |message: &str| {
        log(message.to_string());
    };

    log_str("Beginning import...");

    let codec = importer_options
        .heic_codec
        .unwrap_or("Lossless WebP".into());

    // parse zip
    let cursor = Cursor::new(file);
    let mut masterzip = ZipArchive::new(cursor)?;

    // init db
    let txn = db.begin().await?;

    // some selectors we use for html
    let assets_s = selector(".assetGrid")?;
    let item_s = selector(".gridItem")?;
    let photo_s = selector(".assetType_photo")?;
    let video_s = selector(".assetType_video, .assetType_audio")?;
    let som_s = selector(".assetType_stateOfMind")?;
    let location_s = selector(".assetType_multiPinMap")?;
    let date_s = selector(".pageHeader")?;

    let title_s = selector(".title")?;
    let body_s = selector(".bodyText")?;

    let img = selector("img")?;
    let source = selector("source")?;

    let mut import_journal = journal::ActiveModel::builder().set_title("Apple Journal Import");

    let import_journal = import_journal.insert(&txn).await?;

    log_str("Calculating backup size...");

    let mut entries = Vec::<PathBuf>::new();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Result<(Vec<u8>, i64)>>();
    let mut spawned = 0;
    for i in 0..masterzip.len() {
        // read file contents if it is an entry
        // inside block because getting a ZipFile object is a mutable borrow on masterzip,
        // and you can only do one of those at a time, so we gotta let file go out of scope
        {
            let mut file = masterzip.by_index(i)?;
            let path = file.enclosed_name().context("zip enclosed name error")?;

            // count ESTIMATE for amount of work to do
            if (path.starts_with("AppleJournalEntries/Entries")
                || (path.starts_with("AppleJournalEntries/Resources")
                    && !path.extension().map(|ext| ext == "json").unwrap_or(false)))
                && !file.is_dir()
            {
                *max_prog_signal.write() += 1;
            }

            if !path.starts_with("AppleJournalEntries/Entries") || file.is_dir() {
                continue;
            }
            entries.push(path)
        }
    }
    // max_prog_signal.set(entries.len() as i64);

    // stuff needed for parallel re-encoding later
    let semaphore = Arc::new(Semaphore::new(std::thread::available_parallelism()?.get()));

    // for every file in the zip
    for i in 0..masterzip.len() {
        // read file contents if it is an entry
        // inside block because getting a ZipFile object is a mutable borrow on masterzip,
        // and you can only do one of those at a time, so we gotta let file go out of scope
        let mut buf: String = String::new();
        {
            let mut file = masterzip.by_index(i)?;
            let path = file.enclosed_name().context("zip enclosed name error")?;
            if !path.starts_with("AppleJournalEntries/Entries") || file.is_dir() {
                continue;
            }
            log(format!("Processing entry {:?}", &path));
            file.read_to_string(&mut buf)?;
        }

        // for every entry in the zip:

        // init entry db object
        let mut master_entry = entries::ActiveModel::builder();

        master_entry = master_entry.set_journal_id(import_journal.id);

        let document = Html::parse_document(&*buf);

        // parse datetime
        let dt_str = document.select(&date_s).next().context("date fail")?;
        let dt = parse_date_to_unix_timestamp(dt_str.text().collect::<String>().as_str())?;
        master_entry = master_entry.set_datetime(dt);

        // title
        let title = document.select(&title_s).next();
        if let Some(title) = title {
            let title_text = title.text().collect::<String>();
            master_entry = master_entry.set_title(title_text);
        }

        // body content
        let body = document.select(&body_s).next();
        if let Some(body) = body {
            let body_contents = body.html();
            master_entry = master_entry.add_piece(
                piece::ActiveModel::builder()
                    .set_piece_0_text(
                        piece_0_text::ActiveModel::builder()
                            .set_content(sanitize_apple_html(&body_contents)?),
                    )
                    .set_piece_type(0),
            );
        } else {
            debug!("no body found");
        }

        // grid is where apple journal stores "assets", basically pieces for us
        let grid = document.select(&assets_s).next().context("grid fail")?;

        // mood/"state of mind"
        for element in grid.select(&som_s) {
            // the best mood data we have is the css background color so uh
            // parse out of html
            let color = element.value().attr("style");

            if let Some(color) = color {
                // parse out of css declaration
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
                                // dbg!(&rgba, &valence);
                                // add to entry
                                master_entry = master_entry.add_piece(
                                    piece::ActiveModel::builder()
                                        .set_piece_1_mood(
                                            piece_1_mood::ActiveModel::builder()
                                                .set_pleasantness(valence),
                                        )
                                        .set_piece_type(1),
                                );
                            }
                        }
                    }
                }
            }
        }

        // locations
        for element in grid.select(&location_s) {
            // grab its ID, there's a corresponding .json file with the actual data in it!
            let id = element.value().attr("id").context("id fail")?;
            let mut buf: String = String::new();
            {
                let mut scfile =
                    masterzip.by_path(format!("AppleJournalEntries/Resources/{id}.json"))?;
                scfile.read_to_string(&mut buf)?;
            }

            let json: LocationSidecar = serde_json::from_str(&*buf)?;
            for visit in json.visits {
                master_entry = master_entry.add_piece(
                    piece::ActiveModel::builder()
                        .set_piece_3_location(
                            piece_3_location::ActiveModel::builder()
                                .set_lat(visit.latitude)
                                .set_lon(visit.longitude)
                                .set_name(visit.place_name),
                        )
                        .set_piece_type(3),
                );
            }
        }
        // log_str("Committing to database...");
        let inserted_entry = master_entry.insert(&txn).await?;
        *current_prog_signal.write() += 1;

        let inserted_id = inserted_entry.id;

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
            log(format!("Importing asset {:?}", &canon_path));
            let mut buf: Vec<u8> = Vec::new();

            {
                let mut file = masterzip.by_path(&canon_path)?;
                file.read_to_end(&mut buf)?;
            }

            // HEIC files don't render in dioxus
            if needs_a_normalization(&buf) && codec != "HEIC" {
                log(format!(
                    "Spawning re-encoding thread for asset {:?}",
                    &canon_path
                ));
                let value = codec.clone();
                let permit = semaphore.clone();
                let tx = tx.clone();
                spawned += 1;

                spawn(async move {
                    let result = async move {
                        let owned_permit = permit.acquire_owned().await?;
                        let normalized =
                            tokio::task::spawn_blocking(move || normalize_apple_media(buf, &value))
                                .await??;
                        drop(owned_permit);
                        *current_prog_signal.write() += 1;
                        Ok((normalized, inserted_id))
                    }
                    .await;

                    tx.send(result).unwrap(); // fire-and-forget from the caller's POV
                });
            } else {
                // no reencoding, don't waste time spawning a thread, just insert now
                insert_blob(&txn, inserted_id, buf).await?;
                *current_prog_signal.write() += 1;
            }
        }
    }
    drop(tx);

    if spawned > 0 {
        log_str("Finalizing DB inserts...");
        for i in 0..spawned {
            if let Some(result) = rx.recv().await {
                let (normalized, inserted_id) = result?; // bubbles up errors
                log(format!("Finalizing DB insert for asset {i}/{spawned}..."));
                insert_blob(&txn, inserted_id, normalized).await?;
            }
        }
    }
    current_prog_signal.set(max_prog_signal());
    log_str("Committing database transaction...");
    txn.commit().await?;
    log_str("Finished!");
    Ok(())
}

// pub async fn main() {
//     println!("debug: loading zip");
//     import_apple_journal(fs::File::open("ignore/AppleJournalEntries.zip").expect("file open error"))
//         .await
//         .unwrap()
// }
