#[macro_use]
extern crate serde_derive;

mod reddit;
use reddit::ApiResult;
extern crate futures;
use futures::Future;

use reddit::Connection;
extern crate serde_json;

use self::serde_json::{Value};

fn main() {
    let created_utc: u64 = 1499502377;
    println!("yep works {}", created_utc);

    let mut connection = Connection::new("u/iapark testing rust bot");
    let work = connection.get("https://www.reddit.com/r/all/comments.json")
                .and_then(|json| {
                    let ApiResult::Listing(l) = json; 
                    println!("json {:?}", l.children.get(0));
                    
                    Ok(())
                });
    connection.run(work).unwrap();
}