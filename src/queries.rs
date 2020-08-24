//! Queries that are applied to the game state to resolve current state.

use std::mem::swap;

use hecs::{Entity, World};

use crate::components::{Creature, Permanent};
use crate::pt::{AdjustPtEffect, PtValue, SetPtEffect, SwitchPtEffect};

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
        let mut query = world.query_one::<(&Permanent, &Creature)>(self.0).ok()?;
        let (_permament, creature) = query.get()?;

        // Layer 7a: characteristic-defining P/T.
        let mut calculated_pt = creature.pt.resolve();

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
        //
        // TODO

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
