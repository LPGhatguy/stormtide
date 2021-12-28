#[rustfmt::skip::macros(use_style)]
mod components;

use mtg_engine::{
    components::{Creature, Land, Object, Permanent, Player},
    game::{Game, ZoneId},
    hecs::Entity,
    ident::Ident,
    object_db::ObjectDb,
    pt::{PtCharacteristic, PtValue},
};

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
        let player1 = self.game.players()[0];
        let player2 = self.game.players()[1];

        let players: Vec<_> = [
            player_props(&self.game, player1),
            player_props(&self.game, player2),
        ]
        .into_iter()
        .flatten()
        .collect();

        yew::html! {
            <components::Layout players={ players } />
        }
    }
}

fn player_props(game: &Game, player_id: Entity) -> Option<components::PlayerProps> {
    let db = ObjectDb::load();
    let bear_image = db
        .card_by_name("Grizzly Bears")
        .unwrap()
        .image
        .clone()
        .unwrap();

    let forest_image = db.card_by_name("Forest").unwrap().image.clone().unwrap();

    let turn_order = game
        .players()
        .iter()
        .position(|p| *p == player_id)
        .unwrap_or(0);
    let entity = game.world.entity(player_id).ok()?;
    let player = entity.get::<Player>()?;

    let hand = game
        .zone(ZoneId::Hand(player_id))
        .map(|zone| {
            zone.members()
                .iter()
                .map(|_| forest_image.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(Vec::new);

    let battlefield = game
        .zone(ZoneId::Battlefield)
        .map(|zone| {
            zone.members()
                .iter()
                .filter_map(|entity| {
                    let entity = game.world.entity(*entity).ok()?;
                    let object = entity.get::<Object>()?;

                    if object.controller == Some(player_id) {
                        Some(bear_image.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(Vec::new);

    Some(components::PlayerProps {
        top: turn_order == 0,
        name: player.name.clone(),
        life: player.life,
        hand,
        battlefield,
    })
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
