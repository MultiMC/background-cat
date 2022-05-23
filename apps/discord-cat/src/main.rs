use lazy_static::lazy_static;
use log::{debug, error, info};
use regex::Regex;
use reqwest::get;
use std::{collections::HashSet, env};

use serenity::{
    async_trait,
    builder::{CreateActionRow, CreateButton},
    client::Context,
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
        StandardFramework,
    },
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, GuildId, MessageId, UserId},
        interactions::message_component::ButtonStyle,
    },
    prelude::*,
    utils::Colour,
};

use background_cat::common_mistakes;

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

            let mistakes = common_mistakes(&log);
            if !mistakes.is_empty() {
                send_reply(msg.channel_id, mistakes, ctx).await;
                return;
            } else {
                info!("Didn't find any mistakes in log ({})", link.as_str());
            }
        };

        for attachment in &msg.attachments {
            let content = match attachment.download().await {
                Ok(content) => content,
                Err(_) => return,
            };
            if let Some(ctype) = &attachment.content_type {
                if !ctype.to_ascii_lowercase().contains("charset=utf-8") {
                    continue;
                }
            }
            let log = String::from_utf8_lossy(&content).into_owned();
            let mistakes = common_mistakes(&log);
            if !mistakes.is_empty() {
                debug!("Mistakes found: {:?}", mistakes);
                send_reply(msg.channel_id, mistakes, ctx).await;
                return;
            } else {
                info!(
                    "Didn't find any mistakes in attachment ({})",
                    attachment.filename
                );
            }
        }

        // TODO: delete on reaction

        lazy_static! {
            static ref PASTE_URL: String =
                env::var("PASTEBIN_URL").unwrap_or_else(|_| "".to_string());
        }
        if PASTE_URL.as_str() == "" {
            return;
        }
        let mut rows: Vec<CreateActionRow> = Vec::new();
        let mut row = CreateActionRow::default();
        for attachment in &msg.attachments {
            if let Some(ctype) = &attachment.content_type {
                if !ctype.to_ascii_lowercase().contains("charset=utf-8") {
                    continue;
                }
            }
            let mut button = CreateButton::default();
            button.style(ButtonStyle::Link);
            button.label(format!("View {}", attachment.filename));
            button.emoji('📜');
            button.url(format!(
                "{}/{}/{}/{}",
                PASTE_URL.as_str(),
                msg.channel_id,
                attachment.id,
                attachment.filename
            ));
            row.add_button(button);
            if row.0.len() >= 5 {
                rows.push(row.clone());
                row = CreateActionRow::default()
            }
        }
        // if the length of the current row isn't 0, add it to the list of rows
        if !row.0.is_empty() {
            rows.push(row)
        }
        if rows.is_empty() {
            return;
        }
        if let Err(err) = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.content(format!(
                    "Web version of attachments from <@{}>",
                    msg.author.id
                ))
                .allowed_mentions(|am| am.empty_parse())
                .components(|c| {
                    for actionrow in rows {
                        c.add_action_row(actionrow);
                    }
                    c
                })
                .reference_message(&msg)
            })
            .await
        {
            error!("Error sending message: {:?}", err)
        }
        return;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        use serenity::model::{gateway::Activity, user::OnlineStatus};

        info!("{} is connected!", ready.user.tag());
        ctx.set_presence(
            Some(Activity::playing("DM me: -info")),
            OnlineStatus::Online,
        )
        .await;
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        message_id: MessageId,
        _guild_id: Option<GuildId>,
    ) {
        if let Ok(messages) = channel_id
            .messages(&ctx.http, |get| get.after(message_id).limit(10))
            .await
        {
            for message in messages {
                if message.is_own(&ctx.cache) {
                    if let Some(ref referenced) = message.referenced_message {
                        if referenced.id == message.id {
                            if let Err(err) = message.delete(&ctx.http).await {
                                error!("Failed to delete message: {:?}", err);
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn send_reply(channel_id: ChannelId, mistakes: Vec<(&str, String)>, ctx: Context) {
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
        .await
    {
        error!("Couldn't send message: {}", why)
    }
}
