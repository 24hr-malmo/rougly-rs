# RoughtRight API

This create allows us to communicate with RoughtRight to get data from its api.
It has some 1:1 functions:

- Get all projects
- Get all employees
- Get all customers
- Get all projects for a certain week

And also some helper functions

- Get all projects as a hashmap
- Get all employees as a hashmap
- Get all customers as a hashmap
- Get all projects on a certain week and return who will work on those procects

## Usage

```rust

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
```
