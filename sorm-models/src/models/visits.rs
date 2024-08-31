/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sorm::*;

use crate::{Alien, SpaceShip, Weapon};

use super::planet::Planet;

// Visits

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanet = Visits<Alien, Planet<u64>>;

// VisitsExplicit
#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[sorm(table = visits_explicit)]
pub struct VisitsExplicit<In: Node, Out: Node> {
    #[sorm(ty = "record<visits_explicit>")]
    pub id: SurrealSimpleId<Self>,

    #[serde(rename = "in")]
    #[sorm(link_one = "In", ty = "record<any>")]
    pub in_: LinkOne<In>,

    #[sorm(link_one = "Out", ty = "record<any>")]
    pub out: LinkOne<Out>,
    #[sorm(ty = "duration")]
    pub time_visited: Duration,
}

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanetExplicit = VisitsExplicit<Alien, Planet<u64>>;

#[derive(Edge, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[sorm(table = visits_with_explicit_attributes)]
pub struct VisitsWithExplicitAttributes<In: Node, Out: Node> {
    // #[sorm(ty = "record<visits_with_explicit_attributes>")]
    pub id: SurrealSimpleId<Self>,

    #[serde(rename = "in")]
    #[sorm(link_one = In)]
    pub in_: LinkOne<In>,

    #[sorm(link_one = Out)]
    // #[sorm(ty = "record")]
    pub out: LinkOne<Out>,

    // #[sorm(ty = "string")]
    name: String,

    // #[sorm(ty = "int")]
    age: u8,

    // #[sorm(ty = "datetime")]
    created: DateTime<Utc>,

    // #[sorm(ty = "duration")]
    life_expectancy: Duration,

    // #[sorm(ty = "geometry<polygon>")]
    territory_area: geo::Polygon,

    // #[sorm(ty = "geometry<point>")]
    home: geo::Point,

    // #[sorm(ty = "array<string>")]
    tags: Vec<String>,

    // #[sorm(link_one = "Weapon", ty = "record<weapon>")]
    #[sorm(link_one = Weapon)]
    weapon: LinkOne<Weapon>,

    // Again, we dont have to provide the type attribute, it can auto detect
    // #[sorm(link_many = "SpaceShip", ty = "array<record<space_ship>>")]
    #[sorm(link_many = "SpaceShip")]
    space_ships: LinkMany<SpaceShip>,
}

pub type AlienVisitsPlanetWithExplicitAttributes = VisitsWithExplicitAttributes<Alien, Planet<u64>>;

#[derive(Edge, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[sorm(table = visits)]
pub struct Visits<In: Node, Out>
where
    Out: Node,
{
    pub id: SurrealSimpleId<Self>,
    #[sorm(link_one = In)]
    pub r#in: LinkOne<In>,

    #[sorm(link_one = Out)]
    pub out: LinkOne<Out>,
    pub hair_color: Option<String>,

    #[sorm(ty = "duration")]
    pub time_visited: Duration,

    #[sorm(link_one = "Planet<u64>")]
    pub mana: LinkOne<Planet<u64>>,
    pub name: String,
    pub age: u8,
    pub created: DateTime<Utc>,
    pub life_expectancy: Duration,
    pub line_string: geo::LineString,
    pub multi_line_string: geo::MultiLineString,
    pub polygon: geo::Polygon,
    pub multi_polygon: geo::MultiPolygon,
    pub point: geo::Point,
    pub multi_point: geo::MultiPoint,
    pub geometry_collection: sql::Geometry,
    pub territory_area: geo::Polygon,
    pub home: geo::Point,

    #[sorm(ty = "geometry<point>")]
    pub point_explicit: geo::Point,

    #[sorm(ty = "geometry<multipoint>")]
    pub multi_point_explicit: geo::MultiPoint,

    #[sorm(ty = "geometry<LineString>")]
    pub line_string_explicit: geo::LineString,

    #[sorm(ty = "geometry<multiline>")]
    pub multi_line_string_explicit: geo::MultiLineString,

    #[sorm(ty = "geometry<polygon>")]
    pub polygon_explicit: geo::Polygon,

    #[sorm(ty = "geometry<multipolygon>")]
    pub multi_polygon_explicit: geo::MultiPolygon,

    #[sorm(ty = "geometry<feature>")]
    pub geometry_collection_explicit: sql::Geometry,

    pub tags: Vec<String>,
    #[sorm(link_one = "Weapon")]
    pub weapon: LinkOne<Weapon>,
    // Again, we dont have to provide the type attribute, it can auto detect
    #[sorm(link_many = "SpaceShip")]
    pub space_ships: LinkMany<SpaceShip>,
    //
    // TODO:: Prevent doing this at compile time
    // This is a read only field. This wouldnt make sense. as we are using in node also as edge.
    // e.g visit->visit->plant
    // #[sorm(relate(model = "VistVisitsPlanet", connection = "->visits->planet"))]
    // #[serde(skip_serializing, default)]
    // pub visit_to_planet: Relate<Planet<u64>>,
}
