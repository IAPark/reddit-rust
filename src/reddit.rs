extern crate futures;
extern crate hyper;
extern crate hyper_tls;

extern crate tokio_core;

extern crate serde_json;
extern crate serde_derive;

use self::serde_json::{Value};

use std::io;

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

    pub fn get(&self, uri: &str) -> Box<Future<Item = Value, Error = hyper::Error>> {
        let mut req = Request::new(Method::Get, uri.parse().unwrap());
        req.headers_mut().set(UserAgent::new(self.userAgent.clone()));
        
        Box::new(self.client.request(req).and_then(|res| {
            res.body().concat2()
        }).and_then(|body| {
            serde_json::from_slice(&body).map_err(|e| {
                hyper::Error::Io(io::Error::new(io::ErrorKind::Other,e))
            })
        }))
    }

    pub fn run<F>(&mut self, future: F) -> Result<F::Item, F::Error>
                                              where F: Future,
    {
        self.loop_core.run(future)
    }


    pub fn post(&self, address: &str) {
        
    }
}
