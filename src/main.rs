mod pass;

use crate::pass::entry;
use crate::pass::render_password;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    let ascii = r#"
    #          #     #####  #######         ######     #     #####   #####           #####  #       ### 
#         # #   #     #    #            #     #   # #   #     # #     #         #     # #        #  
#        #   #  #          #            #     #  #   #  #       #               #       #        #  
#       #     #  #####     #            ######  #     #  #####   #####          #       #        #  
#       #######       #    #            #       #######       #       #         #       #        #  
#       #     # #     #    #            #       #     # #     # #     #         #     # #        #  
####### #     #  #####     #            #       #     #  #####   #####           #####  ####### ### 
                                #######                                 #######                     
  ##                                                                                                                                                                                                                                                                                           
 #   #####   ####     #    #  ####  #####     ####  #####  ####  #####  ######    #####  ######   ##   #         #####    ##    ####   ####  #    #  ####  #####  #####   ####     #    # ###### #####  ######    #    # #   #    #    # #  ####   ####    ##      #    #    # # #      #      
#    #    # #    #    ##   # #    #   #      #        #   #    # #    # #         #    # #       #  #  #         #    #  #  #  #      #      #    # #    # #    # #    # #         #    # #      #    # #         ##  ##  # #     ##   # # #    # #    #  #  #     #    #    # # #      #      
#    #    # #    #    # #  # #    #   #       ####    #   #    # #    # #####     #    # #####  #    # #         #    # #    #  ####   ####  #    # #    # #    # #    #  ####     ###### #####  #    # #####     # ## #   #      # #  # # #      #      #    #    #    #    # # #      #      
#    #    # #    #    #  # # #    #   #           #   #   #    # #####  #         #####  #      ###### #         #####  ######      #      # # ## # #    # #####  #    #      #    #    # #      #####  #         #    #   #      #  # # # #  ### #  ### ######    #    # ## # # #      #      
 #   #    # #    #    #   ## #    #   #      #    #   #   #    # #   #  #         #   #  #      #    # #         #      #    # #    # #    # ##  ## #    # #   #  #    # #    #    #    # #      #   #  #         #    #   #      #   ## # #    # #    # #    #    #    ##  ## # #      #      
  ## #####   ####     #    #  ####    #       ####    #    ####  #    # ######    #    # ###### #    # ######    #      #    #  ####   ####  #    #  ####  #    # #####   ####     #    # ###### #    # ######    #    #   #      #    # #  ####   ####  #    #    #    #    # # ###### ###### 
                                                                                                                                                                                                                                                                                               
                                                                  ##   
#####  #    # # #    #    #    # #####     #      # ###### ######   #  
#    # #    # # ##   #    #    # #    #    #      # #      #         # 
#    # #    # # # #  #    #    # #    #    #      # #####  #####     # 
#####  #    # # #  # #    #    # #####     #      # #      #         # 
#   #  #    # # #   ##    #    # #   #     #      # #      #        #  
#    #  ####  # #    #     ####  #    #    ###### # #      ###### ##   
                                                                       
    "#;

    println!("{ascii}");
    loop {
        println!("last_pass_menu: ");
        println!("1. add a password");
        println!("2. list saved passwords");
        println!("3. search saved passwords");
        println!("4. quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
           "1" => {
            
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
                    println!("no saved passowords for this service");
                }
            }
           } 
           "4" => {
           clr();
           }
        }
    }
}


