use diesel::{
    prelude::*,
    backend::{RawValue},
    serialize::{self, Output, ToSql},
    deserialize::{self, FromSql, FromSqlRow},
    sql_types::{SmallInt},
    pg::Pg,
};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Copy, Clone, FromSqlRow)]
#[repr(i16)]
pub enum SubscriptionType {
    Free = 0,
    Cheapskate = 1,
    Starter = 2,
    Business = 3,
    Enterprise = 4,
}

impl ToSql<SmallInt, Pg> for SubscriptionType
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        <i16 as ToSql<SmallInt, Pg>>::to_sql(&(*self as i16), &mut out.reborrow())
    }
}

impl FromSql<SmallInt, Pg> for SubscriptionType
{
    fn from_sql(val: RawValue<Pg>) -> deserialize::Result<Self> {
        match i16::from_sql(val)? {
            0 => Ok(Self::Free),
            1 => Ok(Self::Cheapskate),
            2 => Ok(Self::Starter),
            3 => Ok(Self::Business),
            4 => Ok(Self::Enterprise),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable)]
pub struct Account {
    pub id: Uuid,
    pub subscription_type: SubscriptionType,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<i32>,
    pub is_active: bool,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<i32>,
    pub is_active: bool,
    pub account_id: Uuid,
}

#[derive(Queryable)]
pub struct ApiKey {
    pub id: i32,
    pub key: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<i32>,
    pub is_active: bool,
}