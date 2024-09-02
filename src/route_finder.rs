// contains the tools to solve production chains to given parts

use std::collections::HashMap;
use crate::{objects::{Part,Building},
            recipebook};

type Multiverse<T> = Vec<T>;
type BookOfPaths = HashMap<Part, Multiverse<ProductionNode>>;

#[derive(Debug)]
pub(crate) struct ProductionNode {
    pub(crate)amount:f32,
    pub(crate)building:Building,
    pub(crate)sources: BookOfPaths
}

pub(crate) fn generate_possibilities(ingredient: Part, amount: usize) -> Multiverse<ProductionNode> {
    println!("Called for {} of {:?}",amount, ingredient);
    recipebook::RECIPES.iter()
        .filter(|recipe| {
            recipe.building
                .get_output().iter()
                .any(|&output| {
                    output.0 == ingredient
                })})
        .map(|recipe| {
            ProductionNode{
                amount: ((amount as f32) / (recipe.building
                    .get_output().iter()
                    .find(|(part, ..)| *part == ingredient).unwrap().1) as f32),
                building: recipe.building,
                sources: recipe.building
                    .get_input().iter()
                    .map(|&(part, volume)| (part, generate_possibilities(part, volume)))
                    .collect::<BookOfPaths>()
            }
        })
        .collect::<Vec<ProductionNode>>()
}
