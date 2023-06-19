use std::path::PathBuf;

use yew::prelude::*;

mod model;
mod components;

use model::{ParseError, Reference};
use components::split_pane::SplitPane;

mod config;
use config::{OLD_PREFIX, PREFIX};

fn parse_db(str: &str) -> Result<Vec<Reference>, ParseError> {
    str.lines()
        .map(|line| {
            let (p, qname) = line.split_once(" ")?;
            let p = p.strip_prefix(OLD_PREFIX)?;
            let p: PathBuf = [PREFIX, p].iter().collect();
            let p = p.to_str()?;
            let (m, n) = qname.rsplit_once(".")?;
            Some(Reference {
                name: n.to_string(),
                module: m.to_string(),
                path: p.to_string(),
            })
        })
        .collect::<Option<Vec<Reference>>>().ok_or(ParseError)
}

#[derive(Properties, PartialEq)]
pub struct HaystackRes {
    pub haystack_res: Result<Vec<Reference>, ParseError>,
}

#[function_component]
fn App(haystack_res: &HaystackRes) -> Html {
    if let Ok(haystack) = &haystack_res.haystack_res {
        html![ <SplitPane haystack={haystack.clone()} /> ]
    } else {
        html! {
            <div>
                <h1>{ "Error" }</h1>
                <p>{ "Failed to parse database" }</p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::with_props(
        HaystackRes { haystack_res: parse_db(include_str!("../indexed.txt")) }
    ).render();
}
