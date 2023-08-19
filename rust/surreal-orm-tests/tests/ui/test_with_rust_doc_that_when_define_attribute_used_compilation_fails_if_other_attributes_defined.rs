use serde::{Deserialize, Serialize};
use surreal_orm::{
    statements::{for_, select, Permissions},
    *,
};

fn get_age_default_value() -> u8 {
    18
}

fn get_age_assertion() -> Filter {
    cond(value().is_not(NONE))
}

fn age_permissions() -> Permissions {
    for_([CrudType::Create, CrudType::Delete])
        .where_(StudentTest::schema().firstName.is("Oyelowo"))
        .into()
}

fn student_permissions() -> Permissions {
    for_([CrudType::Create, CrudType::Delete])
        .where_(StudentTest::schema().firstName.is("Oyelowo"))
        .into()
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_test",
    drop,
    // flexible,
    schemafull,
    as = "select(All).from(Student::table_name())",
    permissions = "student_permissions()",
    define = "define_student()"
)]
pub struct StudentTest {
    id: SurrealSimpleId<Self>,
    first_name: String,
    last_name: String,
    #[surreal_orm(
        type = "int",
        value = "18",
        assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        permissions = "for_([CrudType::Create, CrudType::Delete]).where_(StudentTest::schema().firstName.is(\"Oyelowo\"))",
        // define = "define_age()"
    )]
    age_inline_expr: u8,

    #[surreal_orm(
        type = "int",
        value = "get_age_default_value()",
        assert = "get_age_assertion()",
        permissions = "age_permissions()",
        // define = "define_age()"
    )]
    age_default_external_function_invoked_expr: u8,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student")]
pub struct Student {
    id: SurrealSimpleId<Self>,
}

fn main() {}
