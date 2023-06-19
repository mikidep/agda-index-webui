use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::model::Reference;

#[derive(PartialEq, Properties)]
pub struct SearchViewProps {
    pub haystack: Vec<Reference>,
    pub frame_cb: Callback<String>,
}

pub struct SearchView {
    needle: String,
    filtered: Vec<Reference>,
}

impl Component for SearchView {
    type Message = String;
    type Properties = SearchViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            needle: "".to_string(),
            filtered: ctx.props().haystack.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let frame_cb = ctx.props().frame_cb.clone();

        html! {<>
            <div class="searchbox-container">
                <input
                    type="text"
                    class="searchbox"
                    id="i2d_searchbox"
                    value={self.needle.clone()}
                    onchange={ctx.link().callback(|e: Event| {
                        let input = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                        input.map(|i| i.value()).unwrap_or("".to_string())
                })}/>
                <span class="key-shortcut">{ "Tab ⇥" }</span>
            </div>
            <ul id="i2d_search_results">
                { for self.filtered.iter().map(|r| {
                    let p = r.path.clone();
                    html! {
                        <li class="indexentry">
                            <a onclick={ frame_cb.reform(move |_| p.clone()) }>
                                <span class="name">{ r.name.clone() }</span>
                                <span class="package dimmed">{ "cubical" }</span>
                                <br />
                                <span class="namespace dimmed">{ r.module.clone() }</span>
                            </a>
                        </li>
                }}) }
            </ul>
        </>}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.needle = msg;
        self.filtered = ctx.props().haystack.iter()
            .filter(|r| r.name.contains(&self.needle))
            .cloned()
            .collect();
        true
    }
}
