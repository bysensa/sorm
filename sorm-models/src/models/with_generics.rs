use serde::{Deserialize, Serialize};
use sorm::*;
use std::collections::HashSet;

#[derive(Node, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[sorm(table = planet_with_generics)]
pub struct PlanetWithGenerics<'a, T: Serialize + Default + Clone + sorm::validators::Int> {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[sorm(ty = "float")]
    pub strength: Strength,

    #[sorm(ty = int)]
    pub something: T,

    #[sorm(nest_object = "RocketWithGenerics<'a, T>")]
    pub rocket: RocketWithGenerics<'a, T>,

    #[sorm(ty = "option<array<float>>")]
    pub score: Option<Vec<f64>>,
}
type Strength = f64;

#[derive(Object, Serialize, Deserialize, Debug, Clone, Default)]
// #[serde(rename_all = "camelCase")]
pub struct RocketWithGenerics<'a, T: Serialize + Default + Clone + sorm::validators::Int> {
    name: String,
    #[sorm(ty = "int")]
    something: T,

    #[sorm(ty = "option<string>")]
    something2: Option<&'a str>,

    nana: &'static str,
    #[serde(rename = "lowo")]
    fav_number: Option<i32>,
    #[sorm(ty = "set<int>")]
    field_set: HashSet<i32>,

    // TODO: Do a compile check for the array size against the declared field type
    #[sorm(ty = "array<float, 2>")]
    must_number: [Strength; 3],
}

fn _test_partial_ob() {
    let rocket = RocketWithGenerics::partial_builder()
        // .name("Sugar".to_string())
        .something(43)
        .fav_number(None)
        // .something2(None)
        // .fav_number(Some(1919))
        // .must_number(1919)
        .nana("ewe")
        .build();

    let _x = PlanetWithGenerics::partial_builder().rocket(rocket).build();
}
//
// type Lala<'a, T> = <Weapon<'a, T> as PartialUpdater>::StructPartial;
// fn xfd(arg1: String) -> DefineFieldStatement {
//     let x = &mut Weapon::partial_builder()
//         .name("Oyelowo".into())
//         .something(45)
//         // .strength(2.0)
//         .rocket(Rocket::partial_builder().something2(None).build())
//         .build();
//     // Weapon::pa
//     define_field("strength").permissions_full()
// }

// #[sorm-derive(Deserialize)]
// struct User<'a> {
//     id: u32,
//     name: &'a str,
//     screen_name: &'a str,
//     location: &'a str,
// }
//
//
//
//
