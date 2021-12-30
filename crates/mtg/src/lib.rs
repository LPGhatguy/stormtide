use mtg_engine::{
    action::Action,
    components::{Object, Player},
    game::Game,
    hecs::Entity,
    ident::Ident,
    zone::ZoneId,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "startLogging")]
pub fn start_logging() {
    wasm_logger::init(wasm_logger::Config::default());
}

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

    #[wasm_bindgen(js_name = "doAction")]
    pub fn do_action(&mut self, player: JsValue, action: JsValue) {
        log::info!("do_action {:?} {:?}", player, action);
        let player: Entity = player.into_serde().unwrap();
        let action: Action = action.into_serde().unwrap();

        self.inner.do_action(player, action);
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
                    entity,
                    name: player.name.to_owned(),
                    life: player.life as i32,
                })
            })
            .collect::<Vec<_>>();

        JsValue::from_serde(&players).unwrap()
    }

    #[wasm_bindgen(js_name = "objectsInZone")]
    pub fn objects_in_zone(&self, zone: &JsValue) -> JsValue {
        let zone_id: ZoneId = zone.into_serde().unwrap();

        let entities = self.inner.zone(zone_id).unwrap().members();
        let output = entities
            .into_iter()
            .filter_map(|&entity| {
                let entity = self.inner.world().entity(entity).ok()?;
                let object = entity.get::<Object>()?;

                let js_object = JsObject {
                    name: object.name.clone(),
                    zone: object.zone,
                    owner: object.owner,
                    controller: object.controller,
                };

                Some(js_object)
            })
            .collect::<Vec<_>>();

        JsValue::from_serde(&output).unwrap()
    }

    pub fn step(&self) -> JsValue {
        JsValue::from_serde(&self.inner.step()).unwrap()
    }

    pub fn state(&self) -> JsValue {
        JsValue::from_serde(self.inner.state()).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsPlayer {
    pub entity: Entity,
    pub name: String,
    pub life: i32,
}

#[derive(Serialize, Deserialize)]
pub struct JsObject {
    pub name: Ident,
    pub zone: ZoneId,
    pub owner: Entity,
    pub controller: Option<Entity>,
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
