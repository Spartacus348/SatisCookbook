use std::any::Any;
use crate::route_finder::ProductionNode;

mod objects;
mod recipebook;
mod tiers;
mod route_finder;

enum Settings{
    DisplayAll,
    DisplaySize,
    MinPower
}

fn main() {
    let setting = Settings::MinPower;
    let target = objects::Part::Conveyor(
        objects::Conveyable::ReinforcedIronPlate
    );

    let results = route_finder::generate_possibilities(target, 1);

    match setting {
        Settings::DisplayAll => {delve(&results, 0);report_size(&results,true);},
        Settings::DisplaySize => {report_size(&results,true);},
        Settings::MinPower => {
            let path = route_finder::walk_one_path(results, route_finder::OptimizationMode::MinimizePower);
            println!("{:?}",path);
            println!("Total Power: {}", path.get_power() );},
    }

}

fn delve(layer: &Vec<ProductionNode>, depth: usize) {
    let delimiter = "#";
    let prefix = String::from(delimiter.repeat(depth));
    if layer.len() > 0 {
        for timeline in layer {
            println!("{}Amount: {}", prefix, timeline.amount);
            println!("{}{}", prefix, timeline.building);
            //println!("{}Possible sources:", prefix);
            for (part, options) in &timeline.sources {
                println!("{}{}", prefix, part);
                delve(&options, depth + 1);
            }
        }
    }
}

#[derive(Debug)]
struct SizeReport {
    breadth: usize,
    depth: usize,
}

impl SizeReport {
    fn new(below: Vec<SizeReport>) -> Self{
        Self{
            breadth: below.iter()
                .map(|r| r.breadth)
                .sum(),
            depth: 1 + below.iter()
                .map(|r| r.depth)
                .max()
                .unwrap_or(0)
        }
    }
}

impl Default for SizeReport {
    fn default()->Self{
        Self{breadth: 1, depth: 0}
    }
}

fn report_size(results: &Vec<ProductionNode>, express: bool) ->SizeReport {
    if express {println!("Beginning collection");}
    let result = SizeReport::new(results.iter()
        .map(|node| {
            let results = node.sources.iter()
                .map(|(_, &ref options)|
                    match options.len(){
                        0 => SizeReport::default(),
                        _ => report_size(options, false)
                    }
                )
                .collect::<Vec<SizeReport>>();
            let mut s = SizeReport::new(results);
            s.depth -= 1;
            s
        })
        .collect::<Vec<SizeReport>>());
    if express {
        println!("End collection. Results: {:?}", result);
    }
    result
}
