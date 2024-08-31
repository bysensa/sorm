/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use sorm::migrator::Migrator;
use sorm_models::migrations::Resources;

#[tokio::main]
async fn main() {
    Migrator::run(Resources).await;
}
