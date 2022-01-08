mod ffi;

use mtg_engine::{
    action::PlayerAction,
    components::{Card, Object, Permanent, Player},
    game::Game,
    hecs::Entity,
    ident::Ident,
    mana_pool::ManaPool,
    object_db::{CardId, ObjectDb},
    pt::PtCharacteristic,
    types::{CardSubtype, CardSupertype, CardType},
    zone::ZoneId,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "engineInit")]
pub fn engine_init() {
    console_error_panic_hook::set_once();
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

    #[wasm_bindgen(js_name = "objectDb")]
    pub fn object_db(&self) -> JsObjectDb {
        JsObjectDb {
            inner: self.inner.object_db().clone(),
        }
    }

    #[wasm_bindgen(js_name = "doAction")]
    pub fn do_action(&mut self, player: JsValue, action: JsValue) -> Result<(), JsValue> {
        let player: Entity = ffi::from_js(player)?;
        let action: PlayerAction = ffi::from_js(action)?;

        self.inner.do_action(player, action);
        Ok(())
    }

    pub fn players(&self) -> Result<JsValue, JsValue> {
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
                    lands_played_this_turn: player.lands_played_this_turn,
                    mana_pool: player.mana_pool.clone(),
                })
            })
            .collect::<Vec<_>>();

        ffi::to_js(&players)
    }

    #[wasm_bindgen(js_name = "objectsInZone")]
    pub fn objects_in_zone(&self, zone: JsValue) -> Result<JsValue, JsValue> {
        let zone_id: ZoneId = ffi::from_js(zone)?;

        let entities = self.inner.zone(zone_id).unwrap().members();
        let output = entities
            .into_iter()
            .filter_map(|&entity| {
                let entity_ref = self.inner.world().entity(entity).ok()?;
                let object = entity_ref.get::<Object>()?;

                let js_object = JsObject {
                    entity,
                    name: object.name.clone(),
                    types: object.types.clone(),
                    supertypes: object.supertypes.clone(),
                    subtypes: object.subtypes.clone(),
                    pt: object.pt,
                    zone: object.zone,
                    owner: object.owner,
                    controller: object.controller,

                    card: entity_ref.get::<Card>().as_deref().cloned(),
                    permanent: entity_ref.get::<Permanent>().as_deref().cloned(),
                };

                Some(js_object)
            })
            .collect::<Vec<_>>();

        ffi::to_js(&output)
    }

    pub fn step(&self) -> Result<JsValue, JsValue> {
        ffi::to_js(&self.inner.step())
    }

    pub fn state(&self) -> Result<JsValue, JsValue> {
        ffi::to_js(self.inner.state())
    }
}

#[wasm_bindgen(js_name = "ObjectDb")]
pub struct JsObjectDb {
    inner: ObjectDb,
}

#[wasm_bindgen(js_class = "ObjectDb")]
impl JsObjectDb {
    #[wasm_bindgen(js_name = "cardId")]
    pub fn card_id(&self, name: &str) -> Option<u32> {
        self.inner.card_id(name).map(|id| id.0)
    }

    pub fn card(&self, id: u32) -> Result<JsValue, JsValue> {
        ffi::to_js(&self.inner.card(CardId(id)))
    }

    #[wasm_bindgen(js_name = "cardByName")]
    pub fn card_by_name(&self, name: &str) -> Result<JsValue, JsValue> {
        ffi::to_js(&self.inner.card_by_name(name))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsPlayer {
    pub entity: Entity,
    pub name: String,
    pub life: i32,
    pub lands_played_this_turn: u32,
    pub mana_pool: ManaPool,
}

#[derive(Serialize, Deserialize)]
pub struct JsObject {
    pub entity: Entity,

    pub name: Ident,
    pub types: Vec<CardType>,
    pub supertypes: Vec<CardSupertype>,
    pub subtypes: Vec<CardSubtype>,
    pub pt: Option<PtCharacteristic>,
    pub zone: ZoneId,
    pub owner: Entity,
    pub controller: Option<Entity>,

    pub card: Option<Card>,
    pub permanent: Option<Permanent>,
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

        for _ in 0..7 {
            let card = {
                let members = game.zone(ZoneId::Library(player)).unwrap().members();

                members[members.len() - 1]
            };
            game.move_object_to_zone(card, ZoneId::Hand(player));
        }
    }

    game
}
