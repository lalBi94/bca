pub async fn help(c_args: &mut Vec<&str>) -> () {
    if c_args.len() > 1 {
        eprintln!("Bad usage of help {:?}\n", c_args);
        return;
    }

    if c_args.len() == 0 {
        c_args.push("1");
    }

    const COMMANDS_LIST: [[&str; 8]; 1] = [
        [
            "help <page>\t\t- Display the nth page of the command list.",
            "connect <token>\t\t- Login to your BCA identity.",
            "join <instance_id>\t- Join an auction instance.",
            "*message <content>\t- Send message into auction instance.",
            "*offer <amount> <message>\t- Send an offer to auction owner.",
            "leave\t\t\t- Leave the current auction.",
            "quit\t\t\t- Leave the client",
            "*logout\t\t\t- Remove the BCA identity from your client."
        ]
    ];

    let page = usize::from_str_radix(c_args[0], 10);

    match page {
        Ok(v) => {
            if v > COMMANDS_LIST.len() {
                eprintln!("Page {} isn't exist. Min: {} Max: {}\n", v, 1, COMMANDS_LIST.len());
                return;
            }
            
            println!(
                "{}\n| Commands with * need the BCA identity to be execute.\n{}",
                "-".to_string().repeat(5),
                "-".to_string().repeat(5)
            );

            for c in COMMANDS_LIST[v-1] {
                println!("{}", c);
            }
        }, 
        Err(_) => {
            eprintln!("Bad usage of help, {} is not a number.\n", c_args[0]);
            return;
        }
    };

    println!("Page {} over {}\n", c_args[0], COMMANDS_LIST.len());
}
