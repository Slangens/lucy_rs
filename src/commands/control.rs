
use serenity::{
    prelude::*,
    model::{id::GuildId,id::ChannelId,channel::Message},  
    utils::MessageBuilder,
    framework::standard::{Args, macros::*, CommandResult},
};
use std::sync::Arc;
use std::iter::FromIterator;
use tokio::sync::RwLock;

pub struct CurrentServerId; // boilerplate necessary to store Current server id in context::data
impl TypeMapKey for CurrentServerId{
    type Value = Arc<RwLock<GuildId>>;
}
pub struct CurrentChannelId;//same for channel id
impl TypeMapKey for CurrentChannelId{
    type Value = Arc<RwLock<ChannelId>>;
}

pub struct LucyHandler;     //event Handler for the bot


#[group] 
#[only_in(dm)]
#[owners_only]
//#[prefix("!")]
#[commands(sl,cl,ss,cs,cc, generic_texting_execution)]
#[default_command(generic_texting_execution)]
struct Control;//Group of control commands (all the ones in this file). Have effectively no prefix (see main.rs)


#[command] //Serverlist
async fn sl (ctx: &Context,msg: &Message) -> CommandResult{
    println!("Entered Serverlist.");
    let mut list = ctx.cache.guilds().await; 
    list.sort();    //Sort the serverlist consistently by serverid
    let mut serverdisplay = String::from("```\n");
    for (index, id) in list.into_iter().enumerate() {
        if let Some(name) =id.name(&ctx.cache).await{
            serverdisplay.push_str(&format!("[{}] {} \n",index,name)); //List servers in nice format
        };
            
    }
    serverdisplay.push_str("```");
    if let Err(serverlist_speak_err) = msg.channel_id.say(&ctx.http,serverdisplay).await{//Print it 
        println!("Could not send serverlist. Error: {:?}",serverlist_speak_err);
        //Err(serverlist_speak_err)
    }

    Ok(())
}//Lists all servers available to the bot with indexing.

#[command] //Channellist
async fn cl (ctx: &Context,msg: &Message) -> CommandResult{
    println!("Entered Channellist.");
    let mut channeldisplay = String::from("```\n");
     
    let id = {//make id be the current server id
        let mut o = GuildId::from(432969367534305281); //Fallback ID if true current server ID cant be found
        let data_read = ctx.data.read().await; //Read data
        match data_read.get::<CurrentServerId>(){                   //Read specifically serverid (again ?????)
            Some(x) => o = *x.clone().read().await, 
            None => println!("Failed to get current server id from cache. Displaying channels for the Test server."),
        }
        o
    };
    
    if let Ok(channellist) = id.channels(&ctx).await{

        let mut chlist = Vec::from_iter(channellist.values()); //id.channels is a hashmap, take only vals
        chlist.sort_by_key(|ch| ch.name()); 
        
        let mut i:u32=0;
        for channel in chlist {
            channeldisplay.push_str(&format!("[{}] ({}) {} \n",i,&channel.kind.name(),&channel.name));
            i+=1;
        }

        channeldisplay.push_str("```");
        if let Err(channellist_speak_err) = msg.channel_id.say(&ctx.http,channeldisplay).await{
            println!("Could not send serverlist. Error: {:?}",channellist_speak_err);
        }
    }
    Ok(())
}//Lists all channels in the current server with indexing.
 
#[command] //Serverswitch
async fn ss (ctx: &Context,msg: &Message,mut args: Args)-> CommandResult{
    
    let resultmessage = {
        if let Ok(serverindex) = args.single::<usize>() {//Read input as usize index
            let mut list = ctx.cache.guilds().await;//get serverlist again...
            list.sort();                                        //with same order...
            if let Some(id) = list.get(serverindex){    //Get the requested serverid
                if let Some(name) = id.name(&ctx.cache).await{

                    {//In block so the locks are closed ASAP
                        let mut data = ctx.data.write().await;  
                        data.insert::<CurrentServerId>(Arc::new(RwLock::new(*id)));//write the new id
                    }
                    format!("Found server {} at given index. Switched focus to that server.", name)
                }else{
                    format!("Could not get server name, even though ID was found.")
                }
            }else{
                format!("Could not find serverid at that index.")
            }
        } else {
            format!("Couldn't read server index. Aborting.")
        }
    };
    
    if let Err(send_error) = msg.channel_id.say(&ctx.http, resultmessage).await {//Send a success/fail message
        println!("Error sending message: {:?}", send_error);      
    }
    Ok(())
}//Changes server channel id by giving index argument as listed in sl

#[command] //Channelswitch
async fn cs (ctx: &Context,msg: &Message, mut args: Args)-> CommandResult{
    
    let resultmessage = {
        if let Ok(channelindex) = args.single::<usize>() { //input index
             
            let mut readsuccess = false; //using bools here to minimise the time the read handle is open
            let id ={
                let mut o = GuildId::from(432969367534305281);//dummy fallback ID if true current server ID cant be found
                let data_read = ctx.data.read().await;
                match data_read.get::<CurrentServerId>(){
                    Some(x) => {
                        o = *x.clone().read().await;
                        readsuccess = true; 
                    },
                    None => println!("Failed to get current server id from cache."),
                }
                o
            };
            if readsuccess == true { //If the current server id isnt actually read from cache, this isnt the right ID, so abort the whole process then.
                if let Ok(channellist) = id.channels(&ctx).await{
                    let mut chlist = Vec::from_iter(channellist.values());
                    chlist.sort_by_key(|ch| ch.name());     //same ordering again
                    if let Some(chan) = chlist.get(channelindex){
                        if chan.kind.name() == "text"{                  //obv only text channels are valid
                            {                                           //In block so the locks are closed ASAP
                                let mut data = ctx.data.write().await;
                                data.insert::<CurrentChannelId>(Arc::new(RwLock::new(chan.id))); //new id
                            }
                            format!("Found text channel {} at given index. Switched focus to that channel.", chan.name)
                        }else{
                            format!("You chose a non-text channel, dummy. Aborting.")
                        }
                    }else{
                        format!("Could not find channelid at that index.")
                    }
                }else{
                    format!("Could not retrieve channels of current server.")
                }
            }else{
                format!("Could not read the current server ID. Channel switching aborted.")
            }
        } else {
            format!("Couldn't read channel index. Aborting.")
        }
    };

    if let Err(send_error) = msg.channel_id.say(&ctx.http, resultmessage).await { //confirmation message
        println!("Error sending message: {:?}", send_error);      
    }

    Ok(())
}//Changes current channel id by giving index argument as listed in cl

#[command]//Current channel
async fn cc (ctx: &Context,msg: &Message)->CommandResult{ 

    let id = { //see cs and ss for more info
        let mut o = ChannelId::from(432969367534305283); //Fallback ID if true current server ID cant be found
        let data_read = ctx.data.read().await;
        match data_read.get::<CurrentChannelId>(){
            Some(x) => o = *x.clone().read().await,
            None => println!("Failed to get current channel id from cache. Speaking in default Test channel server."),
        }
        o
    };
    let response ={
        if let Some(name)=id.name(&ctx.cache).await{
            format!("The currently focused channel is {}.",name)
        }else{
            format!("No name associated to current channel id found. Most likely a faulty ID. Try switching channels.")
        }
    };
    
    if let Err(send_error) = msg.channel_id.say(&ctx.http, response).await {
        println!("Error sending message: {:?}", send_error);      
    }
    Ok(())
}//Sends the name of the currently focused channel.

#[command]
async fn generic_texting_execution (ctx: &Context,msg: &Message)->CommandResult{ 
    let response = MessageBuilder::new().push(message_processing(msg.content.clone())).build();

    let id = {
        let mut o = ChannelId::from(432969367534305283); //Fallback ID if true current server ID cant be found
        let data_read = ctx.data.read().await;
        match data_read.get::<CurrentChannelId>(){
            Some(x) => o = *x.clone().read().await,
            None => println!("Failed to get current channel id from cache. Speaking in default Test channel server."),
        }
        o
    };
    
    if let Err(send_error) = id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", send_error);      
    }
    Ok(())
}//Sends the message content, modified by message_processing, to the current channel.

fn message_processing(mess: String) -> String{
    mess
}//Modifies the sent input somehow. Optional.
