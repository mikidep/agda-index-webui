use yew::prelude::*;

use crate::components::{search_view::SearchView, doc_view::DocView};
use crate::model::Reference;

pub struct SplitPane {
    frame_cb: Callback<String>,
    frame_url: Option<String>
}    

#[derive(Properties, PartialEq)]
pub struct SplitPaneProps {
    pub haystack: Vec<Reference>
}


impl Component for SplitPane {
    type Message = String;
    type Properties = SplitPaneProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            frame_cb: ctx.link().callback(|url: String| url),
            frame_url: None,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        self.frame_url = Some(msg);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex-container">
                <div id="sidebar">
                    <SearchView haystack={ctx.props().haystack.clone()} frame_cb={self.frame_cb.clone()} />
                </div>
                <div id="content">
                    <DocView url={ self.frame_url.clone() } />
                </div>
            </div>        
        }
    }
}