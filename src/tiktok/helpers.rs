use reqwest::{Client, redirect::Policy};
use regex::Regex;

pub async fn unshorten_link(url: &str) -> String {
	let client = Client::builder()
		.redirect(Policy::none())
		.build()
		.expect("Couldn't build reqwest client.");

	let res = client.get(url)
		.send().await
		.expect("Couldn't send HTTP request");
	
	let location = res.headers()
		.get("location")
		.unwrap();

	return location
		.to_str()
		.unwrap()
		.to_string();
}

pub fn get_aweme_id(url: &str) -> String { 
	let re = Regex::new("[0-9]{19}").unwrap();
	let captures = re.captures(url)
		.expect("Couldn't capture aweme_id in URL");

	let aweme_id = captures[0].to_string();
	return aweme_id;
}
