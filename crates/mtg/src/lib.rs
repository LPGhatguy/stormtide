use mtg_engine::{components::Player, game::Game, zone::ZoneId};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "Game")]
pub struct JsGame {
    inner: Game,
}

#[wasm_bindgen(js_class = "Game")]
impl JsGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: sample_game(),
        }
    }

    #[wasm_bindgen]
    pub fn players(&self) -> JsValue {
        let players = self
            .inner
            .players()
            .to_vec()
            .into_iter()
            .filter_map(|entity| {
                let player = self.inner.world().get::<Player>(entity).ok()?;

                Some(JsPlayer {
                    name: player.name.to_owned(),
                    life: player.life as i32,
                })
            })
            .collect::<Vec<_>>();

        JsValue::from_serde(&players).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsPlayer {
    pub name: String,
    pub life: i32,
}

pub fn sample_game() -> Game {
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
