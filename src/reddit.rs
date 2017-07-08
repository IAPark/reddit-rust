extern crate futures;
extern crate hyper;
extern crate hyper_tls;

extern crate tokio_core;

extern crate serde_json;

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
#[derive(Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ApiResult {
    Listing(Listing)
}
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Listing {
    pub modhash: String,
    pub children: Vec<ApiEntity>,
    pub after: String,
    pub before: Option<String>
}

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ApiEntity {
    t1(Comment)
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct Comment {
    pub subreddit_id: String,
    pub edited: bool,
    pub banned_by: Option<String>,
    pub removal_reason: Option<String>,
    pub link_id: String,
    pub link_author: String,
    pub likes: Option<String>, // I suspect this is now unused
    pub replies: String,
    pub user_reports: Vec<String>,
    pub saved: bool,
    pub id: String,
    pub gilded: u64,
    pub archived: bool,
    pub score: u64,
    pub report_reasons: Value,
    pub author: String,
    pub num_comments: u64,
    pub parent_id: String,
    pub subreddit_name_prefixed: String,
    pub approved_by: Option<String>,
    pub over_18: bool,
    pub controversiality: f64,
    pub body: String,
    pub link_title: String,
    pub author_flair_css_class: Option<String>,
    pub downs: i64,
    pub body_html: String,
    pub quarantine: bool,
    pub can_gild: bool,
    pub subreddit: String,
    pub name: String,
    pub score_hidden: bool,
    pub num_reports: Option<i64>,
    pub link_permalink: String,
    pub stickied: bool,
    pub created: u64,
    pub author_flair_text: Option<String>,
    pub link_url: String,
    pub created_utc: u64,
    pub distinguished: Option<String>,
    pub mod_reports: Vec<String>,
    pub subreddit_type: String,
    pub ups: u64
}

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

    pub fn get(&self, uri: &str) -> Box<Future<Item = ApiResult, Error = hyper::Error>> {
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
