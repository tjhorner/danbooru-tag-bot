use std::env;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::{PgConnection, Connection, Queryable};
use super::schema::subscriptions;
use super::schema::post_index;

pub fn establish_db_connection() -> PgConnection {
  let database_url = env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
      .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
pub struct Subscription {
  pub id: i32,
  pub tag: String,
  pub user_id: i64,
}

#[derive(Insertable)]
#[table_name="subscriptions"]
pub struct NewSubscription<'a> {
  pub tag: &'a str,
  pub user_id: &'a i64,
}

pub fn get_subscription(conn: &PgConnection, tag: &str, user_id: &i64) -> Option<Subscription> {
  subscriptions::table
    .filter(subscriptions::tag.eq(tag))
    .filter(subscriptions::user_id.eq(user_id))
    .first::<Subscription>(conn)
    .ok()
}

#[derive(Queryable)]
pub struct SubscriptionResult {
  pub user_id: i64,
  pub tags: String,
}

pub fn get_users_subscribed_to_tags(conn: &PgConnection, tags: &[String]) -> Vec<SubscriptionResult> {
  subscriptions::table
    .select((
      subscriptions::user_id,
      sql("STRING_AGG(tag, ', ') tags"),
    ))
    .filter(subscriptions::tag.eq_any(tags))
    .group_by(subscriptions::user_id)
    .load::<SubscriptionResult>(conn)
    .expect("Error loading subscriptions")
}

pub fn create_subscription(conn: &PgConnection, tag: &str, user_id: &i64) -> Subscription {
  let new_subscription = NewSubscription { tag, user_id };

  diesel::insert_into(subscriptions::table)
      .values(&new_subscription)
      .get_result(conn)
      .expect("Error saving new subscription")
}

pub fn remove_subscription(conn: &PgConnection, tag: &str, user_id: &i64) -> bool {
  diesel::delete(
      subscriptions::table
        .filter(subscriptions::tag.eq(tag))
        .filter(subscriptions::user_id.eq(user_id))
    )
    .execute(conn)
    .is_ok()
}

#[derive(Queryable)]
pub struct PostIndex {
  pub id: i32,
  pub last_seen_post: i32,
}

pub fn get_post_index(conn: &PgConnection) -> i32 {
  post_index::table
    .select(post_index::last_seen_post)
    .first::<i32>(conn)
    .unwrap_or(0)
}

pub fn set_post_index(conn: &PgConnection, post_id: i32) {
  diesel::update(post_index::table)
    .set(post_index::last_seen_post.eq(post_id))
    .execute(conn)
    .expect("Error updating post index");
}