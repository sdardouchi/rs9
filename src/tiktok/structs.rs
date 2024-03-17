use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
	pub nickname: String
}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct PlayUrl {
	pub uri: String,
	pub url_list: Vec<String>
}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct ImagePostInfo {
	pub images: Vec<Images>
}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct DisplayImages {
	pub url_list: Vec<String>
}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct Images {
	pub display_image: DisplayImages
}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct Video {
	pub play_addr: PlayUrl	
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Music {
	pub play_url: PlayUrl	
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aweme {
	pub aweme_id: String,
	pub desc: String,
	pub share_url: String,
	pub author: Author,
	pub music: Music,

	#[serde(default)]
	pub video: Video,

	#[serde(default)]
	pub image_post_info: ImagePostInfo
}
