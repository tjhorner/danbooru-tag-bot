use diesel::PgConnection;
use teloxide::Bot;
use teloxide::adaptors::AutoSend;
use teloxide::prelude::Requester;
use teloxide::prelude::RequesterExt;
use teloxide::types::ChatId;
use futures::future::join_all;

use crate::api;
use crate::db;

pub async fn run(conn: PgConnection) {
  let post_index = db::get_post_index(&conn);
  let bot = Bot::from_env().auto_send();

  let posts = api::get_posts_after_id(post_index).await;
  match posts {
    Ok(posts) => {
      if let Some(first_post) = posts.first() {
        let id = first_post.actual_id();
        if id > post_index {
          db::set_post_index(&conn, id);
          log::info!("Updated post index to {}", id);
        }
      }

      let futures = posts.iter().map(|p| notify_users_for_post(&conn, p, &bot));
      join_all(futures).await;
    },
    Err(e) => {
      log::error!("Error getting posts: {}", e);
    }
  }
}

async fn notify_users_for_post(conn: &PgConnection, post: &api::Post, bot: &AutoSend<Bot>) {
  let subscriptions = db::get_users_subscribed_to_tags(&conn, &post.tags());
  for sub in subscriptions {
    log::info!("Notifying user {} for post {}", sub.user_id, post.actual_id());

    let msg_result = bot.send_message(
      ChatId(sub.user_id),
      format!("New post {} matches your subscription for tags: {}", post.post_url(), sub.tags)
    ).await;

    if let Err(e) = msg_result {
      log::error!("Error sending message to user {}: {}", sub.user_id, e);
    }
  }
}