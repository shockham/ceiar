#![feature(convert)]

extern crate temple;
extern crate banana_rs;
extern crate kv_cab;

use temple::*;
use banana_rs::{App, Request};
use kv_cab::KV;

fn main() -> () {
    let mut a = App::new();

    fn root_handler(_:Request) -> String{
        base_template("shockham", "programmer".to_string())
    }

    fn post_handler(req:Request) -> String{
        let title = req.route.trim_matches('/');
        
        let db = KV::new("./db.cab");
        let body = db.get(title.to_string())
            .unwrap_or("Sorry this page could not be found".to_string());

        base_template(title, body)
    }

    fn base_template(page_title:&str, body:String) -> String {
        html(title(page_title.to_string()),
            h1("title", page_title.to_string()) +
            div("menu", get_menu()).as_str() +
            div("container",
                p("", body)
            ).as_str()
        )
    }

    fn get_menu() -> String {
        let db = KV::new("./db.cab");
        db.keys().iter()
            .map(|k| {
                let menu_item = elem("a", format!("href=\"/{}\"", k).as_str(), format!("{}", k));
                menu_item
            }).collect()
    }

    a.routes.insert("^/$", root_handler); 
    a.routes.insert("^/(?P<title>[^']+)$", post_handler);

    a.run("127.0.0.1:8080");
}
