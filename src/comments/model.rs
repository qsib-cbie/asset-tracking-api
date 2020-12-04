use crate::asset_tags::AssetTag;
use crate::db;
use crate::error_handler::CustomError;
use crate::schema::comments;
use crate::users::User;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Identifiable, Queryable, AsChangeset, Insertable, Associations,
)]
#[belongs_to(User)]
#[belongs_to(AssetTag)]
#[table_name = "comments"]
pub struct Comment {
    pub id: i64,
    pub content: String,
    pub user_id: i64,
    pub asset_tag_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "comments"]
pub struct MaybeComment {
    pub content: String,
    pub user_id: i64,
    pub asset_tag_id: i64,
}

impl Comment {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let comments = comments::table.load::<Comment>(&conn)?;
        Ok(comments)
    }

    pub fn find_by_id(id: i64) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let comment = comments::table.filter(comments::id.eq(id)).first(&conn)?;
        Ok(comment)
    }

    pub fn find_by_user(id: i64) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let comments = comments::table
            .filter(comments::user_id.eq(id))
            .load::<Comment>(&conn)?;
        Ok(comments)
    }

    pub fn find_by_asset_tag(id: i64) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let comments = comments::table
            .filter(comments::asset_tag_id.eq(id))
            .load::<Comment>(&conn)?;
        Ok(comments)
    }

    pub fn create(comment: MaybeComment) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let comment = diesel::insert_into(comments::table)
            .values(comment)
            .get_result(&conn)?;
        Ok(comment)
    }

    pub fn update(id: i64, comment: MaybeComment) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let comment = diesel::update(comments::table)
            .filter(comments::id.eq(id))
            .set(comment)
            .get_result(&conn)?;
        Ok(comment)
    }

    pub fn delete(id: i64) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(comments::table.filter(comments::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
