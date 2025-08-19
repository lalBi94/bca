mod  commands;

use crossterm::{execute, terminal::{Clear, ClearType}};
use std::{io::{stdin,stdout,Write}, sync::Arc};
use crate::client::CBCAClient;

#[derive(Debug)]
pub struct CBCAIdentity {
    display_name: String,
    token: String,
    current_instance: Option<String>
}

impl CBCAIdentity {
    pub fn spawn(
        display_name: String,
        token: String,
        current_instance: Option<String>
    ) -> Self {
        Self {
            display_name,
            token, 
            current_instance
        }
    }

    pub fn get_display_name(&self) -> &String {
        &self.display_name
    }

    pub fn get_token(&self) -> &String {
        &self.token 
    }

    pub fn get_current_instance(&self) -> &Option<String> {
        &self.current_instance
    }
}

pub struct CBCACli {
    author: Option<CBCAIdentity>,
    client: Arc<tokio::sync::Mutex<CBCAClient>>,
}

impl CBCACli {
    pub fn spawn(
        author: Option<CBCAIdentity>, 
        client: Arc<tokio::sync::Mutex<CBCAClient>>
    ) -> Self {
        Self {
            author,
            client
        }
    }
    
    pub fn clear(&self) -> Result<(), std::io::Error> {
        let _ = execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn input(&self, message: Option<String>) -> Result<String, std::io::Error> {
        let mut s= String::new();

        if let Some(v) = message {
            println!("{}", v);
        }

        match &self.author {
            Some(v) => {
                if let Some(i) = v.get_current_instance() {
                    print!("[{:?} - {}]> ", v, i);
                } else {
                    print!("[{:?} - Nowhere]> ", v);
                }
            },
            None => {
                print!("[Unknown User - Nowhere]> ");
            },
        }

        let _ = stdout().flush();
        stdin().read_line(&mut s)?;

        if let Some('\n') = s.chars().next_back() 
            { s.pop(); };

        if let Some('\r') = s.chars().next_back() 
            { s.pop(); };

        Ok(s)
    }

    pub async fn execute_from_str(&self, raw_command: String) -> () {
        let mut command: &str ="";
        let mut c_args: Vec<&str> = Vec::new();

        for (i, v) in raw_command.split(" ")
            .into_iter()
            .enumerate() {
                if i == 0 {
                    command = v;
                } else {
                    c_args.push(v);
                }
        }

        match command {
            "help" => { commands::help(&mut c_args).await; },
            "connect" => {}
            _ => {
                println!("unknow command.");
            }
        }
    }

    pub async fn run_cli(
        &self,
        instance_id: Option<String>
    ) -> Result<(), std::io::Error> {
        self.clear()?;
        println!("Hi,\nWelcome to BCA Protocol, the auction ochestrator.\nType 'help' to see commands.\n");

        loop {
            if let Some(v) = &instance_id {
                println!("Joining instance [{:?}]...", v);   
            }

            let command: String = self.input(None)?; 
            self.execute_from_str(command).await;
        }
    }
}