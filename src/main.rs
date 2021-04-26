use serenity::{
    async_trait,
    prelude::*,
    model::prelude::*,
    model::id::GuildId,
    model::id::ChannelId,
    Client,
    utils::MessageBuilder,
    
};
use std::collections::HashMap;

struct LucyHandler{ 
    owner_id:u64, 
    current_channel_id: ChannelId, //ID object to be changed at runtime
    current_server_id: GuildId,
    hashmap: std::collections::HashMap<std::string::String, CommandEnum>,
}
impl LucyHandler { 
    fn new(ownr:u64,ccid:u64, ssid:u64)->Self{ //boilerplate constructor

        let mut command_hash = HashMap::new();
        command_hash.insert(String::from("!ss"), CommandEnum::Serverlist); //make the hashmap here manually.
        command_hash.insert(String::from("!cs"), CommandEnum::Channellist);
        //needs proper lifetime management. dunno how xdd
        LucyHandler{
            owner_id:ownr
            ,current_channel_id:ChannelId::from(ccid)
            ,current_server_id:GuildId::from(ssid)
            ,hashmap: command_hash
        }
    }
}
#[async_trait]
impl EventHandler for LucyHandler{
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let bootup_message = String::from("Lucy is online. All systems nominal.");
        if let Err(send_error) = &self.current_channel_id.say(&ctx.http, &bootup_message).await {
            println!("Error sending message: {:?}", send_error);      
        }

    }//Prints a few messages upon startup and updates the standard objects.

    //Idea: Owner sends DM to Bot, Bot takes input from the DM. Bot acts based on that.
    //Prototype: Send hello to a fixed channel
    async fn message(&self, ctx: Context, msg: Message) {       
        if dm_predicate(&self, &ctx, &msg)==true{
            
             match &self.hashmap.get(&msg.content){
                 Some(cmdname) => {
                    match **cmdname {
                        CommandEnum::Serverlist => {
                            serverlist(&self,&ctx,&msg).await;                           
                        },
                        CommandEnum::Channellist => {
                            channellist(&self, &ctx, &msg).await;
                        }
                    };
                 }
                 None => {
                    println!("No command registered. Sending message instead.");
                    generic_texting_execution(&self, &ctx, &msg).await;
                 }
             };
        }
    }

}

enum CommandEnum {
    Serverlist,
    Channellist,
}
/*How to make commands(atm, subject to change): 
1) Make make command function (async)
2) Make new CommandEnum variant
3) To to LucyHandler constructor and manually insert key/value pair
4) Add usecase to EventHandler::message function -> match there
*/
//Commands
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
        channeldisplay.push("[").push(i).push("] ").push(&channel.name).push("\n");
        i+=1;
    }
    if let Err(channellist_speak_err) = msg.channel_id.say(&ctx.http,channeldisplay.push("```").build()).await{
        println!("Could not send serverlist. Error: {:?}",channellist_speak_err)
    }
}

//Other functions
fn dm_predicate (lh:&LucyHandler,_ctx: &Context,msg: &Message) -> bool  {
    if msg.author.id.as_u64() == &lh.owner_id {
        if msg.is_private() == true{
            return true;
        }                   
    }
    return false;
}//Generic predicate for use in OwnerDM channel

async fn generic_texting_execution (lh:&LucyHandler,ctx: &Context,msg: &Message){ 
    let response = MessageBuilder::new().push(message_processing(msg.content.clone())).build();

    if let Err(send_error) = &lh.current_channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", send_error);      
    }
}//Sends the message content, modified by message_processing, to the current channel.

fn message_processing(mess: String) -> String{
    mess + ", bitch"
}//Modifies the sent input somehow. 


#[tokio::main]
async fn main() {
    let handler = LucyHandler::new(251121149981884423,432969367534305283,432969367534305281);
    let token = std::env::var("DISCORD_TOKEN").expect("No token found.");
    let mut lucy_client = Client::builder(token)
        .event_handler(handler)
        .await
        .expect("Failed to construct Lucy.");

    if let Err(boot_error) = lucy_client.start().await{
        println!("Failed to boot Lucy, error: {:?}", boot_error);
    } 
}

//print server/channellist as embed
//Take user input 