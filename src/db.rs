use std::env;
use diesel::dsl::{sql, count};
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

pub trait Database {
  fn create_subscription(&self, tag: &str, user_id: &i64) -> Subscription;
  fn remove_subscription(&self, tag: &str, user_id: &i64) -> bool;
  fn get_subscription(&self, tag: &str, user_id: &i64) -> Option<Subscription>;
  fn has_subscription(&self, tag: &str, user_id: &i64) -> bool;
  fn get_users_subscribed_to_tags(&self, tags: &[String]) -> Vec<SubscriptionResult>;

  fn get_post_index(&self) -> i32;
  fn set_post_index(&self, post_id: i32);
}

pub struct Db {
  pub conn: PgConnection,
}

impl Database for Db {
  fn get_subscription(&self, tag: &str, user_id: &i64) -> Option<Subscription> {
    subscriptions::table
      .filter(subscriptions::tag.eq(tag))
      .filter(subscriptions::user_id.eq(user_id))
      .first::<Subscription>(&self.conn)
      .ok()
  }

  fn has_subscription(&self, tag: &str, user_id: &i64) -> bool {
    let result = subscriptions::table
      .select(count(subscriptions::user_id))
      .filter(subscriptions::tag.eq(tag))
      .filter(subscriptions::user_id.eq(user_id))
      .limit(1)
      .first(&self.conn);
    
    match result {
      Ok(1) => true,
      _ => false,
    }
  }

  fn get_users_subscribed_to_tags(&self, tags: &[String]) -> Vec<SubscriptionResult> {
    subscriptions::table
      .select((
        subscriptions::user_id,
        sql("STRING_AGG(tag, ', ') tags"),
      ))
      .filter(subscriptions::tag.eq_any(tags))
      .group_by(subscriptions::user_id)
      .load::<SubscriptionResult>(&self.conn)
      .expect("Error loading subscriptions")
  }

  fn create_subscription(&self, tag: &str, user_id: &i64) -> Subscription {
    let new_subscription = NewSubscription { tag, user_id };
  
    diesel::insert_into(subscriptions::table)
        .values(&new_subscription)
        .get_result(&self.conn)
        .expect("Error saving new subscription")
  }

  fn remove_subscription(&self, tag: &str, user_id: &i64) -> bool {
    diesel::delete(
        subscriptions::table
          .filter(subscriptions::tag.eq(tag))
          .filter(subscriptions::user_id.eq(user_id))
      )
      .execute(&self.conn)
      .is_ok()
  }

  fn get_post_index(&self) -> i32 {
    post_index::table
      .select(post_index::last_seen_post)
      .first::<i32>(&self.conn)
      .unwrap_or(0)
  }
  
  fn set_post_index(&self, post_id: i32) {
    diesel::update(post_index::table)
      .set(post_index::last_seen_post.eq(post_id))
      .execute(&self.conn)
      .expect("Error updating post index");
  }
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