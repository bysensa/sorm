/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

mod migration;
use migration::generate_embedded_migrations;
use proc_macro::TokenStream;

/// embed_migrations!() is a macro that embeds migrations in the binary at compile time.
/// It takes 2 arguments:
/// 1. The path to the migrations directory
/// 3. The migration mode
/// The path to the migrations directory is optional. If not provided, it defaults to 'migrations'.
/// The migration mode is optional. If not provided, it defaults to Mode::Strict.
///
/// # Example
/// ```rust, ignore
/// use surreal_orm::sorm-migrator::{self, embed_migrations};
/// use surreal_orm::sorm-migrator::{FileManager, MigrationFlag, MigratorDatabase, Mode};
///
/// // Embed migrations as constant
/// const MIGRATIONS_ONE_WAY: sorm-migrator::EmbeddedMigrationsOneWay = embed_migrations!();
/// const MIGRATIONS_ONE_WAY: sorm-migrator::EmbeddedMigrationsOneWay = embed_migrations!("migrations");
/// const MIGRATIONS_ONE_WAY: sorm-migrator::EmbeddedMigrationsOneWay = embed_migrations!("migrations", strict);
///
/// const MIGRATIONS_TWO_WAY: sorm-migrator::EmbeddedMigrationsTwoWay = embed_migrations!();
/// const MIGRATIONS_TWO_WAY: sorm-migrator::EmbeddedMigrationsTwoWay = embed_migrations!("migrations");
/// const MIGRATIONS_TWO_WAY: sorm-migrator::EmbeddedMigrationsTwoWay = embed_migrations!("migrations", strict);
/// ```
#[proc_macro]
pub fn embed_migrations(input: TokenStream) -> TokenStream {
    generate_embedded_migrations(input)
}
