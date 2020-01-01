#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;
extern crate clap;

use std::ops::{Index, IndexMut};

use select::document::Document;
use select::predicate::Name;

use clap::{Arg, App};

error_chain! {
   foreign_links {
       ReqError(reqwest::Error);
       IoError(std::io::Error);
   }
}

#[derive(Debug)]
struct Switch {
    ed2k: bool,
    magnet: bool,
    thunder: bool,
    all: bool,
}

impl Index<&str> for Switch {
    type Output = bool;
    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "ed2k" => &self.ed2k,
            "magnet" => &self.magnet,
            "thunder" => &self.thunder,
            "all" => &self.all,
            _ => panic!("unwanted value!{}", index),
        }
    }
}

impl IndexMut<&str> for Switch {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match index {
            "ed2k" => &mut self.ed2k,
            "magnet" => &mut self.magnet,
            "thunder" => &mut self.thunder,
            "all" => &mut self.all,
            _ => panic!("unwanted value!{}", index),
        }
    }
}

fn parse_args() -> (String, Switch) {
    let matches = App::new("link_exporter")
                        .version("0.1.1")
                        .author("spin6lock")
                        .about("crawl from website html page and export download link")
                        .arg(Arg::with_name("ed2k")
                             .short("e")
                             .long("ed2k")
                             .help("show ed2k link only")
                             )
                        .arg(Arg::with_name("magnet")
                             .short("m")
                             .long("magnet")
                             .help("show magnet link only")
                             )
                        .arg(Arg::with_name("thunder")
                             .short("t")
                             .long("thunder")
                             .help("show thunder link only")
                             )
                        .arg(Arg::with_name("url")
                             .index(1)
                             .required(true)
                             )
                        .get_matches();
    let mut result = Switch { ed2k: false, magnet: false, thunder: false, all: true };
    for s in vec!["ed2k", "magnet", "thunder"] {
        if matches.is_present(s) {
            result[s] = true;
            result.all = false;
        }
    }
    let url = matches.value_of("url").unwrap().to_string();
    (url, result)
}

fn main() -> Result<()> {
    let (url, flags) = parse_args();
    let res = reqwest::get(&url)?;

    let mut links = Vec::new(); 
    Document::from_read(res)?
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|link| {
            if flags.all {
                link.contains("magnet") | link.contains("ed2k") | link.contains("thunder")
            } else {
                let mut ret = false;
                if flags.magnet {
                    ret = ret | link.contains("magnet");
                } 
                if flags.ed2k {
                    ret = ret | link.contains("ed2k");
                } 
                if flags.thunder {
                    ret = ret | link.contains("thunder");
                }
                ret
            }
        })
        .for_each(|x| links.push(x.to_string()));

    for link in links.iter() {
        println!("{}", link);
    }
    Ok(())
}
