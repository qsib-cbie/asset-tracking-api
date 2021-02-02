use crate::db;
use crate::error_handler::CustomError;
use crate::schema::assets;
use crate::asset_tags::AssetTag;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Identifiable, Queryable, AsChangeset, Insertable, Associations,
)]
#[belongs_to(AssetTag)]
#[table_name = "assets"]
pub struct Asset {
    pub id: i64,    
    pub asset_tag_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "assets"]
pub struct MaybeAsset {    
    pub asset_tag_id: Option<i64>,
}

impl Asset {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let assets = assets::table.load::<Asset>(&conn)?;
        Ok(assets)
    }

    pub fn find_by_id(id: i64) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset = assets::table.filter(assets::id.eq(id)).first(&conn)?;
        Ok(asset)
    }    

    pub fn find_by_asset_tag(id: i64) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let assets = assets::table
            .filter(assets::asset_tag_id.eq(id))
            .load::<Asset>(&conn)?;
        Ok(assets)
    }

    pub fn create(asset: MaybeAsset) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset = diesel::insert_into(assets::table)
            .values(asset)
            .get_result(&conn)?;
        Ok(asset)
    }

    pub fn update(id: i64, asset: MaybeAsset) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset = diesel::update(assets::table)
            .filter(assets::id.eq(id))
            .set(asset)
            .get_result(&conn)?;
        Ok(asset)
    }

    pub fn delete(id: i64) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(assets::table.filter(assets::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
