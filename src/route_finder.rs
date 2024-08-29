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

pub(crate) fn build_tree(starting_part: Part) -> Multiverse<ProductionNode> {
    match starting_part {
        Part::Pump(..) | Part::Mine(..) => Vec::<ProductionNode>::new(),
        part =>{
            recipebook::RECIPES.iter()
                .filter(|recipe| {
                    recipe.building
                        .get_output().iter()
                        .any(|&ingredient| ingredient == part)})
                .map(|recipe| {
                    ProductionNode{
                        amount: 0.0,
                        building: recipe.building,
                        sources: recipe.building
                            .get_input().iter()
                            .map(|&part| (part, build_tree(part)))
                            .collect::<BookOfPaths>(),
                    }
                })
                .collect::<Vec<ProductionNode>>()
        }
    }
}
