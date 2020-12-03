use crate::db;
use crate::error_handler::CustomError;
use crate::schema::roles;
use crate::users::User;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Identifiable, Queryable, AsChangeset, Insertable, Associations,
)]
#[belongs_to(User)]
#[table_name = "roles"]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub user_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "roles"]
pub struct MaybeRole {
    pub name: String,
    pub user_id: Option<i64>,
}

impl Role {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let roles = roles::table.load::<Role>(&conn)?;
        Ok(roles)
    }

    pub fn find_by_id(id: i64) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let role = roles::table.filter(roles::id.eq(id)).first(&conn)?;
        Ok(role)
    }

    pub fn find_by_name(name: String) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let role = roles::table.filter(roles::name.eq(name)).first(&conn)?;
        Ok(role)
    }

    pub fn find_by_user(id: i64) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let roles = roles::table
            .filter(roles::user_id.eq(id))
            .load::<Role>(&conn)?;
        Ok(roles)
    }

    pub fn create(role: MaybeRole) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let role = diesel::insert_into(roles::table)
            .values(role)
            .get_result(&conn)?;
        Ok(role)
    }

    pub fn update(id: i64, role: MaybeRole) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let role = diesel::update(roles::table)
            .filter(roles::id.eq(id))
            .set(role)
            .get_result(&conn)?;
        Ok(role)
    }

    pub fn delete(id: i64) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(roles::table.filter(roles::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
