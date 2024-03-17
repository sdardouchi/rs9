use poise::serenity_prelude as serenity;
use serenity::{CreateAttachment, CreateEmbed};
use poise::reply::CreateReply;

use super::helpers;
use crate::tiktok::extractor::TikTok;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn create_reply(user_id: String, url: String) -> Result<CreateReply, String> {
	let t = TikTok::new(&url).await;
	let meta = t.get_metadata();
	if t.is_images() { 
		let mut embeds: Vec<CreateEmbed> = vec![];
		for image in t.get_images_url() {
			let embed = CreateEmbed::default()
				.title("RS9")
				.description(helpers::format_metadata(user_id.clone(), meta.clone()))
				.url(meta.share_url.clone())
				.image(image.clone());

			embeds.push(embed)
		}
		embeds.truncate(10);

		let mut reply = CreateReply::default();
		reply.embeds = embeds;

		return Ok(reply);
	} else {
		match t.download_video().await { 
			Ok(video) => {
				let filename = format!("{}.mp4", meta.id);
				let reply = CreateReply::default()
					.content(helpers::format_metadata(user_id, meta))
					.attachment(CreateAttachment::bytes(video, filename));

				return Ok(reply);
			},
			Err(msg) => {
				return Err(msg);
			}
		}
	}
}


async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
	match event {
		serenity::FullEvent::Message { new_message } => {
			match helpers::validate_url(&new_message.content) { 
				Ok(url) => {
					match create_reply(new_message.author.id.to_string(), url).await {
						Ok(reply) => {
							let _ = new_message.channel_id.send_message(&ctx.http, reply.to_prefix(serenity::MessageReference::from(new_message))).await;
							let _ = new_message.delete(ctx).await;
							return Ok(())
						},
						Err(msg) => { 
							let _ = new_message.reply(ctx, msg).await;
							let _ = new_message.delete(ctx).await;
							return Ok(())
						}
					}
				}
	
				_ => {
					return Ok(());		
				}
			}
		},
			
			&_ => {}
	}

	return Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn tiktok(
	ctx: Context<'_>,
	#[description = "The video's URL"] url: String
) -> Result<(), Error> {
	match helpers::validate_url(&url) { 
		Ok(_) => {
			match create_reply(ctx.author().id.to_string(), url).await { 
				Ok(reply) => { ctx.send(reply).await?; },
				Err(msg) => { ctx.say(msg).await?; }
			}
		},
		Err(msg) => {
			ctx.say(msg).await?;
		}
	}
	
	return Ok(());
}

pub async fn run() {
	let framework = poise::Framework::builder()
    	.options(poise::FrameworkOptions {
        	commands: vec![tiktok()],
			event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
    	.setup(move |ctx, _ready, framework| {
    	    Box::pin(async move {
    	        println!("Logged in as {}", _ready.user.name);
    	        poise::builtins::register_globally(ctx, &framework.options().commands).await?;
    	        Ok(Data {})
    	    })
    	})
    	.build();

    let token = helpers::get_bot_token();
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
		.await;

    client.unwrap().start().await.unwrap()
}
