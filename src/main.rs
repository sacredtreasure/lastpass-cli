mod pass;

use crate::pass::entry;
use crate::pass::ServiceDetails;
use crate::pass::render_password;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    let ascii = r#"
    ██╗      █████╗ ███████╗████████╗     ██████╗  █████╗ ███████╗███████╗         ██████╗██╗     ██╗  ☸
    ██║     ██╔══██╗██╔════╝╚══██╔══╝     ██╔══██╗██╔══██╗██╔════╝██╔════╝        ██╔════╝██║     ██║
    ██║     ███████║███████╗   ██║        ██████╔╝███████║███████╗███████╗        ██║     ██║     ██║  ☸  ⟰
    ██║     ██╔══██║╚════██║   ██║        ██╔═══╝ ██╔══██║╚════██║╚════██║        ██║     ██║     ██║
    ███████╗██║  ██║███████║   ██║███████╗██║     ██║  ██║███████║███████║███████╗╚██████╗███████╗██║  ☸
    ╚══════╝╚═╝  ╚═╝╚══════╝   ╚═╝╚══════╝╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝ ╚═════╝╚══════╝╚═
    "#;

    println!("{ascii}");
    loop {
        let lastpassmenu = r#"
░█░░░█▀█░█▀▀░▀█▀░░░░░█▀█░█▀█░█▀▀░█▀▀░░░░░█▄█░█▀▀░█▀█░█░█░░░░
░█░░░█▀█░▀▀█░░█░░░░░░█▀▀░█▀█░▀▀█░▀▀█░░░░░█░█░█▀▀░█░█░█░█░░▀░
░▀▀▀░▀░▀░▀▀▀░░▀░░▀▀▀░▀░░░▀░▀░▀▀▀░▀▀▀░▀▀▀░▀░▀░▀▀▀░▀░▀░▀▀▀░░▀░
        "#;

        println!("{lastpassmenu}");
        println!("------------------------------------");
        println!("1. add a password");
        println!("------------------------------------");
        println!("2. list saved passwords");
        println!("------------------------------------");
        println!("3. search saved passwords");
        println!("------------------------------------");
        println!("4. quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let new_entry = ServiceDetails::new(
                    entry("service: "),
                    entry("username: "),
                    entry("password: "),
                );
                println!("password entry saved!");
                new_entry.write_to_file();
            }
            "2" => {
                clr();
                let services = render_password().unwrap_or_else(|err| {
                    eprintln!("error reading passwords: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "service {}
                    - username: {}
                    - password: {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let services = render_password().unwrap_or_else(|err| {
                    eprintln!("error reading passwords: {}", err);
                    Vec::new()
                });
                let search = entry("search: ");
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "service {}
                        - username: {}
                        - password: {}",
                            item.service, item.username, item.password
                        );
                    } else {
                        println!("no saved passwords for this service");
                    }
                }
            }
            "4" => {
                clr();
                println!("see you soon!");
                break;
            }
            _ => println!("invalid command. pick from: 1, 2 or 3 and press 4 to quit"),
        }
        println!("\n\n");
    }
}
