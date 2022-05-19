use crate::schema::subscriptions;
use diesel::Queryable;

#[derive(Queryable)]
pub struct Subscription {
    pub id: i32,
    pub tag: String,
    pub user_id: i64,
}

#[derive(Insertable)]
#[table_name = "subscriptions"]
pub struct NewSubscription<'a> {
    pub tag: &'a str,
    pub user_id: &'a i64,
}

#[derive(Queryable)]
pub struct SubscriptionResult {
    pub user_id: i64,
    pub tags: String,
}

#[derive(Queryable)]
pub struct PostIndex {
    pub id: i32,
    pub last_seen_post: i32,
}
