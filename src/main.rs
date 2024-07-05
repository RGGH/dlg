/// let's road test 'dialoguer'
use dialoguer::{theme::ColorfulTheme, Input};
use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct UserInfo {
    name: String,
    email: String,
    country: String,
}

fn main() {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your name")
        .interact_text()
        .unwrap();

    println!("Hello {}!", &name);

    let email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your email")
        .validate_with({
            let mut force = None;
            move |input: &String| -> Result<(), &str> {
                if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
                    Ok(())
                } else {
                    force = Some(input.clone());
                    Err("This is not a mail address; type the same value again to force use")
                }
            }
        })
        .interact_text()
        .unwrap();

    println!("Email: {}", email);

    let country: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your country")
        .with_initial_text("England".to_string())
        .interact_text()
        .unwrap();

    println!("Country : {}", &country);

    let user_data = UserInfo {
        name,
        email,
        country,
    };
    let user_data_json = serde_json::to_vec(&user_data).unwrap();
    let mut f = File::create("res.txt").unwrap();

    let _ = f.write_all(&user_data_json);
}
