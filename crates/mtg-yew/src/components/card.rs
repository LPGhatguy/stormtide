use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct CardProps {
    pub image: String,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let root = use_style!(
        "
        display: inline-block;
        aspect-ratio: 5 / 7;
        height: 100%;
        "
    );

    html! {
        <div class={ root }>
            <img src={ props.image.clone() } />
        </div>
    }
}
