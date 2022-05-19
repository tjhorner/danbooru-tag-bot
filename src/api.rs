use std::error::Error;

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
        self.id.unwrap_or_else(|| self.parent_id.unwrap_or(0))
    }

    pub fn post_url(&self) -> String {
        format!("https://danbooru.donmai.us/posts/{}", self.actual_id())
    }
}

#[cfg(test)]
mod tests {
    use super::Post;

    #[test]
    fn tags_splits_tag_string_correctly() {
        let post = Post {
            id: Some(1),
            parent_id: Some(2),
            tag_string: "tag1 tag2 tag3".to_string(),
            file_url: None,
        };

        assert_eq!(post.tags(), vec!["tag1", "tag2", "tag3"]);
    }

    #[test]
    fn actual_id_returns_id_if_present() {
        let post = Post {
            id: Some(1),
            parent_id: None,
            tag_string: "".to_string(),
            file_url: None,
        };

        assert_eq!(post.actual_id(), 1);
    }

    #[test]
    fn actual_id_returns_parent_id_if_no_id() {
        let post = Post {
            id: None,
            parent_id: Some(1),
            tag_string: "".to_string(),
            file_url: None,
        };

        assert_eq!(post.actual_id(), 1);
    }

    #[test]
    fn post_url_returns_correct_url() {
        let post = Post {
            id: Some(1),
            parent_id: Some(2),
            tag_string: "".to_string(),
            file_url: None,
        };

        assert_eq!(post.post_url(), "https://danbooru.donmai.us/posts/1");
    }
}

pub async fn get_posts_after_id(after_id: i32) -> Result<Vec<Post>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let posts: Vec<Post> = client
        .get("https://danbooru.donmai.us/posts.json")
        .query(&[("limit", "200"), ("page", &format!("a{after_id}"))])
        .send()
        .await?
        .json()
        .await?;

    Ok(posts)
}
