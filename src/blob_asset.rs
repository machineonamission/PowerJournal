use crate::database::entity::prelude::*;
use dioxus::desktop::{use_asset_handler, AssetRequest, RequestAsyncResponder};
use dioxus::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait};

pub fn register_blob_asset(db_signal: Resource<DatabaseConnection>) {
    use_asset_handler("dbimage", move |request: AssetRequest, responder:RequestAsyncResponder| {
        // dbg!(&request);
        let id: i32 = request
            .uri()
            .path()
            .trim_start_matches("/dbimage/")
            .parse()
            .unwrap();
        let db = db_signal().unwrap();

        spawn(async move {
            let data = piece_2_blob::Entity::find_by_id(id)
                .one(&db)
                .await
                //TODO: more graceful error handling
                .unwrap()
                .unwrap();
            let response = http::Response::builder()
                .header("Content-Type", "image/heic")
                .body(data.data)
                .unwrap();
            responder.respond(response);
        });
    });
}
