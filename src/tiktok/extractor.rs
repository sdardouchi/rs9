use serde::{Serialize, Deserialize};
use serde_json::from_str;
use reqwest::get;
use super::helpers;

use super::structs::Aweme;

#[derive(Debug, Serialize, Deserialize)]
pub struct TikTok {
	pub aweme_list: Vec<Aweme>	
}

#[derive(Clone)]
pub struct Metadata {
	pub id:	String,
	pub author: String,
	pub desc: String,
	pub media_url: String,
	pub music_url: String,
	pub share_url: String,
}

impl TikTok {	
	pub async fn new(url: &str) -> TikTok {
		let mut usable_url = url.to_string();
		if url.contains("vm.tiktok.com") {
			usable_url = helpers::unshorten_link(&url).await;	
		}
		
		let aweme_id = helpers::get_aweme_id(&usable_url);	
		let api_url = format!("https://api22-normal-c-useast2a.tiktokv.com/aweme/v1/feed/?aweme_id={}", aweme_id);

		let body = get(api_url)
			.await
			.expect("Couldn't fetch URL content")
			.text()
			.await
			.expect("Couldn't get Tiktok URL body");
		

		return from_str(&body)
			.expect("Couldn't serialize item");
	}
	
	pub async fn download_video(&self) -> Result<Vec<u8>, String> {
		let video_url = self.aweme_list[0].video.play_addr.url_list[0].clone();
		
		match reqwest::get(video_url).await {
			Ok(result) => {
				match result.bytes().await {
					Ok(body) => {
						return Ok(body.to_vec());
					},
					
					Err(_) => {
						return Err("Couldn't download the video.".to_string())
					}
				}
			},

			Err(_) => {
				return Err("Couldn't GET the video URL.".to_string());
			}

		}	
	}

	pub fn get_images_url(&self) -> Vec<String> {
		let mut images_urls: Vec<String> = Default::default();
		
		for img in self.aweme_list[0].image_post_info.images.iter() {
			images_urls.push(img.display_image.url_list[0].clone());
		}

		return images_urls;
	}

	pub fn is_images(&self) -> bool {
		if self.aweme_list[0].image_post_info.images.len() != 0 {
			return true;
		}
		return false;
	}
	
	pub fn get_metadata(&self) -> Metadata {
		let media_url = if self.is_images() {
			self.aweme_list[0].image_post_info.images[0].display_image.url_list[0].clone()
		} else {
			self.aweme_list[0].video.play_addr.url_list[0].clone()
		};

		return Metadata {
			id: self.aweme_list[0].aweme_id.clone(),
			author: self.aweme_list[0].author.nickname.clone(),
			desc: self.aweme_list[0].desc.clone(),
			media_url: media_url.clone(),
			music_url: self.aweme_list[0].music.play_url.uri.clone(),
			share_url: self.aweme_list[0].share_url.clone()
		}
	}
}
