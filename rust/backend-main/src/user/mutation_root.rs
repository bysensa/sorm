use super::{User, UserInput};
use async_graphql::*;
use mongodb::Database;
use validator::Validate;
use wither::Model;

#[derive(Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "user data")] user_input: UserInput,
    ) -> User {
        user_input.validate().expect("Invalid input");
        let db = ctx.data_unchecked::<Database>();
        // let mut user1 = User::builder()
        //     .first_name(user_input.first_name.into())
        //     .last_name(user_input.last_name.into())
        //     .email(user_input.email.into())
        //     .age(user_input.age)
        //     .social_media(user_input.social_media)
        //     .build();
        let mut user = User { ..user_input };

        user.save(db, None).await.expect("problem storing user");
        user
    }
}
