use crate::route_finder::ProductionNode;

mod objects;
mod recipebook;
mod tiers;
mod route_finder;

fn main() {
    let target = objects::Part::Conveyor(
        objects::Conveyable::ReinforcedIronPlate
    );

    let results = route_finder::generate_possibilities(target, 1);
    println!("Displaying results");

    fn delve(layer: Vec<ProductionNode>, depth: usize) {
        let prefix = String::from_iter((0..depth).map(|_| "\t"));
        if layer.len() > 0 {
            println!("{}Begin sources:", prefix);
            for timeline in layer {
                println!("{}Amount: {}", prefix, timeline.amount);
                println!("{}Building: {:?}", prefix, timeline.building);
                println!("{}Possible sources:", prefix);
                for (part, options) in timeline.sources {
                    println!("{}\t{:?}", prefix, part);
                    delve(options, depth + 1);
                }
                println!();
            }
        }
    }

    delve(results, 0);
}
