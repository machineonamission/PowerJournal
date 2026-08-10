use crate::database::entity::prelude::*;
use dioxus::desktop::{AssetRequest, RequestAsyncResponder, use_asset_handler};
use dioxus::prelude::*;
use sea_orm::{DatabaseConnection, EntityLoaderTrait, EntityTrait, QueryOrder};

// lets the renderer have an image like <img src="/dbimage/1" /> and it will fetch the image from the database
pub fn register_blob_asset(db_signal: Resource<DatabaseConnection>) {
    use_asset_handler(
        "dbimage",
        move |request: AssetRequest, responder: RequestAsyncResponder| {
            // dbg!(&request);
            let id: i32 = request
                .uri()
                .path()
                .trim_start_matches("/dbimage/")
                .parse()
                .unwrap();
            let db = db_signal().unwrap();

            spawn(async move {
                let data = piece_2_blob::Entity::load()
                    .with(blobs::Entity)
                    .filter_by_id(id)
                    .one(&db)
                    .await
                    .unwrap()
                    .unwrap();
                // dbg!(&data.mime_type);
                let response = http::Response::builder()
                    .header("Content-Type", data.mime_type)
                    .body(data.blob.unwrap().data)
                    .unwrap();
                responder.respond(response);
            });
        },
    );
}
