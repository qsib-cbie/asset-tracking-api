use crate::db;
use crate::error_handler::CustomError;
use crate::schema::asset_tags;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, AsChangeset, Insertable)]
#[table_name = "asset_tags"]
pub struct AssetTag {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub serial_number: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "asset_tags"]
pub struct MaybeAssetTag {
    pub name: String,
    pub description: Option<String>,
    pub serial_number: String,
}

impl AssetTag {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let asset_tags = asset_tags::table.load::<AssetTag>(&conn)?;
        Ok(asset_tags)
    }

    pub fn find_by_name(name: String) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset_tag = asset_tags::table
            .filter(asset_tags::name.eq(name))
            .first(&conn)?;
        Ok(asset_tag)
    }

    pub fn find_by_id(id: i64) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset_tag = asset_tags::table
            .filter(asset_tags::id.eq(id))
            .first(&conn)?;
        Ok(asset_tag)
    }

    pub fn create(asset_tag: MaybeAssetTag) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset_tag = diesel::insert_into(asset_tags::table)
            .values(asset_tag)
            .get_result(&conn)?;
        Ok(asset_tag)
    }

    pub fn update(id: i64, asset_tag: MaybeAssetTag) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset_tag = diesel::update(asset_tags::table)
            .filter(asset_tags::id.eq(id))
            .set(asset_tag)
            .get_result(&conn)?;
        Ok(asset_tag)
    }

    pub fn delete(id: i64) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(asset_tags::table.filter(asset_tags::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
