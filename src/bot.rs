use diesel::PgConnection;
use teloxide::{prelude::*, utils::command::BotCommands, dispatching::{Dispatcher, update_listeners}};
use std::{error::Error, sync::{Arc}};
use tokio::sync::Mutex;
use crate::db::{Db, Database};

pub async fn run(db_connection: PgConnection) {
  log::info!("Starting bot...");

  let bot = Bot::from_env().auto_send();

  let ignore_update = |_upd| Box::pin(async {});

  let cloned_bot = bot.clone();
  let listener = update_listeners::polling_default(cloned_bot).await;
  let handler =
    Update::filter_message()
      .filter_command::<Command>()
      .chain(dptree::endpoint(answer));

  Dispatcher::builder(
     bot, handler,
  )
  .default_handler(ignore_update)
  .dependencies(dptree::deps![
    Arc::new(Mutex::new(Db { conn: db_connection }))
  ])
  .build()
  .setup_ctrlc_handler()
  .dispatch_with_listener(
    listener,
    LoggingErrorHandler::with_custom_text("An error from the update listener"),
  )
  .await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
  Help,
  #[command(description = "Subscribe to a tag")]
  Subscribe(String),
  #[command(description = "Unsubscribe from a tag")]
  Unsubscribe(String),
}

async fn answer(
  bot: AutoSend<Bot>,
  message: Message,
  command: Command,
  db: Arc<Mutex<dyn Database + Send + Sync>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  match command {
    Command::Help => {
      bot.send_message(message.chat.id, Command::descriptions().to_string()).await?;
    }
    Command::Subscribe(tag) => {
      let conn = db.lock().await;
      let tag = tag.to_lowercase();

      if tag.trim().contains(" ") {
        bot.send_message(message.chat.id, "Tags cannot contain spaces; use underscores if you are trying to subscribe to a tag with multiple words").await?;
        return Ok(());
      }

      let user_id = message.from().unwrap().id.0 as i64;

      if conn.has_subscription(&tag, &user_id) {
        bot.send_message(message.chat.id, format!("You are already subscribed to {tag}")).await?;
      } else {
        conn.create_subscription(&tag, &user_id);
        log::info!("User {} subscribed to {}", user_id, tag);
        bot.send_message(message.chat.id, format!("Subscribed to {tag}")).await?;
      }
    },
    Command::Unsubscribe(tag) => {
      let conn = db.lock().await;
      let tag = tag.to_lowercase();
      let user_id = message.from().unwrap().id.0 as i64;

      if conn.has_subscription(&tag, &user_id) {
        conn.remove_subscription(&tag, &user_id);
        log::info!("User {} unsubscribed from {}", user_id, tag);
        bot.send_message(message.chat.id, format!("Unsubscribed from {tag}")).await?;
      } else {
        bot.send_message(message.chat.id, format!("You're not subscribed to {tag}")).await?;
      }
    },
  };

  Ok(())
}