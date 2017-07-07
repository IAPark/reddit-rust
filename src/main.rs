mod reddit;
extern crate futures;
use futures::Future;

use reddit::Connection;
extern crate serde_json;

use self::serde_json::{Value};

fn main() {
    let mut connection = Connection::new("u/iapark testing rust bot");
    let work = connection.get("https://www.reddit.com/r/all/comments.json")
                .and_then(|json| {
                    if let Value::Object(object) = json {
                        println!("json {}", object.get("data").unwrap());
                    }
                    
                    Ok(())
                });
    connection.run(work).unwrap();
}