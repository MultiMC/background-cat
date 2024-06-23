use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
    utils::Colour,
};

use log::{debug, error};
use serde::Deserialize;

mod xkcd;
use xkcd::XKCD_COMMAND;

macro_rules! static_text_command {
    ( $($name:ident $($($aliases:literal)+)?, $title:tt, $message:tt;)+ ) => {
        #[group("Text")]
        #[commands( $($name),* )]
        struct StaticText;

        $(
            #[command]
            $( #[aliases($($aliases),+)] )?
            async fn $name(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
                if let Err(why) = msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title($title);
                        e.colour(Colour::DARK_TEAL);
                        e.description($message);
                        e.footer(|f| {
                            f.icon_url("https://cdn.discordapp.com/emojis/280120125284417536.png?v=1")
                        })
                    });
                    debug!("Message: {:?}", m);
                    m
                }).await {
                    error!("couldn't send message: {}", why);
                }
                Ok(())
            }
        )+
    };
}

macro_rules! static_image_command {
    ( $($name:ident $($($aliases:literal)+)?, $image:tt$(, $message:tt)?;)+ ) => {
        #[group("Images")]
        #[commands( $($name,)* )]
        struct StaticImage;

        $(
            #[command]
            $( #[aliases($($aliases),+)] )?
            async fn $name(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
                if let Err(why) = msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.image($image);
                        $(e.title($message);)?
                        e.colour(Colour::DARK_TEAL);
                        e.footer(|f| {
                            f.icon_url("https://cdn.discordapp.com/emojis/280120125284417536.png?v=1")
                        })
                    });
                    debug!("Message: {:?}", m);
                    m
                }).await {
                    error!("couldn't send message: {}", why);
                }
                Ok(())
            }
        )+
    };

}

// Format: Name (Optional Alias1 Alias2...) , Title , Message ;
static_text_command! {
    install_java "ijava", "Please install the right Java version:",
        "https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java";
    too_much_ram "tmram" "vazkiiram",
        "Allocating too much RAM to Minecraft is bad for performance:",
        "https://vazkii.notion.site/A-semi-technical-explanation-of-why-you-shouldn-t-allocate-too-much-RAM-to-Minecraft-78e7bd41ba6646de8d1c55c033674bce";
    mod_repost "repost" "vazkiirepost" "9mc" "9minecraft",
        "Please make sure you only download mods from reputable sources.",
        "For more info, please read https://vazkii.net/repost/";
    ipv4,
        "Add this to your Java arguments to make Minecraft prefer IPv4 over IPv6:",
        "`-Djava.net.preferIPv4Stack=true`";
    optifine,
        "To use OptiFine with MultiMC, please read this page:",
        "https://github.com/MultiMC/MultiMC5/wiki/MultiMC-and-OptiFine";
    modpack_editing_checklist "modpack_checklist" "pack_checklist" "pack_edit",
        "Modpack editing checklist:",
        "1. Make sure you have all the dependencies of mods\n\
        2. Make sure the mods are for the correct version and loader\n\
        3. Make sure you don't have duplicate mods\n\
        4. Download mods only from reputable sources, like Curseforge, \
        Modrinth or the mod's own website\n\
        5. Add mods one by one, so you know which mod is causing an issue, \
        prevents you from reading many logs\n\
        6. Smaller is better. Kitchensink modpacks not only are underperformant, \
        but mostly they are so big, that you will never use every mod in it. \
        Especially since many mods do similar things.\n\
        7. Optifine is almost always a bad idea in large modpacks, and always with Fabric\n\
        8. You don't need to always be up to date, as long as it works. Updating can even cause \
        your worlds to become unplayable because they are corrupted, or because blocks get changed \
        due to mods changing behavior.\n\
        \n\
        Even if your pack fulfills those, it's good to have somewhat of an idea on how to approach \
        modpack creating. This also goes for modifying existing packs.";
}

// Format: Name (Optional Alias1 Alias2...) , Image Link (, Optional Message) ;
static_image_command! {
    upload_log "log", "https://raw.githubusercontent.com/MultiMC/background-cat/master/apps/discord-cat/src/images/upload_log.png",
        "Please upload your log:";
    select_java "sjava", "https://raw.githubusercontent.com/MultiMC/background-cat/master/apps/discord-cat/src/images/set_java.png",
        "Please select your Java version in the MultiMC settings:";
    select_memory "smemory" "sram", "https://raw.githubusercontent.com/MultiMC/background-cat/master/apps/discord-cat/src/images/set_ram.png",
        "Please set your instance memory allocation:";
    install_forge "iforge", "https://raw.githubusercontent.com/MultiMC/background-cat/master/apps/discord-cat/src/images/install_forge.gif",
        "How to install Forge:";
    javaarg "javaargs" "jarg" "jargs",
        "https://raw.githubusercontent.com/MultiMC/background-cat/master/apps/discord-cat/src/images/jvm_args.png";
    select_java_instance "instjava",
        "https://raw.githubusercontent.com/MultiMC/background-cat/master/apps/discord-cat/src/images/java-instance-select.jpg",
        "Please select the right java for your current instance:";

}

#[group]
#[commands(info)]
struct Other;

#[command]
async fn info(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let creator_name = match UserId::from(185_461_862_878_543_872).to_user(ctx).await {
        Ok(o) => o.tag(),
        Err(why) => {
            error!("Couldn't get info about creator: {}", why);
            "<Error getting name>".to_string()
        }
    };
    if let Err(why) = msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("<:backgroundcat:280120125284417536>A bot to parse logfiles on the MultiMC discord<:backgroundcat:280120125284417536>");
                    e.colour(Colour::DARK_TEAL);
                    e.description(format!(r"
Developed by {}.
To start, just upload a log from MultiMC. (Type `-log` for help)

[Source Code available under AGPLv3](https://github.com/MultiMC/background-cat)
", creator_name))
                });
                m
            }).await {
                error!("Couldn't send info message: {}", why)
            }
    Ok(())
}

#[group]
#[commands(drama, xkcd)]
struct Fun;

#[derive(Deserialize)]
struct Drama {
    drama: String,
    version: String,
    seed: String,
}

#[command]
#[description = "Generate some Minecraft modding drama."]
#[description = "Add 'fabric' as the first argument for Fabric-brand drama"]
#[usage = "[fabric]"]
async fn drama(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    const MC_DRAMA: &str = "https://ftb-drama.herokuapp.com";
    const FABRIC_DRAMA: &str = "https://fabric-drama.herokuapp.com";

    let dest = if msg.content.to_lowercase().contains("fabric") {
        FABRIC_DRAMA
    } else {
        MC_DRAMA
    }
    .to_owned();

    let drama = reqwest::get(&(dest.clone() + "/json"))
        .await?
        .json::<Drama>()
        .await?;
    let permalink = dest + "/" + &drama.version + "/" + &drama.seed;

    if let Err(why) = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("MC Drama Generator");
                e.description(&drama.drama);
                e.colour(Colour::DARK_TEAL);
                e.footer(|f| {
                    f.icon_url("https://cdn.discordapp.com/emojis/280120125284417536.png?v=1");
                    f.text(permalink)
                })
            })
        })
        .await
    {
        error!("Couldn't send drama: {}", why);
    }

    Ok(())
}
