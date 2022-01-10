//! Queries that are applied to the game state to resolve current state.

use std::mem::swap;

use hecs::{Entity, World};

use crate::components::{Counters, Object, Permanent};
use crate::counters::Counter;
use crate::pt::{AdjustPtEffect, PtValue, SetPtEffect, SwitchPtEffect};
use crate::types::CardType;

/// Trait implemented on types to read information from the game state.
pub trait Query {
    type Output;

    fn query(&self, world: &World) -> Self::Output;
}

/// Query the power and toughness of a given entity, returning `None` if the
/// entity is not a creature permanent.
pub struct QueryPt(pub Entity);

impl Query for QueryPt {
    type Output = Option<PtValue>;

    fn query(&self, world: &World) -> Self::Output {
        let entity = world.entity(self.0).ok()?;

        if !entity.has::<Permanent>() {
            return None;
        }

        let object = entity.get::<Object>()?;
        let counters = entity.get::<Counters>();

        // Layer 7a: characteristic-defining P/T.
        let mut calculated_pt = object.pt?.resolve();

        // Layer 7b: any effects that directly set power/toughness.
        //
        // TODO: Sort by timestamp.
        let mut layer_7b_query = world.query::<(&SetPtEffect,)>();
        for (_entity, (effect,)) in layer_7b_query.iter() {
            if effect.target == self.0 {
                calculated_pt = effect.value;
            }
        }

        // Layer 7c: any effects that adjust power/toughness without setting it.
        //
        // TODO: Sort by timestamp.
        let mut layer_7c_query = world.query::<(&AdjustPtEffect,)>();
        for (_entity, (effect,)) in layer_7c_query.iter() {
            if effect.target == self.0 {
                calculated_pt.power += effect.adjustment.power;
                calculated_pt.toughness += effect.adjustment.toughness;
            }
        }

        // Layer 7d: power/toughness adjustments from counters
        if let Some(counters) = counters {
            for counter in &counters.counters {
                if let Counter::Pt(adjustment) = counter {
                    calculated_pt.power += adjustment.power;
                    calculated_pt.toughness += adjustment.toughness;
                }
            }
        }

        // Layer 7e: power/toughness switching
        let mut layer_7e_query = world.query::<(&SwitchPtEffect,)>();
        for (_entity, (effect,)) in layer_7e_query.iter() {
            if effect.target == self.0 {
                swap(&mut calculated_pt.power, &mut calculated_pt.toughness);
            }
        }

        Some(calculated_pt)
    }
}

pub struct QueryMaxHandSize(pub Entity);

impl Query for QueryMaxHandSize {
    type Output = i64;

    fn query(&self, _world: &World) -> Self::Output {
        // TODO: Look for affects that alter maximum hand size.
        7
    }
}

#[derive(Debug)]
pub struct QueryCreatures;

impl Query for QueryCreatures {
    type Output = Vec<Entity>;

    fn query(&self, world: &World) -> Self::Output {
        let mut query = world.query::<(&Object,)>().with::<Permanent>();
        let mut output = Vec::new();

        for (entity, (object,)) in query.into_iter() {
            if object.types.contains(&CardType::Creature) {
                output.push(entity);
            }
        }

        output
    }
}
