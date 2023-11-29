use serde::{ Deserialize, Serialize };
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceDetails {
    pub service: String,
    pub username: String,
    pub password: String,
}
impl ServiceDetails {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceDetails {
            service,
            username,
            password,
        }
    }
    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }
    #[allow(dead_code)]
    pub fn from_user_input() -> Self {
        println!("this password is for: ");
        let mut service = String::new();
        io::stdin()
            .read_line(&mut service)
            .expect("failed to read new password entry");
        print!("enter username: ");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("failed to read username");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("failed to read password");

        ServiceDetails::new(
            service.trim().to_string(),
            username.trim().to_string(),
            password.trim().to_string(),
        )
    }
    
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("failed to serialize json")
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json")
            {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(json_output.as_bytes()) {
                        eprintln!("error writing to file: {}", e);
                    } else {
                        println!("successfully saved passwords to file.")
                    }
                } Err(e) => eprintln!("Error opening file: {}", e),
            }
    }
} 

pub fn render_password() -> Result<Vec<ServiceDetails>, io::Error> {
    let file = File::open("passwords.json")?;
    let reader = io::BufReader::new(file);
    let mut services = Vec::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let Ok(service_details) = ServiceDetails::from_json(&json_string) {
                services.push(service_details);
            }
        }
    }
    Ok(services)
}

pub fn entry(entry: &str) -> String {
    print!("{}", entry);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
