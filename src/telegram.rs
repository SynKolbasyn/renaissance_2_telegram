use std::cmp::max;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use rayon::prelude::*;


#[derive(Deserialize, Serialize, Debug)]
struct Status {
    ok: bool,
    result: Vec<Update>,
}


#[derive(Deserialize, Serialize, Debug)]
struct Update {
    update_id: i64,
    message: Message,

}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    pub text: String,
    pub from: User,
}


impl Message {

}


#[macro_export]
macro_rules! bot {
    ($id:expr) => {
        Bot::from_token($id.to_string())
    };
}


pub struct Bot {
    token: String,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}


impl Bot {
    pub fn from_token(token: String) -> Bot {
        let (tx, rx): (Sender<Message>, Receiver<Message>) = channel();
        Bot {
            token,
            tx,
            rx,
        }
    }


    fn get_updates(token: &String, last_update_id: i64) -> Result<Vec<Update>> {
        let client: Client = Client::new();
        let res: Response = client.get(format!("{}{}{}", "https://api.telegram.org/bot", token, "/getUpdates"))
            .timeout(Duration::from_secs(10))
            .query(&[("offset", last_update_id + 1)])
            .send()?;

        let result: Vec<Update> = from_str::<Status>(res.text()?.as_str())?.result;

        println!("{:#?}", result);

        Ok(result)
    }


    fn send_text(token: &String, text: String, chat_id: String) -> Result<()>{
        let client: Client = Client::new();
        let res: Response = client.get(format!("{}{}{}", "https://api.telegram.org/bot", token, "/sendMessage"))
            .timeout(Duration::from_secs(10))
            .query(&[("chat_id", chat_id.as_str()), ("text", text.as_str())])
            .send()?;

        // println!("{:#?}", res);

        Ok(())
    }
    
    
    pub fn start(self, process_message: fn(message: Message) -> Option<Result<(String, String)>>) -> Result<()> {
        let token: String = self.token.clone();

        spawn(move || {
            let mut last_message_id: i64 = 0;
            let mut updates: Vec<Update> = Self::get_updates(&token, last_message_id).unwrap();
            while updates.len() != 0 {
                last_message_id = updates.last().unwrap().update_id;
                sleep(Duration::from_secs(1));
                updates = Self::get_updates(&token, last_message_id).unwrap();
            }
            
            loop {
                updates = Self::get_updates(&token, last_message_id).unwrap();
                // let messages: Vec<Message> = updates.par_iter().map(|i| { i.message.clone() }).collect();
                for update in updates {
                    last_message_id = update.update_id;
                    self.tx.send(update.message).unwrap();
                }
                sleep(Duration::from_secs(1));
            }
        });


        let token: String = self.token.clone();
        
        let mut prev_send_time = Instant::now();
        for message in &self.rx {
            match process_message(message) {
                Some(Ok((T, I))) => {
                    while prev_send_time.elapsed() < Duration::from_millis(333) {};
                    Self::send_text(&token, T, I)?;
                    prev_send_time = Instant::now();
                },
                Some(Err(e)) => bail!(e),
                None => (),
            }
        }

        Ok(())
    }
}
