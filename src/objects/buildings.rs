use crate::objects::ingredients::Conveyable;

trait Building {

    // returns the power consumption of the building
    fn get_power(&self) -> isize;

    // returns the inputs into the building
    fn get_inputs(&self) -> Vec<Conveyable>;

    // returns the outputs out of the building
    fn get_outputs(&self) -> Vec<Conveyable>;

    // returns the period of the assigned construction
    fn get_time(&self) -> usize;
}

struct Miner{
    inputs: [Conveyable;0],
    outputs: [Conveyable;1],
    time: usize
}
impl Building for Miner{
    fn get_power(&self) -> isize {
        5
    }

    fn get_inputs(&self) -> Vec<Conveyable> {
        Vec::from(&self.inputs)
    }

    fn get_outputs(&self) -> Vec<Conveyable> {
        Vec::from(&self.outputs)
    }

    fn get_time(&self) -> usize {
        self.time
    }
}

struct Constructor{
    inputs:[Conveyable;1],
    outputs:[Conveyable;1],
    time:usize
}

impl Building for Constructor{
    fn get_power(&self) -> isize {
        15
    }

    fn get_inputs(&self) -> Vec<Conveyable> {
        Vec::from(&self.inputs)
    }

    fn get_outputs(&self) -> Vec<Conveyable> {
        todo!()
    }

    fn get_time(&self) -> usize {
        todo!()
    }
}
