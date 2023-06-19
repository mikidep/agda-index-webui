use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DocViewProps {
    pub url: Option<String>,
}

#[function_component]
pub fn DocView(props: &DocViewProps) -> Html {
    if let DocViewProps { url: Some(url) } = props {
        html! {
            <iframe style="width: 100%; height: 100%;" src={ url.clone() }/>
        }
    } else {
        html! {
            <div></div>
        }
    }
}
