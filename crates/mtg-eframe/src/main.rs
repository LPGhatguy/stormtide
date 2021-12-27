use eframe::{egui, epi};
use egui::Ui;
use mtg_engine::components::{Creature, Land, Object, Permanent, Player};
use mtg_engine::game::{Game, ZoneId};
use mtg_engine::hecs::{Entity, World};
use mtg_engine::ident::Ident;
use mtg_engine::pt::{PtCharacteristic, PtValue};

struct App {
    game: Game,
}

impl epi::App for App {
    fn name(&self) -> &str {
        "MtG"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let player1 = self.game.players()[0];
        let player2 = self.game.players()[1];

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                widget_zone(ui, &self.game, ZoneId::Hand(player1));
                widget_zone(ui, &self.game, ZoneId::Battlefield);
                widget_zone(ui, &self.game, ZoneId::Hand(player2));
            });
        });
    }
}

fn player_name(world: &World, player: Entity) -> String {
    world
        .get::<Player>(player)
        .map(|player| player.name.clone())
        .unwrap_or("Unknown Player".to_owned())
}

fn zone_label(world: &World, zone: ZoneId) -> String {
    match zone {
        ZoneId::Library(owner) => {
            format!("{}'s Library", player_name(world, owner))
        }
        ZoneId::Hand(owner) => {
            format!("{}'s Hand", player_name(world, owner))
        }
        ZoneId::Graveyard(owner) => {
            format!("{}'s Hand", player_name(world, owner))
        }
        _ => format!("{:?}", zone),
    }
}

fn widget_zone(ui: &mut Ui, game: &Game, zone_id: ZoneId) {
    ui.vertical(|ui| {
        let zone = match game.zone(zone_id) {
            Some(z) => z,
            None => return,
        };

        ui.heading(zone_label(&game.world, zone_id));
        ui.horizontal(|ui| {
            for &member in zone.members() {
                let entity = match game.world.entity(member) {
                    Ok(o) => o,
                    Err(_) => continue,
                };

                egui::Frame::none()
                    .margin((5.0, 5.0))
                    .stroke(egui::Stroke::new(2.0, egui::Rgba::from_gray(0.3)))
                    .show(ui, |ui| {
                        if let Some(object) = entity.get::<Object>() {
                            ui.label(&object.name);
                        } else {
                            ui.label("(Not an Object)");
                        }
                    });
            }
        });
    });
}

fn main() {
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

    let app = App { game };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
