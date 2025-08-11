use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::{stdin,stdout,Write};

pub struct CBCACli {
    author: Option<String>,
    instance_id: String
}

impl CBCACli {
    pub fn spawn(author: Option<String>, instance_id: String) -> Self {
        Self {
            author,
            instance_id
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
                print!("[{}]> ", v);
            },
            None => {
                print!("[Unknown User]> ");
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

    pub async fn run_cli(
        &self
    ) -> Result<(), std::io::Error> {
        self.clear()?;

        loop {
            self.input(Some(format!("Hi,\nenter [1] to propose offer\nenter [2] to send message.\n")))?;
        }
    }
}