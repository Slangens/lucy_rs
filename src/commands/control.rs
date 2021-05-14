
use serenity::{
    prelude::*,
    model::{prelude::*,id::GuildId,id::ChannelId},  
    utils::MessageBuilder,
    cache::{Cache, Settings},
};
use std::iter::FromIterator;

pub struct LucyHandler{ 
    owner_id:u64, 
    current_channel_id: ChannelId, //ID object to be changed at runtime
    current_server_id: GuildId,
}
impl LucyHandler { 
    pub fn new(ownr:u64,ccid:u64, ssid:u64)->Self{ //boilerplate constructor

        LucyHandler{
            owner_id:ownr
            ,current_channel_id:ChannelId::from(ccid)
            ,current_server_id:GuildId::from(ssid)
        }
    }

}

pub async fn cmd (lh:&LucyHandler,ctx: &Context,msg: &Message){
    if dm_predicate(&lh, &ctx, &msg)==true{
        if msg.content == "!cl"{
            channellist(&lh, &ctx, &msg).await;
        }else if msg.content == "!sl"{
            serverlist(&lh, &ctx, &msg).await;
        }else if msg.starts_with("!ss") {
            serverselect(&lh, &ctx, &msg);
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
    
    let mut list = ctx.cache.guilds().await;
    list.sort();
    let mut serverdisplay = MessageBuilder::new();
    serverdisplay.push("```\n");
    for (index, id) in list.into_iter().enumerate() {
        let name =id.name(&ctx.cache).await.unwrap_or(String::from("Server could not be reached."));
            serverdisplay.push("[").push(index).push("] ").push(name).push("\n");
    }

    if let Err(serverlist_speak_err) = msg.channel_id.say(&ctx.http,serverdisplay.push("```").build()).await{
        println!("Could not send serverlist. Error: {:?}",serverlist_speak_err)
    }
}

async fn channellist (lh:&LucyHandler,ctx: &Context,msg: &Message) {
    
    let mut channeldisplay = String::from("```\n");
    
    if let Ok(channellist) = lh.current_server_id.channels(&ctx).await{
            
        let mut chlist = Vec::from_iter(channellist.values());//channellist.values().collect();
        chlist.sort_by_key(|ch| ch.id);
        let mut i:u32=0;
        for channel in chlist {
            channeldisplay.push_str(&format!("[{}] ({}) {} \n",i,&channel.kind.name(),&channel.name));//").push(i).push("] ").push(" (").push(&channel.kind.name()).push(") ").push(&channel.name).push("\n");
            i+=1;
        }
        channeldisplay.push_str("```");
        if let Err(channellist_speak_err) = msg.channel_id.say(&ctx.http,channeldisplay).await{
            println!("Could not send serverlist. Error: {:?}",channellist_speak_err);
        }
    }
    
}

async fn serverselect (_lh:&LucyHandler,ctx: &Context,msg: &Message){

}

async fn channelselect (_lh:&LucyHandler,ctx: &Context,msg: &Message){
    
}

async fn generic_texting_execution (lh:&LucyHandler,ctx: &Context,msg: &Message){ 
    let response = MessageBuilder::new().push(message_processing(msg.content.clone())).build();

    if let Err(send_error) = &lh.current_channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", send_error);      
    }
}//Sends the message content, modified by message_processing, to the current channel.

fn message_processing(mess: String) -> String{
    mess
}//Modifies the sent input somehow. 


//MO: make persistent list of servers/channels and switch via argument.