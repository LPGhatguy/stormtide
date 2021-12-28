use stylist::yew::use_style;
use yew::prelude::*;

use super::Card;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PlayerProps {
    pub top: bool,
    pub name: String,
    pub image: String,
    pub life: i64,
    pub library_count: u64,
    pub hand: Vec<String>,
    pub battlefield: Vec<String>,
}

#[function_component(Player)]
#[rustfmt::skip::macros(use_style)]
pub fn player(props: &PlayerProps) -> Html {
    let outer = use_style!("
        display: flex;
        flex-grow: 1;
        background-color: #242526;
        width: 100%;
        height: 100%;
    ");

    let left = use_style!("
        display: flex;
        flex: 0 0 10rem;
        flex-direction: column;
        background-color: #0f0f10;
        color: #fefefe;
    ");

    let identity = use_style!("
        display: flex;
        align-items: center;
        gap: 1rem;
        margin: 1rem;
    ");

    let portrait = use_style!("
        display: flex;
        flex: 0 0 2rem;
        aspect-ratio: 1 / 1;

        img {
            width: 100%;
            height: 100%;
        }
    ");

    let name = use_style!("
        flex: 1 1;
        font-size: 1.2rem;
        font-weight: bold;
        text-align: center;
    ");

    let separator = use_style!("
        height: 1px;
        background-color: #fefefe;
    ");

    let life_total = use_style!("
        font-size: 1.4rem;
        text-align: center;
    ");

    let library = use_style!("
        margin: 1rem;
        aspect-ratio: 5 / 7;
        background-image: url(${ card_back });
        background-size: contain;

        display: flex;
        justify-content: center;
        align-items: center;
        font-weight: bold;
        -webkit-text-stroke: 2px black;
        color: white;
        font-size: 2rem;
    ", card_back = crate::symbols::CARD_BACK);

    let zone_direction = if props.top {
        "column-reverse"
    } else {
        "column"
    };

    let main = use_style!("
        display: flex;
        flex: 1 0 40rem;
        flex-direction: ${ direction };
    ", direction = zone_direction);

    let battlefield = use_style!("
        display: flex;
        flex-direction: ${ direction };
        flex: 1 0 10rem;
    ", direction = zone_direction);

    let battlefield_row = use_style!("
        display: flex;
        flex: 1 1 4rem;
        margin: 0.3rem;
        gap: 0.3rem;
    ");

    let hand = use_style!("
        display: flex;
        background-color: #18191a;
        flex: 0 0 9rem;
        padding: 0.3rem;
        gap: 0.3rem;
    ");

    html! {
        <div class={ outer }>
            <div class={ left }>
                <div class={ identity }>
                    <div class={ portrait }>
                        <img src={ props.image.clone() } />
                    </div>
                    <div class={ name }>{ props.name.to_string() }</div>
                </div>
                <div class={ separator } />
                <div class={ life_total }>{ props.life }</div>
                <div class={ library }>{ props.library_count }</div>
            </div>
            <div class={ main }>
                <div class={ battlefield }>
                    <div class={ battlefield_row.clone() }>
                        { props.battlefield.iter().map(|image| {
                            html! {
                                <Card image={ image.clone() } />
                            }
                        }).collect::<Html>() }
                    </div>
                    <div class={ battlefield_row }>
                    </div>
                </div>
                <div class={ hand }>
                    { props.hand.iter().map(|image| html! {
                            <Card image={ image.clone() } />
                    }).collect::<Html>() }
                </div>
            </div>
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct LayoutProps {
    pub players: Vec<PlayerProps>,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    #[rustfmt::skip]
    let outer = use_style!("
        display: flex;
        flex-direction: column;
        gap: 20px;
        background-color: black;
        flex-grow: 1;
        width: 100%;
        height: 100%;
    ");

    html! {
        <div class={ outer }>
            { props.players.iter().map(|player| html! {
                <Player ..player.clone() />
            }).collect::<Html>() }
        </div>
    }
}
