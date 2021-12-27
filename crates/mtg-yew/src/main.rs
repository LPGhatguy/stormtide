#[rustfmt::skip::macros(use_style)]
mod components;

use mtg_engine::{
    components::{Creature, Land, Object, Permanent},
    game::{Game, ZoneId},
    ident::Ident,
    pt::{PtCharacteristic, PtValue},
};

pub static GIANT_GROWTH: &str = "https://c1.scryfall.com/file/scryfall-cards/large/front/6/b/6b712e6e-eb48-4a71-b95d-ce343966b236.jpg";
pub static GRIZZLY_BEARS: &str = "https://c1.scryfall.com/file/scryfall-cards/large/front/4/0/409f9b88-f03e-40b6-9883-68c14c37c0de.jpg";

struct App {
    game: Game,
}

impl yew::Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            game: sample_game(),
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        let players = vec![
            components::PlayerProps {
                top: true,
                name: "lpg".to_owned(),
                life: 20,
                hand: vec![(); 3],
                battlefield: vec![(); 2],
            },
            components::PlayerProps {
                top: false,
                name: "eryn".to_owned(), // ;)
                life: 20,
                hand: vec![(); 7],
                battlefield: vec![(); 4],
            },
        ];

        yew::html! {
            <components::Layout players={ players } />
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &yew::Context<Self>) {}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("Hello, world!");
    yew::start_app::<App>();
}

fn sample_game() -> Game {
    let mut game = Game::new();

    let player1 = game.players()[0];

    let _forest1 = game.world.spawn((
        Object {
            name: Ident::new("Forest"),
            zone: ZoneId::Battlefield,
            owner: player1,
            controller: Some(player1),
        },
        Land,
        Permanent { tapped: false },
    ));
    let _forest2 = game.world.spawn((
        Object {
            name: Ident::new("Forest"),
            zone: ZoneId::Battlefield,
            owner: player1,
            controller: Some(player1),
        },
        Land,
        Permanent { tapped: false },
    ));
    let _bear = game.world.spawn((
        Object {
            name: Ident::new("Grizzly Bears"),
            zone: ZoneId::Battlefield,
            owner: player1,
            controller: Some(player1),
        },
        Creature {
            pt: PtCharacteristic::Normal(PtValue {
                power: 2,
                toughness: 2,
            }),
        },
        Permanent { tapped: false },
    ));

    game.HACK_rebuild_zone_index();
    game
}
