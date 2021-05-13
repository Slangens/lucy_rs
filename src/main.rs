mod commands; //Imports the command functions

use serenity::{
    async_trait,
    prelude::*,
    model::{prelude::*},
    Client,
    framework::standard::{StandardFramework},
};

use commands::{
    control::*,
    users::*,
};



#[async_trait]
impl EventHandler for LucyHandler{

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);       
    }

    async fn message(&self, ctx: Context, msg: Message) {       
        cmd(&self,&ctx,&msg).await;
    }

}



#[tokio::main]
async fn main() {
    let handler = LucyHandler::new(251121149981884423,432969367534305283,432969367534305281,2);
    let token = std::env::var("DISCORD_TOKEN").expect("No token found.");
    let framework = StandardFramework::new()
        .configure(|c| c.with_whitespace(true)
                        .prefixes(vec!["Lucy,", "lucy,", "+"])
                        //.prefix("‏‏‎ ‎")
                        .delimiters(vec![", ", ","])
                        .owners(vec![UserId(251121149981884423)].into_iter().collect()) )
        .group(&USERS_GROUP)
        .help(&MY_HELP);
    let mut lucy_client = Client::builder(token)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Failed to construct Lucy.");

    if let Err(boot_error) = lucy_client.start().await{
        println!("Failed to boot Lucy, error: {:?}", boot_error);
    } 
}

//print server/channellist as embed
//Take user input 
