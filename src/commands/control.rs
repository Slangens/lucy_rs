
use serenity::{
    prelude::*,
    model::{prelude::*,id::GuildId,id::ChannelId},  
    utils::MessageBuilder,
    cache::{Cache, Settings},
};

pub struct LucyHandler{ 
    owner_id:u64, 
    current_channel_id: ChannelId, //ID object to be changed at runtime
    current_server_id: GuildId,
    cache:Cache,
}
impl LucyHandler { 
    pub fn new(ownr:u64,ccid:u64, ssid:u64,n_msg:usize)->Self{ //boilerplate constructor

        let mut sett = Settings::new();
        sett.max_messages(n_msg);
        
        LucyHandler{
            owner_id:ownr
            ,current_channel_id:ChannelId::from(ccid)
            ,current_server_id:GuildId::from(ssid)
            ,cache: Cache::new_with_settings(sett)
        }
    }
    //fn cachechange(self)//...
}

pub async fn cmd (lh:&LucyHandler,ctx: &Context,msg: &Message){
    if dm_predicate(&lh, &ctx, &msg)==true{
        if msg.content == "!cl"{
            channellist(&lh, &ctx, &msg).await;
        }else if msg.content == "!sl"{
            serverlist(&lh, &ctx, &msg).await;
        }else{
            generic_texting_execution(&lh, &ctx, &msg).await;
        }
    }
}//This one does the actual switching. optimise if necessary.



fn dm_predicate (lh:&LucyHandler,_ctx: &Context,msg: &Message) -> bool  {
    if msg.author.id.as_u64() == &lh.owner_id {
        if msg.is_private() == true{
            return true;
        }                   
    }
    return false;
}//Generic predicate for use in OwnerDM channel

async fn serverlist (_lh:&LucyHandler,ctx: &Context,msg: &Message) {
    let x = match serenity::http::client::Http::get_current_user(&ctx.http).await{
        Ok(curr_usr) => curr_usr,
        Err(current_user_err) => {println!("Couldn't get current user, Error {:?}", current_user_err); return}
    };

    /*if let Ok(guilds) = x.guilds(&ctx.http).await {
        for (index, guild) in guilds.into_iter().enumerate() {
            println!("{}: {}, {}", index, guild.name, guild.id);
            if GuildId::from(guild)==lh.current_server_id {
                println!("the one");
            }
        }
    }*/


    let mut serverdisplay = MessageBuilder::new();
    serverdisplay.push("```\n");
    if let Ok(guilds) = x.guilds(&ctx.http).await {
        for (index, guild) in guilds.into_iter().enumerate() {
            serverdisplay.push("[").push(index).push("] ").push(guild.name.clone()).push("\n");
        }
    }
    if let Err(serverlist_speak_err) = msg.channel_id.say(&ctx.http,serverdisplay.push("```").build()).await{
        println!("Could not send serverlist. Error: {:?}",serverlist_speak_err)
    }
}

async fn channellist (lh:&LucyHandler,ctx: &Context,msg: &Message) {
    
    let mut channeldisplay = MessageBuilder::new();
    channeldisplay.push("```\n");    
    let channellist = match lh.current_server_id.channels(&ctx).await{
        Ok(channels) => channels,
        Err(channelretrieve_err) => {
            channeldisplay.push(format!("Failed to retrieve channels here, error {:?} \n",channelretrieve_err));
            return
        }
    };
    let chlist_iter = channellist.values();
    let mut i:u32=0;
    for channel in chlist_iter {               
        channeldisplay.push("[").push(i).push("] ").push("(").push(&channel.kind.name()).push(")").push(&channel.name).push("\n");
        i+=1;
    }
    if let Err(channellist_speak_err) = msg.channel_id.say(&ctx.http,channeldisplay.push("```").build()).await{
        println!("Could not send serverlist. Error: {:?}",channellist_speak_err)
    }
}

async fn generic_texting_execution (lh:&LucyHandler,ctx: &Context,msg: &Message){ 
    let response = MessageBuilder::new().push(message_processing(msg.content.clone())).build();

    if let Err(send_error) = &lh.current_channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", send_error);      
    }
}//Sends the message content, modified by message_processing, to the current channel.

fn message_processing(mess: String) -> String{
    mess + ", bitch"
}//Modifies the sent input somehow. 