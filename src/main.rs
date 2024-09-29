use std::{env, error::Error};
use roughly_rs::RoughlyRight;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let username = env::var("EMAIL").unwrap();
    let password = env::var("PASSWORD").unwrap();

    let week = "202440";

    let mut roughly_right = RoughlyRight::new(&username, &password);

    let the_week = roughly_right.weekly_work(week).await?;

    for (_key, value) in the_week.iter() {
        println!("{}: {:?}", value.project, value.employees);
    }

    Ok(())

}
