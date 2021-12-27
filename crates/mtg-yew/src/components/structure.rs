use stylist::yew::use_style;
use yew::prelude::*;

use super::Card;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PlayerProps {
    pub top: bool,
    pub name: String,
    pub life: i64,
    pub hand: Vec<()>,
    pub battlefield: Vec<()>,
}

#[function_component(Player)]
#[rustfmt::skip::macros(use_style)]
pub fn player(props: &PlayerProps) -> Html {
    let outer = use_style!("
        display: flex;
        flex-grow: 1;
        width: 100%;
        height: 100%;
        border: 8px solid black;
    ");

    let left = use_style!("
        display: flex;
        flex: 0 0 10rem;
        flex-direction: column;
        background-color: #999;
    ");

    let portrait = use_style!("
        display: inline-block;
        background-color: #e9e;
        aspect-ratio: 1 / 1;
        margin: 1rem 2rem;
    ");

    let name = use_style!("
        font-size: 1.2rem;
        font-weight: bold;
        text-align: center;
        margin-bottom: 1rem;
    ");

    let separator = use_style!("
        height: 1px;
        background-color: black;
    ");

    let life_total = use_style!("
        font-size: 1.4rem;
        text-align: center;
    ");

    let zone_direction = if props.top {
        "column-reverse"
    } else {
        "column"
    };

    let main = use_style!("
        display: flex;
        flex: 1 0 40rem;
        flex-direction: ${ direction };
        background-color: #eee;
    ", direction = zone_direction);

    let battlefield = use_style!("
        display: flex;
        flex-direction: ${ direction };
        flex: 1 0 10rem;
        background-color: #333;
    ", direction = zone_direction);

    let battlefield_row = use_style!("
        display: flex;
        flex: 1 1 4rem;
        margin: 0.3rem;
        gap: 0.3rem;
    ");

    let hand = use_style!("
        display: flex;
        background-color: #e99;
        flex: 0 0 12rem;
        padding: 0.3rem;
        gap: 0.3rem;
    ");

    html! {
        <div class={ outer }>
            <div class={ left }>
                <div class={ portrait }></div>
                <div class={ name }>{ props.name.to_string() }</div>
                <div class={ separator } />
                <div class={ life_total }>{ props.life }</div>
            </div>
            <div class={ main }>
                <div class={ battlefield }>
                    <div class={ battlefield_row.clone() }>
                        { props.battlefield.iter().map(|_| {
                            html! {
                                <Card image={ crate::GRIZZLY_BEARS } />
                            }
                        }).collect::<Html>() }
                    </div>
                    <div class={ battlefield_row }>
                    </div>
                </div>
                <div class={ hand }>
                    { props.hand.iter().map(|_| html! {
                            <Card image={ crate::GIANT_GROWTH } />
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
