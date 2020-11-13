use crate::db;
use crate::error_handler::CustomError;
use crate::schema::asset_scanners;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, AsChangeset, Insertable)]
#[table_name = "asset_scanners"]
pub struct AssetScanner {
    pub id: i64,
    pub name: String,    
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "asset_scanners"]
pub struct MaybeAssetScanner {
    pub name: String    
}

impl AssetScanner {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let asset_scanners = asset_scanners::table.load::<AssetScanner>(&conn)?;
        Ok(asset_scanners)
    }

    pub fn find_by_name(name: String) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset_scanner = asset_scanners::table.filter(asset_scanners::name.eq(name)).first(&conn)?;
        Ok(asset_scanner)
    }

    pub fn find_by_id(id: i64) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset_scanner = asset_scanners::table.filter(asset_scanners::id.eq(id)).first(&conn)?;
        Ok(asset_scanner)
    }

    pub fn create(asset_scanner: MaybeAssetScanner) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset_scanner = diesel::insert_into(asset_scanners::table)
            .values(asset_scanner)
            .get_result(&conn)?;
        Ok(asset_scanner)
    }

    pub fn update(id: i64, asset_scanner: MaybeAssetScanner) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let asset_scanner = diesel::update(asset_scanners::table)
            .filter(asset_scanners::id.eq(id))
            .set(asset_scanner)
            .get_result(&conn)?;
        Ok(asset_scanner)
    }

    pub fn delete(id: i64) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(asset_scanners::table.filter(asset_scanners::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}