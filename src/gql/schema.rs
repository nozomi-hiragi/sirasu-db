use async_graphql::{Context, EmptySubscription, Object, Result, Schema};

use crate::{
    gql::sirasu_user::{sirasu_login, sirasu_signin, SirasuLoginInput, SirasuSigninInput},
    utils::jwt::decode_context_token,
};

pub struct Query;
#[Object]
impl Query {
    async fn add(&self, ctx: &Context<'_>, a: i32, b: i32) -> Result<i32> {
        let _ = decode_context_token(ctx)?;
        Ok(a + b)
    }
}

pub struct Mutation;
#[Object]
impl Mutation {
    async fn sirasu_signin(&self, input: SirasuSigninInput) -> String {
        sirasu_signin(input).await.unwrap_or_else(|err| err)
    }
    async fn sirasu_login(&self, input: SirasuLoginInput) -> String {
        sirasu_login(input).await.unwrap_or_else(|err| err)
    }
}

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;
pub fn build_schema() -> ApiSchema {
    Schema::build(Query, Mutation, EmptySubscription).finish()
}
