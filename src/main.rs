use serenity::{
    async_trait,
    prelude::*,
    model::prelude::*,
    model::id::ChannelId,
    Client,
    utils::MessageBuilder,
    
};
use std::collections::HashMap;

#[async_trait]
trait Command {
    async fn predicate(lh: &LucyHandler,ctx: &Context,msg: &Message) -> bool;
    async fn run(lh: &LucyHandler,ctx: &Context,msg: &Message);
}

struct LucyHandler{ 
    owner_id:u64, 
    current_channel_id: ChannelId, //ID object to be changed at runtime
}

impl LucyHandler { 
    fn new(ownr:u64,ccid:u64)->Self{ //boilerplate constructor
        LucyHandler{owner_id:ownr,current_channel_id:ChannelId::from(ccid)}
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
    }//Prints a few messages upon startup.

    //Idea: Owner sends DM to Bot, Bot takes input from the DM. Bot acts based on that.
    //Prototype: Send hello to a fixed channel
    async fn message(&self, ctx: Context, msg: Message) {       
        if dm_predicate(&self, &ctx, &msg)==true{
            //Change to hashmap or something. Can't have a Yandev situation here.
            /*if Serverlist::predicate(&self, &ctx , &msg).await{
                Serverlist::run(&self, &ctx, &msg).await;
            }
            else {
                generic_texting_execution(&self, &ctx, &msg).await;
            }*/

            let command_short  = vec![String::from("!ss")];
            let command_long = vec![CommandEnum::Serverlist];
            let command_hash: HashMap<_, _> = command_short.iter().zip(command_long.iter()).collect();

             match command_hash.get(&msg.content){
                 Some(cmdname) => {
                    match **cmdname {
                        CommandEnum::Serverlist => {
                            serverlist_run(&self,&ctx,&msg).await;                           
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
}

//Command structs
struct Serverlist;
#[async_trait]
impl Command for Serverlist { //Prints a list of servers upon sending "!ss" in OwnerDM.
    async fn predicate (lh:&LucyHandler,ctx: &Context,msg: &Message) -> bool  {
        if msg.content == "!ss" && dm_predicate(lh, ctx, msg) { 
            return true;
        }
        return false;
    }

    async fn run (_lh:&LucyHandler,ctx: &Context,_msg: &Message) {
        let x = match serenity::http::client::Http::get_current_user(&ctx.http).await{
            Ok(curr_usr) => curr_usr,
            Err(current_user_err) => {println!("Couldn't get current user, Error {:?}", current_user_err); return}
        };
    
        if let Ok(guilds) = x.guilds(&ctx.http).await {
            for (index, guild) in guilds.into_iter().enumerate() {
                println!("{}: {}", index, guild.name);
            }
        }
    }
}

async fn serverlist_run (_lh:&LucyHandler,ctx: &Context,_msg: &Message) {
    let x = match serenity::http::client::Http::get_current_user(&ctx.http).await{
        Ok(curr_usr) => curr_usr,
        Err(current_user_err) => {println!("Couldn't get current user, Error {:?}", current_user_err); return}
    };

    if let Ok(guilds) = x.guilds(&ctx.http).await {
        for (index, guild) in guilds.into_iter().enumerate() {
            println!("{}: {}", index, guild.name);
        }
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
    let handler = LucyHandler::new(251121149981884423,432969367534305283);
    let token = std::env::var("DISCORD_TOKEN").expect("No token found.");
    let mut lucy_client = Client::builder(token)
        .event_handler(handler)
        .await
        .expect("Failed to construct Lucy.");

    if let Err(boot_error) = lucy_client.start().await{
        println!("Failed to boot Lucy, error: {:?}", boot_error);
    } 
}
