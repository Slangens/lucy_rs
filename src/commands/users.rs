use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{macros::*, CommandResult,help_commands,Args,HelpOptions,CommandGroup},
    utils::MessageBuilder,
};
use std::collections::HashSet;

#[group] //User commands group
#[only_in(guild)]
#[commands(ping,am_i_slang)]
struct Users;



#[command]
#[help_available]
#[description("Sends a pong back")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    
    let send = MessageBuilder::new()
    .push("You got ponged, ")
    .mention(&msg.author)
    .push(". Are you happy?")
    .build();
    let dm = msg.author.dm(
        &ctx.http, 
        |m| {
            m.content(&send)
        }).await;

    match dm {
        Ok(_x) => {// P I N G
            let _ = msg.react(&ctx, '🇵').await;
            let _ = msg.react(&ctx, '🇮').await;
            let _ = msg.react(&ctx, '🇳').await;
            let _ = msg.react(&ctx, '🇬').await;
            
        },
        Err(why) => {
            println!("Err sending pong: {:?}", why);

            let _z = msg.reply(&ctx, "pong!").await;
        },
    };

    Ok(())
}//Pongs the person that pings.

#[command]
#[help_available]
#[description("Tells you if you are my husband")]
async fn am_i_slang(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if msg.author.id == 251121149981884423 {
        msg.channel_id.say(&ctx.http,"yes, you're slang").await?;
    Ok(())
    }else{
        msg.channel_id.say(&ctx.http,"no, you're not slang").await?;
    Ok(())
    }
}//duh

#[help]
async fn my_help( //called help regardless of name here
   context: &Context,
   msg: &Message,
   args: Args,
   help_options: &'static HelpOptions,
   groups: &[&'static CommandGroup],
   owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())  
}//generic help command

