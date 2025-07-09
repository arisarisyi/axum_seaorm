use serde::{Deserialize, Serialize};

pub struct PostModel{
    pub id: i32,
    pub text: String,
    pub user_id: i32,
    pub image:String,
    pub title:String,
}

#[derive(Serialize,Deserialize)]
pub struct CreatePostModel{
    pub text: String,
    pub image:String,
    pub title:String,
}