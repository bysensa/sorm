/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sorm::*;

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[sorm(table = user)]
pub struct User<'a> {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub created: DateTime<Utc>,
    pub company: &'a str,
    pub tags: Vec<String>,
}

#[derive(Edge, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[sorm(table = like)]
pub struct Like<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,

    #[sorm(ty = "option<float>")]
    pub score: Option<f64>,

    #[sorm(ty = "array<array<float>>")]
    pub scores_plural: Vec<Vec<f64>>,

    #[serde(rename = "in")]
    #[sorm(link_many = In)]
    pub in_: LinkOne<In>,

    #[sorm(link_one = Out)]
    pub out: LinkOne<Out>,

    #[sorm(nest_object = Time)]
    pub time: Time,
}
pub type CompanyLikeUser<'a> = Like<Company<'a>, User<'a>>;

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[sorm(table = company)]
pub struct Company<'b> {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub namex: &'b str,

    #[sorm(link_many = "User<'b>")]
    #[serde(default)]
    pub users: LinkMany<User<'b>>,

    #[sorm(relate(model = "CompanyLikeUser<'b>", connection = "->like->user"))]
    #[serde(skip_serializing, default)]
    pub devs: Relate<User<'b>>,
}

#[derive(Object, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    // pub name: String,
    pub connected: DateTime<Utc>,
}

#[derive(Node, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[sorm(table = organization)]
pub struct Organization<'a> {
    pub id: SurrealSimpleId<Self>,
    pub name: String,

    #[sorm(link_many = "User<'a>")]
    pub users: LinkMany<User<'a>>,

    #[sorm(nest_object = Time)]
    pub time: Time,
    pub age: u8,
}
