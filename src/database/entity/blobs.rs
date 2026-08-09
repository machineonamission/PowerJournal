use sea_orm::entity::prelude::*;


// piece_2_blob — heavy, never touched by the loader
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blobs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    #[sea_orm(column_type = "Blob")]
    pub data: Vec<u8>,
    #[sea_orm(belongs_to, from = "id", to = "id", on_update = "NoAction", on_delete = "Cascade")]
    pub meta: HasOne<super::piece_2_blob::Entity>,
}
impl ActiveModelBehavior for ActiveModel {}
