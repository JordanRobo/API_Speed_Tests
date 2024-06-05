// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::*;
use serde::{ Serialize, Deserialize };
use crate::schema::test;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = test)]
pub struct Test {
    pub id: i32,
    pub content: String,
}

