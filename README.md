[![surreal-orm](https://github.com/Oyelowo/surreal-orm/actions/workflows/rust.yaml/badge.svg)](https://github.com/Oyelowo/surreal-orm/actions/workflows/rust.yaml)

[![cleanup old images](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/delete-old-images.yaml/badge.svg)](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/delete-old-images.yaml)

# Quick Start

# Surreal ORM Documentation

## Introduction

Surreal ORM is an Object-Relational Mapping and query-building library for Rust
that provides a high-level API for interacting with SurrealDB, a distributed
graph database. This documentation will guide you through the usage and features
of the Surreal ORM library.

## Getting Started

To use Surreal ORM in your Rust project, you need to add it as a dependency in
your `Cargo.toml` file:

```toml
[dependencies]
surreal_orm = "0.1"
```

After adding the dependency, you can import the necessary modules in your Rust
code:

```rust
use surreal_orm::*;
```

## Connecting to SurrealDB

Before interacting with SurrealDB, you need to establish a connection to the
database. The following example demonstrates how to create a connection to a
local SurrealDB instance:

```rust
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[tokio::main]
async fn main() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
}
```

In this example, we create a new SurrealDB instance using the `Surreal::new`
function with the `local::Mem` engine. The `local::Mem` engine represents a
local in-memory database. You can replace it with other engine types according
to your setup.

## Defining a Model

A model in Surreal ORM represents a database table. You can define a model by
creating a Rust struct and implementing the `Node` or `Edge` trait. Here's an
example of defining a `SpaceShip` model:

```rust
use surreal_orm::*;

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub age: u8,
}
```

In this example, we define a `SpaceShip` struct and annotate it with the `Node`
derive macro. The `table_name` attribute specifies the name of the corresponding
database table.

## Querying Data

Surreal ORM provides a fluent and expressive API for querying data from the
database. You can use the `select` function to start a select statement and
chain various methods to build the query. Here's an example:

```rust
use surreal_orm::statements::{select, All};

let space_ship_schema::SpaceShip { name, age, .. } = SpaceShip::schema();

let statement = select(All)
    .from(space_ship)
    .where_(name.equal("Millennium Falcon"))
    .order_by(age.desc())
    .limit(10);
```

In this example, we start a select statement using the `select` function and
pass the `All` argument to select all fields. We specify the table name using
the `from` method and add a condition using the `where_` method. We can also use
the `order_by` method to specify the sorting order and the `limit` method to
limit the number of results.

## Inserting Data

To insert data into the database, you can use the `insert` function and provide
the data as a vector of structs. Here's an example:

```rust
use surreal_orm::statements::insert;

let spaceships = vec![
    SpaceShip {
        id: SpaceShip::create_simple_id(),
        name: "Millennium Falcon".to_string(),
        age: 79,
    },
    SpaceShip {
        id: SpaceShip::create_simple_id(),
        name: "Starship Enterprise".to_string(),
        age: 15,
    },
];

insert(spaceships).return_many(db.clone()).await?;
```

In this example, we define a vector of `SpaceShip` structs and pass it to the
`insert` function. We then call the `run` method to execute the insertion
operation.

## Updating Data

To update data in the database, you can use the `update` function and provide
the updated data as a struct. Here's an example:

```rust
use surreal_orm::statements::update;

let spaceship = SpaceShip {
    id: SpaceShip::create_simple_id(),
    name: "Millennium Falcon".to_string(),
    age: 60
};

update(spaceship).run(db.clone()).await?;
```

In this example, we define a `SpaceShip` struct with the updated data and pass
it to the `update` function. We then call the `run` method to execute the update
operation.

## Deleting Data

To delete data from the database, you can use the `delete` function and provide
the condition for deletion. Here's an example:

```rust
use surreal_orm::{*, statements::{delete}};

let space_ship_schema::SpaceShip { name, age, .. } = SpaceShip::schema();
let condition = name.eq("Millennium Falcon");

delete(space_ship)
    .where_(cond(name.equal("Millennium Falcon")).and(age.less_then(50)))
    .run(db.clone())
    .await?;
```

In this example, we use the `delete` function and specify the table name. We add
a condition using the `where_` method, and then call the `run` method to execute
the deletion operation.

## Transactions

Surreal ORM supports transactions, which are a series of operations that are
treated as a single unit of work. Here's an example of a transaction that
involves creating two accounts, updating their balances, and then committing the
transaction:

```rust
use surreal_orm::{
    statements::{begin_transaction, create, update},
    *,
};
use surreal_models::Account;
use surrealdb::{engine::local::Mem, Surreal};

let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

let id1 = &Account::create_id("one".into());
let id2 = &Account::create_id("two".into());
let amount_to_transfer = 300.00;

let acc = Account::schema();

block! {
    BEGIN TRANSACTION;

    LET balance = create().content(Balance {
            id: Balance::create_id("balance1".into()),
            amount: amount_to_transfer,
        });

    LET acc1 = create().content(Account {
        id: id1.clone(),
        balance: 135_605.16,
    });
    LET acc2 = create().content(Account {
        id: id2.clone(),
        balance: 91_031.31,
    });

    // You can reference the balance object by using the $balance variable and pass the amount
    // as a parameter to the decrement_by function. i.e $balance.amount
    LET updated1 = update::<Account>(id1).set(acc.balance.increment_by(balance.with_path::<Balance>(E).amount));
            
    // You can also pass the amount directly to the decrement_by function. i.e 300.00
    LET update2 = update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));

    COMMIT TRANSACTION;
}
.run(db.clone())
.await?;

let accounts = select(All)
    .from(id1..=id2)
    .return_many::<Account>(db.clone())
    .await?;
```

In this example, we begin a transaction and then create two accounts with
initial balances. We then increment the balance of the first account and
decrement the balance of the second account by the same amount. Finally, we
commit the transaction and then verify that the balances were updated correctly.

## Conclusion

This concludes the basic usage and features of the Surreal ORM library. You can
explore more advanced features and methods in the API documentation. If you have
any further questions or need assistance, feel free to reach out.

## Convention

To carry out certain tasks in any directory, these are the standard commands:

| Commands       | Purpose                                          |
| -------------- | :----------------------------------------------- |
| `make setup`   | To setup the codebase for development            |
| `make install` | install packages                                 |
| `make upgrade` | upgrade packages                                 |
| `make sync`    | synchronize/generate local code etc              |
| `make dev`     | start cluster/app locally in live reloading mode |
| `make format`  | format code                                      |
| `make check`   | check that code aligns with standard             |
| `make test`    | run automated tests                              |
