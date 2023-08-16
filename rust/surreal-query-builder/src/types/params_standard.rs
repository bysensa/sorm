/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

/// Macro to generate a SurrealDB native parameter structure and associated function.
///
/// SurrealDB employs a set of predefined variables. While these variables can be utilized
/// within your queries, it's important to note that you cannot declare new parameters
/// with any of the names generated by this macro.
///
/// # Examples
///
/// ```
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::*;
/// create_param_name_fn!(
///     /// $auth: Represents the currently authenticated scope user
///     => auth
/// );
/// ```
///
/// This invocation will produce a structure `ParamAuth` and a function `auth()` for use in
/// SurrealDB queries.
#[macro_export]
macro_rules! create_param_name_fn {
    ($(#[$attr:meta])* => $value:expr) => {
        $crate::internal_tools::paste! {
            #[allow(non_camel_case_types)]
            $(#[$attr])*
            pub struct [< Param $value >]($crate::Param);

            impl ::std::ops::Deref for [< Param $value >] {
                type Target = $crate::Param;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }


            $(#[$attr])*
            pub fn $value() -> [< Param $value >] {
                [< Param $value >]($crate::Param::new(stringify!($value)))
            }
        }
    };
    ($value:expr) => {
        $crate::internal_tools::paste! {
            #[allow(non_camel_case_types)]
            pub struct [< Param $value >]($crate::Param);

            impl ::std::ops::Deref for [< Param $value >] {
                type Target = $crate::Param;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }


            pub fn $value() -> [< Param $value >] {
                [< Param $value >]($crate::Param::new(stringify!($value)))
            }
        }
    };
}
// SurrealDB employs a set of predefined variables.
// While these variables can be utilized within your queries,
// it's important to note that you cannot declare new parameters with any of the names listed below:

create_param_name_fn!(
    /// $auth: Represents the currently authenticated scope user
    => auth
);

create_param_name_fn!(
    /// $token: Represents values held inside the JWT token used for the current session
    => token
);

create_param_name_fn!(
    /// $session: Represents values from the session functions as an object
    => session
);

create_param_name_fn!(
    /// $before: Represents the value before a mutation on a field
    => before
);

create_param_name_fn!(
    /// $after: Represents the value after a mutation on a field
    => after
);

create_param_name_fn!(
    /// $value: Represents the value after a mutation on a field (identical to $after in the case of an event)
    => value
);

create_param_name_fn!(
    /// $input: Represents the initially inputted value in a field definition, as the value clause could have modified the $value variable
    => input
);

create_param_name_fn!(
    /// $parent: Represents the parent record in a subquery
    => parent
);

create_param_name_fn!(
    /// $event: Represents the type of table event triggered on an event
    => event
);

create_param_name_fn!(
    /// $this: Represents the current record in a query
    => this
);
