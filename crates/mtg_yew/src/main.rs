mod components;
mod symbols;

use mtg_engine::{
    components::{Object, Player},
    game::Game,
    hecs::Entity,
    object_db::ObjectDb,
    zone::ZoneId,
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
    let entity = game.world().entity(player_id).ok()?;
    let player = entity.get::<Player>()?;

    let image = if turn_order == 0 {
        crate::symbols::PLAYER1
    } else {
        crate::symbols::PLAYER2
    };

    let hand = game
        .zone(ZoneId::Hand(player_id))
        .map(|zone| {
            zone.members()
                .iter()
                .map(|_| forest_image.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(Vec::new);

    let library_count = game
        .zone(ZoneId::Library(player_id))
        .map(|zone| zone.members().len() as u64)
        .unwrap_or(0);

    let battlefield = game
        .zone(ZoneId::Battlefield)
        .map(|zone| {
            zone.members()
                .iter()
                .filter_map(|entity| {
                    let entity = game.world().entity(*entity).ok()?;
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
        image: image.to_owned(),
        life: player.life,
        library_count,
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

    let forest = game.object_db().card_id("Forest").unwrap();
    let bear = game.object_db().card_id("Grizzly Bears").unwrap();

    let players = game.players().to_vec();

    for player in players {
        // each player gets a nice 40 card deck
        for _ in 0..15 {
            game.create_card(forest, ZoneId::Library(player), player)
                .unwrap();
        }

        for _ in 0..25 {
            game.create_card(bear, ZoneId::Library(player), player)
                .unwrap();
        }

        game.zone_mut(ZoneId::Library(player)).unwrap().shuffle();
    }

    game
}
