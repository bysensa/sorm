/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// HTTP functions
// These functions can be used when opening and submitting remote web requests, and webhooks.
//
// Function	Description
// http::head()	Perform a remote HTTP HEAD request
// http::get()	Perform a remote HTTP GET request
// http::put()	Perform a remote HTTP PUT request
// http::post()	Perform a remote HTTP POST request
// http::patch()	Perform a remote HTTP PATCH request
// http::delete()	Perform a remote HTTP DELETE request

use std::collections::{hash_map, HashMap};

use surrealdb::sql;

use crate::{
    traits::{Binding, Buildable, ToRaw},
    types::{Field, Function, ObjectLike, Param, StrandLike},
    Parametric,
};

pub type Url = StrandLike;

fn create_fn_with_two_args(
    url: impl Into<Url>,
    custom_headers: Option<impl Into<ObjectLike>>,
    method: &str,
) -> Function {
    let url: Url = url.into();
    let mut all_bindings = url.get_bindings();

    let string = match custom_headers {
        None => {
            format!("http::{method}({})", &url.build())
        }
        Some(headers) => {
            let headers: ObjectLike = headers.into();
            all_bindings.extend(headers.get_bindings());

            format!("http::{method}({}, {})", url.build(), headers.build())
        }
    };

    Function {
        query_string: string,
        bindings: all_bindings,
    }
}

fn create_fn_with_three_args(
    url: impl Into<Url>,
    request_body: Option<impl Into<ObjectLike>>,
    custom_headers: Option<impl Into<ObjectLike>>,
    method: &str,
) -> Function {
    let url: Url = url.into();
    let mut all_bindings = url.get_bindings();

    let string = match request_body {
        None => {
            format!("http::{method}({}, {{ }}", &url.build())
        }
        Some(body) => {
            let body: ObjectLike = body.into();
            all_bindings.extend(body.get_bindings());

            format!("http::{method}({}, {}", url.build(), body.build())
        }
    };

    let string = match custom_headers {
        None => {
            format!("{string}, {{ }})")
        }
        Some(headers) => {
            let headers: ObjectLike = headers.into();
            all_bindings.extend(headers.get_bindings());

            format!("{string}, {})", headers.build())
        }
    };

    Function {
        query_string: string,
        bindings: all_bindings,
    }
}

macro_rules! create_fn_with_url_and_head {
    ($function_name: expr) => {
        paste::paste! {
            pub fn [<$function_name _fn>](url: impl Into<Url>, custom_headers: Option<impl Into<ObjectLike>>) -> Function {
               create_fn_with_two_args(url, custom_headers, $function_name)
           }

           #[macro_export]
           macro_rules! [<http_ $function_name>] {
               ( $url:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, None as Option<ObjectLike>)
               };
               ( $url:expr, $custom_headers:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, Some($custom_headers))
               };
           }
           pub use [<http_ $function_name>] as [<$function_name>];


            #[test]
            fn [<test_ $function_name _method_with_empty_header>]() {
                let result = [<$function_name _fn>]("https://codebreather.com", None as Option<ObjectLike>);
                assert_eq!(result.fine_tune_params(), format!("http::{}($_param_00000001)", $function_name));
                assert_eq!(
                    result.to_raw().build(),
                    format!("http::{}('https://codebreather.com')", $function_name)
                );
            }

            #[test]
            fn [<test_ $function_name _method_with_field_and_empty_header>]() {
                let homepage = Field::new("homepage");

                let result = [<$function_name _fn>](homepage, None as Option<ObjectLike>);
                assert_eq!(result.fine_tune_params(), format!("http::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().build(), format!("http::{}(homepage)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _method_with_plain_custom_header>]() {
                let headers = hash_map::HashMap::from([("x-my-header".into(), "some unique string".into())]);
                let result = [<$function_name _fn>]("https://codebreather.com", Some(headers));
                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002)", $function_name)
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!("http::{}('https://codebreather.com', {})", $function_name, "{ \"x-my-header\": 'some unique string' }")
                );
            }

            #[test]
            fn [<test_ $function_name _method_with_field_custom_header>]() {
                let homepage = Field::new("homepage");
                let headers = Field::new("headers");

                let result = [<$function_name _fn>](homepage, Some(headers));
                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002)", $function_name)
                );
                assert_eq!(result.to_raw().build(), format!("http::{}(homepage, headers)", $function_name));
            }

            // Macro version
            #[test]
            fn [<test_ $function_name _macro_method_with_empty_header_not_listed>]() {
                let result = [<$function_name>]!("https://codebreather.com");
                assert_eq!(result.fine_tune_params(), format!("http::{}($_param_00000001)", $function_name));
                assert_eq!(
                    result.to_raw().build(),
                    format!("http::{}('https://codebreather.com')", $function_name)
                );
            }

            #[test]
            fn [<test_ $function_name _macro_method_with_empty_header>]() {
                let homepage = Field::new("homepage");

                let result = [<$function_name>]!(homepage);
                assert_eq!(result.fine_tune_params(), format!("http::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().build(), format!("http::{}(homepage)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_method_with_plain_custom_header>]() {
                let headers = hash_map::HashMap::from([("x-my-header".into(), "some unique string".into())]);
                let result = [<$function_name>]!("https://codebreather.com", headers);
                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002)", $function_name)
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!("http::{}('https://codebreather.com', {})", $function_name, "{ \"x-my-header\": 'some unique string' }")
                );
            }

            #[test]
            fn [<test_ $function_name _macro_method_with_field_custom_header>]() {
                let homepage = Field::new("homepage");
                let headers = Field::new("headers");

                let result = [<$function_name>]!(homepage, headers);
                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002)", $function_name)
                );
                assert_eq!(result.to_raw().build(), format!("http::{}(homepage, headers)", $function_name));
            }

        }
    };
}

create_fn_with_url_and_head!("head");
create_fn_with_url_and_head!("get");
create_fn_with_url_and_head!("delete");

macro_rules! create_fn_with_3args_url_body_and_head {
    ($function_name: expr) => {
        paste::paste! {
            pub fn [<$function_name _fn>](
                url: impl Into<Url>,
                request_body: Option<impl Into<ObjectLike>>,
                custom_headers: Option<impl Into<ObjectLike>>,
            ) -> Function {
               create_fn_with_three_args(url, request_body, custom_headers, $function_name)
           }

           #[macro_export]
           macro_rules! [<http_ $function_name>] {
               ( $url:expr, $request_body:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, Some($request_body), None as Option<ObjectLike>)
               };
               ( $url:expr, $request_body:expr, $custom_headers:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, Some($request_body) ,Some($custom_headers))
               };
           }
           pub use [<http_ $function_name>] as [<$function_name>];


            #[test]
            fn [<test_field_ $function_name _method_with_empty_body_and_headers>]() {
                let homepage = Field::new("homepage");
                let result = [<$function_name _fn>]("https://codebreather.com", None as Option<ObjectLike>, None as Option<ObjectLike>);

                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!("http::{}('https://codebreather.com', {}, {})", $function_name, "{  }", "{  }")
                );
            }

            #[test]
            fn [<test_field_ $function_name _method_with_fields_as_args>]() {
                let homepage = Field::new("homepage");
                let request_body = Field::new("request_body");
                let headers = Field::new("headers");

                let result = [<$function_name _fn>](homepage, Some(request_body), Some(headers));
                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!("http::{}(homepage, request_body, headers)", $function_name)
                );
            }

            #[test]
            fn [<test_field_ $function_name _method_with_params_as_args>]() {
                let homepage = Param::new("homepage");
                let request_body = Param::new("request_body");
                let headers = Param::new("headers");

                let result = [<$function_name _fn>](homepage, Some(request_body), Some(headers));
                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!("http::{}($homepage, $request_body, $headers)", $function_name)
                );
            }

            #[test]
            fn [<test_ $function_name _method_with_body_and_custom_headers_as_plain_values>]() {
                let body = HashMap::from([
                    ("id".into(), 1.into()),
                    ("body".into(), "This is some awesome thinking!".into()),
                    ("postId".into(), 100.into()),
                ]);
                let headers = HashMap::from([("x-my-header".into(), "some unique string".into())]);
                let result = [<$function_name _fn>]("https://codebreather.com", Some(body), Some(headers));

                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!(
                        "http::{}('https://codebreather.com', {}, {})",
                        $function_name,
                        "{ body: 'This is some awesome thinking!', id: 1, postId: 100 }",  "{ \"x-my-header\": 'some unique string' }" )
                );
            }

            // Macro versions
            #[test]
            fn [<test_field_ $function_name _macro_method_with_no_custom_headers>]() {
                let homepage = Field::new("homepage");
                let body = Field::new("body");
                let result = [<$function_name>]!("https://codebreather.com", body);

                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!("http::{}('https://codebreather.com', body, {})", $function_name, "{  }")
                );
            }
            #[test]
            fn [<test_field_ $function_name _macro_method_with_empty_body_and_headers>]() {
                let homepage = Field::new("homepage");
                let result = [<$function_name>]!("https://codebreather.com", None, None);

                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!("http::{}('https://codebreather.com', {}, {})", $function_name, "{  }", "{  }")
                );
            }

            #[test]
            fn [<test_field_ $function_name _macro_method_with_fields_as_args>]() {
                let homepage = Field::new("homepage");
                let request_body = Field::new("request_body");
                let headers = Field::new("headers");

                let result = [<$function_name>]!(homepage, request_body, headers);
                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!("http::{}(homepage, request_body, headers)", $function_name)
                );
            }

            #[test]
            fn [<test_field_ $function_name _macro_method_with_params_as_args>]() {
                let homepage = Param::new("homepage");
                let request_body = Param::new("request_body");
                let headers = Param::new("headers");

                let result = [<$function_name>]!(homepage, request_body, headers);
                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!("http::{}($homepage, $request_body, $headers)", $function_name)
                );
            }

            #[test]
            fn [<test_ $function_name _macro_method_with_body_and_custom_headers_as_plain_values>]() {
                let body = HashMap::from([
                    ("id".into(), 1.into()),
                    ("body".into(), "This is some awesome thinking!".into()),
                    ("postId".into(), 100.into()),
                ]);
                let headers = HashMap::from([("x-my-header".into(), "some unique string".into())]);
                let result = [<$function_name>]!("https://codebreather.com", body, headers);

                assert_eq!(
                    result.fine_tune_params(),
                    format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                );
                assert_eq!(
                    result.to_raw().build(),
                    format!(
                        "http::{}('https://codebreather.com', {}, {})",
                        $function_name,
                        "{ body: 'This is some awesome thinking!', id: 1, postId: 100 }",  "{ \"x-my-header\": 'some unique string' }" )
                );
            }
        }
    };
}
create_fn_with_3args_url_body_and_head!("post");
create_fn_with_3args_url_body_and_head!("put");
create_fn_with_3args_url_body_and_head!("patch");
