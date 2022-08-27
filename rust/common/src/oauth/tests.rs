use super::{
    api::{Config, Provider},
    cache_storage::{CacheStorage, HashMapCache, RedisCache},
};

#[cfg(test)]
mod tests {
    use async_std;
    use httpmock::prelude::*;
    use std::collections::HashMap;

    use oauth2::http::Uri;
    use pretty_assertions::{assert_str_eq, assert_eq};

    use crate::{
        configurations::oauth::{OauthCredentials, OauthGithubCredentials, OauthGoogleCredentials},
        oauth::{
            github::{GithubConfig, OauthGithubSettings},
            google::{GoogleConfig, OauthGoogleSettings},
        },
    };

    use super::*;

    #[test]
    fn oauth_api_tests() {
        // let giCrede = OauthCredentials::builder().google(google).
        /*
             GITHUB_CLIENT_ID: '89c19374f7e7b5b35164',
        GITHUB_CLIENT_SECRET: '129488cc92e2d2f91e3a5a024086396c48c65339',
        GOOGLE_CLIENT_ID: '855174209543-6m0f088e55d3mevhnr8bs0qjap8j6g0g.apps.googleusercontent.com',
        GOOGLE_CLIENT_SECRET: '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',

        */
        let github_credentials = OauthGithubCredentials::builder()
            .client_id(String::from("89c19374f7e7b5b35164"))
            .client_secret(String::from("129488cc92e2d2f91e3a5a024086396c48c65339"))
            .build();

        let google_credentials = OauthGoogleCredentials::builder()
            .client_id(String::from(
                "855174209543-6m0f088e55d3mevhnr8bs0qjap8j6g0g.apps.googleusercontent.com",
            ))
            .client_secret(String::from("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"))
            .build();

        // let oauth_credentials = OauthCredentials::builder()
        //     .google(google_credentials.clone())
        //     .github(github_credentials.clone())
        //     .build();
        let oauth_github_settings = OauthGithubSettings::builder()
            .base_url("https://oyelowo.test".to_string())
            .credentials(github_credentials)
            .build();
        let oauth_goole_settings = OauthGoogleSettings::builder()
            .base_url("https://oyelowo.test".to_string())
            .credentials(google_credentials)
            .build();
        let github = GithubConfig::new(oauth_github_settings);
        let google = GoogleConfig::new(oauth_goole_settings);

        let providers = Provider::builder().github(github).google(google).build();
        // let cache = HashMap::new();
        let mut cache_storage = HashMapCache::new();
        // {

        // }

        async_std::task::block_on(async {
            // use httpmock::{Mock, MockServer};
            // use std::time::{Duration, SystemTime};
            // use tokio_test::block_on;
            let _ = env_logger::try_init;
            // Arrange
            let server = MockServer::start_async().await;
            let mock = server
                .mock_async(|when, then| {
                    when.path_contains("google.com");
                    then.status(200);
                })
                .await;
// https://accounts.google.com/o/oauth2/auth?client_id=XXXXX&redirect_uri=http://localhost:8080/WEBAPP/youtube-callback.html&response_type=code&scope=https://www.googleapis.com/auth/youtube.upload

            let redirect_uri = "https://oyelowo.test/redirect?code=g0ZGZmNjVmOWI&state=dkZmYxMzE2";
            // let redirect_uri = "https://oyelowo.test/redirect?code=g0ZGZmNjVmOWI&state=dkZmYxMzE2";
//             let redirect_uri = "https://accounts.google.com/o/oauth2/v2/auth?\
//  scope=email%20profile&\
//  response_type=code&\
//  state=security_token%3D138r5719ru3e1%26url%3Dhttps%3A%2F%2Foauth2.example.com%2Ftoken&\
//  redirect_uri=oyelowo.test%3A/oauth/callback&\
//  client_id=client_id";

            let conf = Config::builder()
                .base_url("https://oyelowo.test".to_string())
                .uri(Uri::from_static(redirect_uri))
                // .uri(Uri::from("/oauth/callback"))
                .provider_configs(providers)
                // .cache_storage(&mut cache_storage)
                .build();

            // let p = conf
            //     .initiate_oauth(crate::oauth::OauthProvider::Google,  cache_storage.clone())
            //     .await
            //     .unwrap();

            let p = conf.fetch_account(cache_storage).await.unwrap();

            // let k = o.0.insert("key".to_string(), "query".to_string());
            assert_eq!(4, 4);
            mock.assert_async().await;
            // let s = HashMap::from([
            //     ("1".to_string(), "2".to_string()),
            //     ("3".to_string(), "4".to_string()),
            // ]);
            // assert_eq!(cache_storage.0, s);
        });
    }
}

/* 
https://accounts.google.com/o/oauth2/v2/auth?
 scope=email%20profile&
 response_type=code&
 state=security_token%3D138r5719ru3e1%26url%3Dhttps%3A%2F%2Foauth2.example.com%2Ftoken&
 redirect_uri=com.example.app%3A/oauth2redirect&
 client_id=client_id
 */
/* 
https://accounts.google.com/o/oauth2/v2/auth?
 scope=https%3A//www.googleapis.com/auth/drive.metadata.readonly&
 access_type=offline&
 include_granted_scopes=true&
 response_type=code&
 state=state_parameter_passthrough_value&
 redirect_uri=https%3A//oauth2.example.com/code&
 client_id=client_id
*/


/* 
https://accounts.google.com/o/oauth2/v2/auth?
 scope=https%3A//www.googleapis.com/auth/drive.metadata.readonly&
 access_type=offline&
 include_granted_scopes=true&
 response_type=code&
 state=state_parameter_passthrough_value&
 redirect_uri=https%3A//oauth2.example.com/code&
 client_id=client_id
*/


/* 
https://accounts.google.com/o/oauth2/auth?
  client_id=21302922996.apps.googleusercontent.com&
  redirect_uri=https://www.example.com/back&
  scope=https://www.google.com/m8/feeds/&
  response_type=token&
  state=asdafwswdwefwsdg,
*/

/* 
https://accounts.google.com/o/oauth2/auth?client_id={clientid}.apps.googleusercontent.com&redirect_uri=urn:ietf:wg:oauth:2.0:oob&scope=https://www.googleapis.com/auth/analytics.readonly&response_type=code
*/


/*
async_std::task::block_on(async {
    use std::time::{SystemTime, Duration};
    use httpmock::{MockServer, Mock};
    use tokio_test::block_on;
    let _ = env_logger::try_init();

    // Arrange
    let server = MockServer::start_async().await;

    let mock = Mock::new()
        .return_status(200)
        .create_on_async(&server)
        .await;

    // Act
    let response = isahc::get_async(server.url("/delay")).await.unwrap();

    // Assert
    mock.assert_async().await;
});
*/
