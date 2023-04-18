/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Crypto functions
// These functions can be used when hashing data, encrypting data, and for securely authenticating users into the database.
//
// Function	Description
// crypto::md5()	Returns the md5 hash of a value
// crypto::sha1()	Returns the sha1 hash of a value
// crypto::sha256()	Returns the sha256 hash of a value
// crypto::sha512()	Returns the sha512 hash of a value
// crypto::argon2::compare()	Compares an argon2 hash to a password
// crypto::argon2::generate()	Generates a new argon2 hashed password
// crypto::pbkdf2::compare()	Compares an pbkdf2 hash to a password
// crypto::pbkdf2::generate()	Generates a new pbkdf2 hashed password
// crypto::scrypt::compare()	Compares an scrypt hash to a password
// crypto::scrypt::generate()	Generates a new scrypt hashed password

use crate::{
    statements::let_, traits::Buildable, types::Function, Field, Param, Parametric, StrandLike,
    ToRaw,
};

pub(crate) fn create_fn_with_single_value(
    value: impl Into<StrandLike>,
    function_suffix: &str,
) -> Function {
    let value: StrandLike = value.into();

    Function {
        query_string: format!("crypto::{function_suffix}({})", value.build()),
        bindings: value.get_bindings(),
    }
}

pub(crate) fn create_fn_with_two_values(
    value1: impl Into<StrandLike>,
    value2: impl Into<StrandLike>,
    function_suffix: &str,
) -> Function {
    let value1: StrandLike = value1.into();
    let value2: StrandLike = value2.into();
    let mut bindings = vec![];
    bindings.extend(value1.get_bindings());
    bindings.extend(value2.get_bindings());

    Function {
        query_string: format!(
            "crypto::{function_suffix}({}, {})",
            value1.build(),
            value2.build()
        ),
        bindings,
    }
}

macro_rules! create_fn_with_single_arg_value {
    ($function_name: expr) => {
        paste::paste! {
            pub fn [<$function_name _fn>](value: impl Into<$crate::StrandLike>) -> $crate::Function {
                create_fn_with_single_value(value, $function_name)
            }

            #[macro_export]
            macro_rules! [<cryto_ $function_name>] {
                ( $value:expr ) => {
                    $crate::functions::crypto::[<$function_name _fn>]($value)
                };
            }
            pub use [<cryto_ $function_name>] as [<$function_name>];

            #[test]
            fn [<test_ $function_name>]() {
                let result = [<$function_name _fn>]("Oyelowo");
                assert_eq!(result.fine_tune_params(), format!("crypto::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().build(), format!("crypto::{}('Oyelowo')", $function_name));
            }

            #[test]
            fn [<test_ $function_name _with_macro>]() {
                let result = [<$function_name>]!("Oyelowo");
                assert_eq!(result.fine_tune_params(), format!("crypto::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().build(), format!("crypto::{}('Oyelowo')", $function_name));
            }

            #[test]
            fn [<test_ $function_name _with_field>]() {
                let title = $crate::Field::new("title");
                let result = [<$function_name _fn>](title);
                assert_eq!(result.fine_tune_params(), format!("crypto::{}(title)", $function_name));
                assert_eq!(result.to_raw().build(), format!("crypto::{}(title)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_with_field>]() {
                let title = $crate::Field::new("title");
                let result = [<$function_name>]!(title);
                assert_eq!(result.fine_tune_params(), format!("crypto::{}(title)", $function_name));
                assert_eq!(result.to_raw().build(), format!("crypto::{}(title)", $function_name));
            }
        }
    };
}
create_fn_with_single_arg_value!("md5");
create_fn_with_single_arg_value!("sha1");
create_fn_with_single_arg_value!("sha256");
create_fn_with_single_arg_value!("sha512");

pub mod argon2 {
    use surrealdb::sql;

    use crate::{types::Function, StrandLike};

    use super::{create_fn_with_single_value, create_fn_with_two_values};

    pub fn compare_fn(value1: impl Into<StrandLike>, value2: impl Into<StrandLike>) -> Function {
        create_fn_with_two_values(value1, value2, "argon2::compare")
    }
    #[macro_export]
    macro_rules! crypto_argon2_compare {
        ( $value1:expr,  $value2:expr ) => {
            $crate::functions::crypto::argon2::compare_fn($value1, $value2)
        };
    }
    pub use crypto_argon2_compare as compare;

    pub fn generate_fn(value: impl Into<StrandLike>) -> Function {
        create_fn_with_single_value(value, "argon2::generate")
    }
    #[macro_export]
    macro_rules! crypto_argon2_generate {
        ( $value1:expr) => {
            $crate::functions::crypto::argon2::generate_fn($value1)
        };
    }
    pub use crypto_argon2_generate as generate;
}

pub mod pbkdf2 {
    use super::{create_fn_with_single_value, create_fn_with_two_values};
    use crate::{types::Function, StrandLike};
    use surrealdb::sql;

    pub fn compare_fn(value1: impl Into<StrandLike>, value2: impl Into<StrandLike>) -> Function {
        create_fn_with_two_values(value1, value2, "pbkdf2::compare")
    }

    #[macro_export]
    macro_rules! crypto_pbkdf2_compare {
        ( $value1:expr,  $value2:expr ) => {
            $crate::functions::crypto::pbkdf2::compare_fn($value1, $value2)
        };
    }
    pub use crypto_pbkdf2_compare as compare;

    // Crypto generate function
    pub fn generate_fn(value: impl Into<StrandLike>) -> Function {
        create_fn_with_single_value(value, "pbkdf2::generate")
    }
    #[macro_export]
    macro_rules! crypto_pbkdf2_generate {
        ( $value1:expr) => {
            $crate::functions::crypto::pbkdf2::generate_fn($value1)
        };
    }
    pub use crypto_pbkdf2_generate as generate;
}

pub mod scrypt {
    use surrealdb::sql;

    use crate::{types::Function, StrandLike};

    use super::{create_fn_with_single_value, create_fn_with_two_values};

    pub fn compare_fn(value1: impl Into<StrandLike>, value2: impl Into<StrandLike>) -> Function {
        create_fn_with_two_values(value1, value2, "scrypt::compare")
    }
    #[macro_export]
    macro_rules! crypto_scrypt_compare {
        ( $value1:expr,  $value2:expr ) => {
            crate::functions::crypto::scrypt::compare_fn($value1, $value2)
        };
    }
    pub use crypto_scrypt_compare as compare;

    pub fn generate_fn(value: impl Into<StrandLike>) -> Function {
        create_fn_with_single_value(value, "scrypt::generate")
    }
    #[macro_export]
    macro_rules! crypto_scrypt_generate {
        ( $value1:expr) => {
            crate::functions::crypto::scrypt::generate_fn($value1)
        };
    }
    pub use crypto_scrypt_generate as generate;
}

#[test]
fn test_argon2_compare() {
    let hash = Param::new("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    let pass = Param::new("the strongest password");
    let result = argon2::compare_fn("Oyelowo", "Oyedayo");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::argon2::compare($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::argon2::compare('Oyelowo', 'Oyedayo')"
    );
}

#[test]
fn test_argon2_compare_with_param() {
    let hash = let_("hash").equal("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    let pass = let_("pass").equal("the strongest password");

    let result = argon2::compare_fn(hash.get_param(), pass.get_param());
    assert_eq!(
        result.fine_tune_params(),
        "crypto::argon2::compare($hash, $pass)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::argon2::compare($hash, $pass)"
    );
}

#[test]
fn test_argon2_generate() {
    let result = argon2::generate_fn("Oyelowo");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::argon2::generate($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::argon2::generate('Oyelowo')"
    );
}

#[test]
fn test_argon2_compare_macro_with_raw_values() {
    let result = argon2::compare!("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA", "the strongest password");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::argon2::compare($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::argon2::compare('$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA', 'the strongest password')"
    );
}

#[test]
fn test_argon2_compare_macro_with_fields() {
    let hash = Field::new("hash");
    let pass = Field::new("pass");

    let result = argon2::compare!(hash, pass);
    assert_eq!(
        result.fine_tune_params(),
        "crypto::argon2::compare(hash, pass)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::argon2::compare(hash, pass)"
    );
}

#[test]
fn test_argon2_compare_macro_with_param() {
    let hash = let_("hash").equal("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    let pass = let_("pass").equal("the strongest password");

    let result = argon2::compare!(hash.get_param(), pass.get_param());
    assert_eq!(
        result.fine_tune_params(),
        "crypto::argon2::compare($hash, $pass)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::argon2::compare($hash, $pass)"
    );
}

#[test]
fn test_argon2_generate_fn() {
    let result = argon2::generate!("Oyelowo");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::argon2::generate($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::argon2::generate('Oyelowo')"
    );
}

// pbkdf2
#[test]
fn test_pbkdf2_compare_with_param() {
    let hash = let_("hash").equal("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    let pass = let_("pass").equal("the strongest password");

    let result = pbkdf2::compare_fn(hash.get_param(), pass.get_param());
    assert_eq!(
        result.fine_tune_params(),
        "crypto::pbkdf2::compare($hash, $pass)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::pbkdf2::compare($hash, $pass)"
    );
}

#[test]
fn test_pbkdf2_generate() {
    let result = pbkdf2::generate_fn("Oyelowo");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::pbkdf2::generate($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::pbkdf2::generate('Oyelowo')"
    );
}

#[test]
fn test_pbkdf2_compare_macro_with_raw_values() {
    let result = pbkdf2::compare!("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA", "the strongest password");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::pbkdf2::compare($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::pbkdf2::compare('$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA', 'the strongest password')"
    );
}

#[test]
fn test_pbkdf2_compare_macro_with_fields() {
    let hash = Field::new("hash");
    let pass = Field::new("pass");

    let result = pbkdf2::compare!(hash, pass);
    assert_eq!(
        result.fine_tune_params(),
        "crypto::pbkdf2::compare(hash, pass)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::pbkdf2::compare(hash, pass)"
    );
}

#[test]
fn test_pbkdf2_compare_macro_with_param() {
    let hash = let_("hash").equal("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    let pass = let_("pass").equal("the strongest password");

    let result = pbkdf2::compare!(hash.get_param(), pass.get_param());
    assert_eq!(
        result.fine_tune_params(),
        "crypto::pbkdf2::compare($hash, $pass)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::pbkdf2::compare($hash, $pass)"
    );
}

#[test]
fn test_pbkdf2_generate_fn() {
    let result = pbkdf2::generate!("Oyelowo");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::pbkdf2::generate($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::pbkdf2::generate('Oyelowo')"
    );
}

#[test]
fn test_scrypt_compare_with_param() {
    let hash = let_("hash").equal("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    let pass = let_("pass").equal("the strongest password");

    let result = scrypt::compare_fn(hash.get_param(), pass.get_param());
    assert_eq!(
        result.fine_tune_params(),
        "crypto::scrypt::compare($hash, $pass)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::scrypt::compare($hash, $pass)"
    );
}

#[test]
fn test_scrypt_generate() {
    let result = scrypt::generate_fn("Oyelowo");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::scrypt::generate($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::scrypt::generate('Oyelowo')"
    );
}

#[test]
fn test_scrypt_compare_macro_with_raw_values() {
    let result = scrypt::compare!("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA", "the strongest password");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::scrypt::compare($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::scrypt::compare('$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA', 'the strongest password')"
    );
}

#[test]
fn test_scrypt_compare_macro_with_fields() {
    let hash = Field::new("hash");
    let pass = Field::new("pass");

    let result = scrypt::compare!(hash, pass);
    assert_eq!(
        result.fine_tune_params(),
        "crypto::scrypt::compare(hash, pass)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::scrypt::compare(hash, pass)"
    );
}

#[test]
fn test_scrypt_compare_macro_with_param() {
    let hash = let_("hash").equal("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    let pass = let_("pass").equal("the strongest password");

    let result = scrypt::compare!(hash.get_param(), pass.get_param());
    assert_eq!(
        result.fine_tune_params(),
        "crypto::scrypt::compare($hash, $pass)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::scrypt::compare($hash, $pass)"
    );
}

#[test]
fn test_scrypt_generate_fn() {
    let result = scrypt::generate!("Oyelowo");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::scrypt::generate($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().build(),
        "crypto::scrypt::generate('Oyelowo')"
    );
}
