use crate::db;
use crate::error_handler::CustomError;
use crate::schema::alerts;
use crate::users::User;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, AsChangeset, Insertable, Associations)]
#[belongs_to(User)]
#[table_name = "alerts"]
pub struct Alert {
    pub id: i64,
    pub message: Option<String>,
    pub reason: String,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "alerts"]
pub struct MaybeAlert {
    pub message: Option<String>,
    pub reason: String,
    pub user_id: i64
}

impl Alert {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let alerts = alerts::table.load::<Alert>(&conn)?;
        Ok(alerts)
    }    

    pub fn find_by_id(id: i64) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let alert = alerts::table.filter(alerts::id.eq(id)).first(&conn)?;
        Ok(alert)
    }

    pub fn find_by_user(id:i64) -> Result<Vec<Self>, CustomError>{
        let conn = db::connection()?;
        let alerts = alerts::table.filter(alerts::user_id.eq(id)).load::<Alert>(&conn)?;
        Ok(alerts)
    }

    pub fn create(alert: MaybeAlert) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let alert = diesel::insert_into(alerts::table)
            .values(alert)
            .get_result(&conn)?;
        Ok(alert)
    }

    pub fn update(id: i64, alert: MaybeAlert) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let alert = diesel::update(alerts::table)
            .filter(alerts::id.eq(id))
            .set(alert)
            .get_result(&conn)?;
        Ok(alert)
    }

    pub fn delete(id: i64) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(alerts::table.filter(alerts::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}