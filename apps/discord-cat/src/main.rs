use lazy_static::lazy_static;
use log::{debug, error, info};
use regex::Regex;
use reqwest::get;
use std::{collections::HashSet, env, time::Duration};
use futures::StreamExt;

use serenity::{
    async_trait,
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
        StandardFramework,
    },
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, UserId},
        user::User,
        interactions::InteractionResponseType,
    },
    prelude::*,
    utils::Colour,
    builder::CreateButton,
};

use background_cat::common_mistakes;
use background_cat::common_origins;

mod commands;
use commands::{FUN_GROUP, OTHER_GROUP, STATICIMAGE_GROUP, STATICTEXT_GROUP};

mod hook;
use hook::after_hook;

#[tokio::main]
async fn main() {
    kankyo::load(false).expect("expected a .env file");
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("expected a token in $DISCORD_TOKEN");

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .prefix(&env::var("BACKGROUND_CAT_PREFIX").unwrap_or_else(|_| "-".to_string()))
                .case_insensitivity(true)
        })
        .group(&STATICTEXT_GROUP)
        .group(&STATICIMAGE_GROUP)
        .group(&FUN_GROUP)
        .group(&OTHER_GROUP)
        .help(&MY_HELP)
        .after(after_hook);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

#[help]
#[strikethrough_commands_tip_in_guild(" ")]
#[strikethrough_commands_tip_in_dm(" ")]
#[individual_command_tip = " "]
#[max_levenshtein_distance(3)]
#[embed_success_colour(DARK_TEAL)]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

/// Takes a string of an URL, returns the content.
/// Helper for Error Handling.
async fn get_log(link: &str) -> Result<String, Box<dyn std::error::Error>> {
    let link: reqwest::Url = link.parse()?;
    Ok(get(link).await?.text().await?)
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        lazy_static! {
            static ref PASTEE_REGEX: Regex = Regex::new(r"https:/{2}paste.ee/p/[^\s/]+").unwrap();
        }

        if let Some(link) = PASTEE_REGEX.find(&msg.content) {
            info!(
                "Found paste.ee link: {} in message {}",
                link.as_str(),
                msg.id
            );

            let link_raw = link.as_str().replacen("/p/", "/r/", 1);
            let log = match get_log(&link_raw).await {
                Ok(o) => o,
                Err(_) => return,
            };
            debug!("Content of log: {}", log);

            let origins = common_origins(&log);

            if origins.is_empty() {
                let mistakes = common_mistakes(&log);
                if ! mistakes.is_empty() {
                    send_help_reply(msg.channel_id, mistakes, &ctx).await;
                    return;
                } else {
                    info!("Didn't find any mistakes in log ({})", link.as_str());
                }
            } else {
                info!("Detected pirated, custom or forked launcher ({})", link.as_str());
                send_origins_reply(msg.channel_id, origins, &ctx).await;
                return;
            }
            
        };

        for attachment in msg.attachments {
            let content = match attachment.download().await {
                Ok(content) => content,
                Err(_) => return,
            };
            let content_type = attachment.content_type;
            if content_type.is_some() && str::starts_with(&content_type.unwrap(), "text/plain") {
                let log = String::from_utf8_lossy(&content).into_owned();

                let paste_ee_future = upload_paste_ee(msg.channel_id, &log, &ctx, &msg.author);

                let log_parse_future = async {
                    let origins = common_origins(&log);

                    if origins.is_empty() {
                        let mistakes = common_mistakes(&log);

                        if !mistakes.is_empty() {
                            debug!("Mistakes found: {:?}", mistakes);
                            send_help_reply(msg.channel_id, mistakes, &ctx).await;
                            return;
                        } else {
                            info!(
                                "Didn't find any mistakes in attachment ({})",
                                attachment.filename
                            );
                        }
                    } else {
                        info!(
                            "Detected pirated, custom or forked launcher ({})",
                            attachment.filename
                        );
                        send_origins_reply(msg.channel_id, origins, &ctx).await;
                        return;
                    }
                };

                futures::join!(paste_ee_future, log_parse_future);
            }
        }
        return;
    }

    // TODO: delete on reaction

    async fn ready(&self, ctx: Context, ready: Ready) {
        use serenity::model::{gateway::Activity, user::OnlineStatus};

        info!("{} is connected!", ready.user.tag());
        ctx.set_presence(
            Some(Activity::playing("DM me: -info")),
            OnlineStatus::Online,
        )
        .await;
    }
}

async fn upload_paste_ee(channel_id: ChannelId, log: &String, ctx: &Context, user: &User) {
    let mut button = CreateButton::default();
    button.custom_id("upload-log");
    button.label("Upload Log");

    match channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title("Upload log to paste.ee?");
            e.colour(Colour::DARK_TEAL);
            e.description("This will make it easier for people to read your log.\nThis button only works for the user who sent the log.");
            debug!("Embed: {:?}", e);
            e
        });
        m.components(|c| {
            c.create_action_row(|r| {
                r.add_button(button);
                r
            });
            debug!("Components: {:?}", c);
            c
        });
        debug!("Embed: {:?}", m);
        m
    }).await {
        Err(why) => error!("Couldn't send message: {}", why),
        Ok(mut msg) => {
            let mut interaction_stream = msg.await_component_interactions(&ctx).timeout(Duration::from_secs(180)).build();
            let mut uploaded = false;

            while let Some(interaction) = interaction_stream.next().await {
                if interaction.user.id == user.id {
                    let paste_ee_token = env::var("PASTE_EE_TOKEN").expect("Expected paste.ee API token in $PASTE_EE_TOKEN");
                    let client = reqwest::Client::new();

                    let request_body = json::object! {
                        description: "MultiMC Background Cat Log Upload",
                        sections: [{ contents: log.as_str() }]
                    }.dump();

                    let response = json::parse(
                        client.post("https://api.paste.ee/v1/pastes")
                            .header("Content-Type", "application/json")
                            .header("X-Auth-Token", paste_ee_token)
                            .body(request_body)
                            .send()
                            .await.unwrap()
                            .text()
                            .await.unwrap()
                            .as_str()
                    ).unwrap();

                    if !&response["success"].as_bool().unwrap_or_default() {
                        error!("paste.ee upload failed");
                    } else {
                        let link = &response["link"];

                        interaction.create_interaction_response(&ctx, |r| {
                            r.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
                                d.embed(|e| {
                                    e.title("Uploaded log");
                                    e.colour(Colour::DARK_TEAL);
                                    e.field("Log uploaded to paste.ee", link, true);
                                    debug!("Embed: {:?}", e);
                                    e
                                });
                                d.components(|c| c);
                                debug!("Interaction response data: {:?}", d);
                                d
                            });
                            debug!("Interaction response: {:?}", r);
                            r
                        }).await.unwrap();

                        uploaded = true;
                        info!("Uploaded attachment log to paste.ee: {}", link);
                    }
                } else {
                    interaction.create_interaction_response(&ctx, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|d| {
                            d.ephemeral(true);
                            d.embed(|e| {
                                e.title("You are unauthorized to do this!");
                                e.colour(Colour::DARK_TEAL);
                                e.description("Only the user who sent the log can upload it to paste.ee.");
                                debug!("Embed: {:?}", e);
                                e
                            });
                            debug!("Interaction response data: {:?}", d);
                            d
                        });
                        debug!("Interaction response: {:?}", r);
                        r
                    }).await.unwrap();
                }
            }

            if !uploaded {
                msg.edit(&ctx, |m| {
                    m.embed(|e| {
                        e.title("Timed out");
                        e.colour(Colour::DARK_TEAL);
                        e.description("Log has not been uploaded");
                        debug!("Embed: {:?}", e);
                        e
                    });
                    m.components(|c| c);
                    debug!("Embed: {:?}", m);
                    m
                }).await.unwrap();
            }
        }
    }
}

async fn send_help_reply(channel_id: ChannelId, mistakes: Vec<(&str, String)>, ctx: &Context) {
    if let Err(why) = channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Automated Response: (Warning: Experimental)");
                e.colour(Colour::DARK_TEAL);
                for i in mistakes.iter() {
                    e.field(i.0, &i.1, true);
                }
                e.footer(|f| {
                    f.icon_url("https://cdn.discordapp.com/emojis/280120125284417536.png?v=1");
                    f.text("This might not solve your problem, but it could be worth a try")
                });
                debug!("Embed: {:?}", e);
                e
            });
            debug!("Embed: {:?}", m);
            m
        })
        .await {
            error!("Couldn't send message: {}", why)
        }
    return;
}

async fn send_origins_reply(channel_id: ChannelId, origins: Vec<(&str, String)>, ctx: &Context) {
    if let Err(why) = channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Automated Response: (Warning: Experimental)");
                e.colour(Colour::DARK_TEAL);
                for i in origins.iter() {
                    e.field(i.0, &i.1, true);
                }
                debug!("Embed: {:?}", e);
                e
            });
            debug!("Embed: {:?}", m);
            m
        })
        .await {
            error!("Couldn't send message: {}", why)
        }
    return;
}
