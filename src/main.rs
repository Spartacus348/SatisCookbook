use crate::route_finder::ProductionNode;

mod objects;
mod recipebook;
mod tiers;
mod route_finder;

enum Settings{
    DisplayAll,
    DisplaySize
}

fn main() {
    let setting = Settings::DisplayAll;
    let target = objects::Part::Conveyor(
        objects::Conveyable::FePlate
    );

    let results = route_finder::generate_possibilities(target, 1);

    match setting {
        Settings::DisplayAll => {delve(results, 0);},
        Settings::DisplaySize => {report_size(&results,true);}
    }

}

fn delve(layer: Vec<ProductionNode>, depth: usize) {
    let prefix = String::from("\t".repeat(depth));
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
        Self{breadth: 1, depth: 1}
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
            SizeReport::new(results)
        })
        .collect::<Vec<SizeReport>>());
    if express {
        println!("End collection. Results: {:?}", result);
    }
    result
}
