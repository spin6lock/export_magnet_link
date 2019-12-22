#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;

use std::env;

use select::document::Document;
use select::predicate::Name;

error_chain! {
   foreign_links {
       ReqError(reqwest::Error);
       IoError(std::io::Error);
   }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
	let url = &args[1];
    let res = reqwest::get(url)?;

    let mut links = Vec::new(); 
    Document::from_read(res)?
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|link| link.contains("magnet"))
        .for_each(|x| links.push(x.to_string()));

    for link in links.iter() {
        println!("{:?}", link);
    }

    Ok(())
}
