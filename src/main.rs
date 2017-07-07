mod reddit;

use reddit::Connection;

fn main() {
    let mut connection = Connection::new("u/iapark testing rust bot");
    connection.get("https://www.reddit.com/r/all/comments.json");
}