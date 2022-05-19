use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Post {
  pub id: Option<i32>,
  pub parent_id: Option<i32>,
  pub tag_string: String,
  pub file_url: Option<String>,
}

impl Post {
  pub fn tags(&self) -> Vec<String> {
    self.tag_string.split(' ').map(|s| s.to_string()).collect()
  }

  pub fn actual_id(&self) -> i32 {
    self.id.unwrap_or(self.parent_id.unwrap_or(0))
  }

  pub fn post_url(&self) -> String {
    format!("https://danbooru.donmai.us/posts/{}", self.actual_id())
  }
}

pub async fn get_posts_after_id(after_id: i32) -> Result<Vec<Post>, reqwest::Error> {
  let url = format!("https://danbooru.donmai.us/posts.json?limit=200&page=a{after_id}");
  let posts: Vec<Post> = reqwest::get(url).await?.json().await?;
  Ok(posts)
}