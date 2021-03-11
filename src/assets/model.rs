use crate::asset_tags::AssetTag;
use crate::db;
use crate::error_handler::CustomError;
use crate::schema::assets;
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
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "assets"]
pub struct MaybeAsset {
    pub asset_tag_id: Option<i64>,
    pub deleted: bool,
}

impl Asset {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let assets = assets::table
            .filter(assets::deleted.eq(false))
            .load::<Asset>(&conn)?;
        Ok(assets)
    }

    pub fn find_with_deleted() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let assets = assets::table.load::<Asset>(&conn)?;
        Ok(assets)
    }

    pub fn find_deleted() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let assets = assets::table
            .filter(assets::deleted.eq(true))
            .load::<Asset>(&conn)?;
        Ok(assets)
    }

    pub fn find_by_id(id: i64) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset = assets::table
            .filter(assets::id.eq(id))
            .filter(assets::deleted.eq(false))
            .first(&conn)?;
        Ok(asset)
    }

    pub fn find_by_asset_tag(id: i64) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let assets = assets::table
            .filter(assets::asset_tag_id.eq(id))
            .filter(assets::deleted.eq(false))
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
            .filter(assets::deleted.eq(false))
            .set(asset)
            .get_result(&conn)?;
        Ok(asset)
    }

    pub fn delete(id: i64) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset = diesel::update(assets::table)
            .filter(assets::id.eq(id))
            .filter(assets::deleted.eq(false))
            .set(assets::deleted.eq(true))
            .get_result(&conn)?;
        Ok(asset)
    }
}
