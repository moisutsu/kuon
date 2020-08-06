use crate::models::*;
use serde_derive::*;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub entities: UserEntities,
    pub protected: bool,
    pub followers_count: u64,
    pub friends_count: u64,
    pub listed_count: u64,
    pub created_at: String, // TODO: use datetime type
    pub favourites_count: u64,
    pub utc_offset: Option<Value>,
    pub time_zone: Option<Value>,
    pub geo_enabled: Option<Value>,
    pub verified: bool,
    pub statuses_count: u64,
    pub lang: Option<Value>,
    pub contributors_enabled: Option<Value>,
    pub is_translator: Option<Value>,
    pub is_translation_enabled: Option<Value>,
    pub profile_background_color: Option<Value>,
    pub profile_background_image_url: Option<Value>,
    pub profile_background_image_url_https: Option<Value>,
    pub profile_background_tile: Option<Value>,
    pub profile_image_url: Option<Value>,
    pub profile_image_url_https: String,
    pub profile_banner_url: Option<String>,
    pub profile_link_color: Option<Value>,
    pub profile_sidebar_border_color: Option<Value>,
    pub profile_sidebar_fill_color: Option<Value>,
    pub profile_text_color: Option<Value>,
    pub profile_use_background_image: Option<Value>,
    pub has_extended_profile: Option<Value>,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub following: Option<Value>,
    pub follow_request_sent: Option<Value>,
    pub notifications: Option<Value>,
    pub translator_type: Option<Value>,
}