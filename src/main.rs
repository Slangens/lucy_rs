mod commands; //Imports the command functions

use serenity::{
    async_trait,
    prelude::*,
    model::{prelude::*,channel::Message, gateway::Ready},
    Client,
    framework::standard::StandardFramework,
};

use commands::{ //get all the commands and structures
    control::*,
    users::*,
};
use tokio::sync::RwLock;
use std::sync::Arc;


#[async_trait]
impl EventHandler for LucyHandler{
    async fn ready(& self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, _ctx: Context, _msg: Message) {       
        //cmd(&self,&ctx,&msg).await;
    }
}

#[tokio::main]
async fn main() {
    let handler = LucyHandler{};
    let token = std::env::var("DISCORD_TOKEN").expect("No token found.");

    let framework = StandardFramework::new()
        .configure(|c| c.with_whitespace(true)
                        .prefixes(vec!["Lucy,", "lucy,", "+"])
                        //.prefix("‏‏‎-‎")
                        .delimiters(vec![", ", ","])
                        .owners(vec![UserId(251121149981884423)].into_iter().collect())  //Set owner id here
                        .no_dm_prefix(true) 
                        )
        .group(&USERS_GROUP)
        .group(&CONTROL_GROUP)
        .help(&MY_HELP);

    let mut lucy_client = Client::builder(token)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Failed to construct Lucy.");

    {//In block so the locks are closed ASAP
        let mut data = lucy_client.data.write().await;
        data.insert::<CurrentServerId>(Arc::new(RwLock::new(GuildId::from(432969367534305281))));
        data.insert::<CurrentChannelId>(Arc::new(RwLock::new(ChannelId::from(432969367534305283))));
    } //sets the start values of Current server and channel ID.

    if let Err(boot_error) = lucy_client.start().await{
        println!("Failed to boot Lucy, error: {:?}", boot_error);
    } 

}
