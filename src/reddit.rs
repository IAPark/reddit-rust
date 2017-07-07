extern crate futures;
extern crate hyper;
extern crate hyper_tls;

extern crate tokio_core;
extern crate serialize;

use self::serialize::json;
use std::env;
use std::io::{self, Write};

use self::futures::{Future, BoxFuture};
use self::futures::stream::Stream;

use self::hyper::{Method, Request};
use self::hyper::Client;

use self::tokio_core::reactor::Core;
use self::hyper::client::HttpConnector;

use self::hyper_tls::HttpsConnector;
use std::{thread, time};
use self::hyper::header::{Headers, UserAgent};


#[derive(Debug)]
pub struct Connection {
    loop_core: Core,
    client: Client<HttpsConnector<HttpConnector>>,
    userAgent: String

}
impl Connection {
    pub fn new(userAgent: &str) -> Connection {
        let mut core = Core::new().unwrap();
        let client = Client::configure()
                    .connector(HttpsConnector::new(4, &core.handle()).unwrap())
                    .build(&core.handle());
        Connection {loop_core: core, client: client, userAgent: userAgent.to_string()}
    }
    pub fn r(&self, subreddit: &str) {
        // return an object references a subreddit
    }

    pub fn get(&mut self, uri: &str) {
        let mut req = Request::new(Method::Get, uri.parse().unwrap());
        req.headers_mut().set(UserAgent::new(self.userAgent.clone()));
        
        let work = self.client.request(req).and_then(|res| {
            println!("Response: {}", res.status());

            res.body().concat2().and_then(|result| {
                Ok(())
            })
        });
    }


    pub fn post(&self, address: &str) {
        
    }
}
