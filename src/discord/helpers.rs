use regex::Regex;
use std::env::var;
use dotenv::dotenv;
use indoc::formatdoc;

use crate::tiktok::extractor::Metadata;

pub fn validate_url(msg: &str) -> Result<String, String> {
	let re = Regex::new("https://vm.tiktok\\.com\\/{1}[a-zA-Z0-9]{9}[\\/]{0,1}|(https:\\/\\/www\\.tiktok\\.com\\/@[a-zA-Z0-9._]{0,32}\\/video\\/[0-9]{19}[?]{0,1}.{0,40})")
		.expect("Regex is wrong");
	

	let caps = re.captures(msg); 
	
	if !caps.is_some() {
		return Err("Link is invalid".to_string());
	}

	return Ok(caps.unwrap()[0].to_string());
}

pub fn get_bot_token() -> String {
	match var("DISCORD_BOT_TOKEN") {
		Ok(token) => {
			return token
		}
		
		Err(_) => {
			eprintln!("Coudln't find a token in envvars, trying .env");
			dotenv().expect("Couldn't find a .env file, try again");
			return var("DISCORD_BOT_TOKEN")
				.expect("Couldn't find a DISCORD_BOT_TOKEN var in .env, aborting");
		}
	}
}

pub fn format_metadata(user_id: String, meta: Metadata) -> String {
	return formatdoc! {"
		**Requested by:** <@{}>
		**Author:** {}
		**Desc:** {}
		[Raw link](<{}>), [Music link]({})
		",
		user_id,
		meta.author,
		meta.desc,
		meta.media_url,
		meta.music_url
	};
}
