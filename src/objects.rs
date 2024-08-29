use crate::tiers::*;

#[derive(Clone,Copy,PartialEq,Debug)]
pub(crate) struct Process<'a>{
    pub(crate)name: &'a str,
    pub(crate)time: usize,
    pub(crate)building: Building,
    pub(crate)tier: Tier,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Building{
    Smelter       {input:(Conveyable,),             output:(Conveyable,)},
    Foundry       {input:(Conveyable,Conveyable),   output:(Conveyable,)},
    Constructor   {input:(Conveyable,),             output:(Conveyable,)},
    Assembler     {input:(Conveyable,Conveyable),   output:(Conveyable,)},
    Manufacturer  {input:(Conveyable,Conveyable,Conveyable,Option<Conveyable>),             output:(Conveyable,)},
    Refinery      {input:(Option<Conveyable>,Option<Pipeable>), output:(Option<Conveyable>,Option<Pipeable>)},
    Blender       {input:(Option<Conveyable>,Option<Conveyable>,Pipeable,Option<Pipeable>), output:(Option<Conveyable>,Option<Pipeable>)},
    Packager      {input:(Conveyable,Option<Pipeable>),         output:(Conveyable,Option<Pipeable>)},
    BioPlant      {input:(Part,),                   output:()},
    CoalPlant     {input:(Conveyable, Pipeable),    output:()},
    OilPlant      {input:(Pipeable,),               output:()},
    NuclearPlant  {input:(Conveyable, Pipeable),    output:(Conveyable,)},
    Miner1        {input:(Mineable,),               output:(Conveyable,),},
    Miner2        {input:(Mineable,),               output:(Conveyable,),},
    Miner3        {input:(Mineable,),               output:(Conveyable,),},
    WaterExtractor{input:(Pumpable,),               output:(Pipeable,),},
    OilExtractor  {input:(Pumpable,),               output:(Pipeable,),},
}

impl Building {
    pub(crate) fn get_power(self: &Self) -> isize {
        match self {
            Building::Smelter { .. } => 4,
            Building::Foundry { .. } => 16,
            Building::Constructor { .. } => 4,
            Building::Assembler { .. } => 15,
            Building::Manufacturer { .. } => 55,
            Building::Refinery { .. } => 30,
            Building::Blender { .. } => 75,
            Building::Packager { .. } => 10,
            Building::BioPlant { .. } => -25,
            Building::CoalPlant { .. } => -75,
            Building::OilPlant { .. } => -150,
            Building::NuclearPlant { .. } => -2500,
            Building::Miner1 { .. } => 5,
            Building::Miner2 { .. } => 12,
            Building::Miner3 { .. } => 30,
            Building::WaterExtractor {..} => 20,
            Building::OilExtractor {..} => 40
        }
    }

    pub(crate) fn get_input(self: &Self) -> Vec<Part> {
        match self{
            Building::Smelter       {input:(x,                          ), .. } => Vec::from([Part::Conveyor(*x)]),
            Building::Foundry       {input:(a,      b                   ), .. } => Vec::from([Part::Conveyor(*a),Part::Conveyor(*b)]),
            Building::Constructor   {input:(a,                          ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Assembler     {input:(a,      b                   ), .. } => Vec::from([Part::Conveyor(*a),Part::Conveyor(*b)]),
            Building::Manufacturer  {input:(a,      b,      c,Some(d)   ), .. } => Vec::from([Part::Conveyor(*a),Part::Conveyor(*b),Part::Conveyor(*c),Part::Conveyor(*d)]),
            Building::Manufacturer  {input:(a,      b,      c,None      ), .. } => Vec::from([Part::Conveyor(*a),Part::Conveyor(*b),Part::Conveyor(*c)]),
            Building::Refinery      {input:(None,   None                ), .. } => Vec::<Part>::new(),
            Building::Refinery      {input:(Some(a),Some(b)             ), .. } => Vec::from([Part::Conveyor(*a),Part::Pipe(*b)]),
            Building::Refinery      {input:(Some(a),None                ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Refinery      {input:(None,   Some(b)             ), .. } => Vec::from([Part::Pipe(*b)]),
            Building::Blender       {input:(Some(a),Some(b),c,Some(d)   ), .. } => Vec::from([Part::Conveyor(*a),Part::Conveyor(*b),Part::Pipe(*c),Part::Pipe(*d)]),
            Building::Blender       {input:(Some(a),Some(b),c,None      ), .. } => Vec::from([Part::Conveyor(*a),Part::Conveyor(*b),Part::Pipe(*c)]),
            Building::Blender       {input:(Some(a),None,   c,Some(d)   ), .. } => Vec::from([Part::Conveyor(*a),Part::Pipe(*c),    Part::Pipe(*d)]),
            Building::Blender       {input:(Some(a),None,   c,None      ), .. } => Vec::from([Part::Conveyor(*a),Part::Pipe(*c)]),
            Building::Blender       {input:(None,   Some(b),c,Some(d)   ), .. } => Vec::from([Part::Conveyor(*b),Part::Pipe(*c),    Part::Pipe(*d)]),
            Building::Blender       {input:(None,   Some(b),c,None      ), .. } => Vec::from([Part::Conveyor(*b),Part::Pipe(*c)]),
            Building::Blender       {input:(None,   None,   c,Some(d)   ), .. } => Vec::from([Part::Pipe(*c),    Part::Pipe(*d)]),
            Building::Blender       {input:(None,   None,   c,None      ), .. } => Vec::from([Part::Pipe(*c)]),
            Building::Packager      {input:(a,      None                ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Packager      {input:(a,      Some(b)             ), .. } => Vec::from([Part::Conveyor(*a),Part::Pipe(*b)]),
            Building::BioPlant      {input:(a,                          ), .. } => Vec::from([*a]),
            Building::CoalPlant     {input:(a,      b                   ), .. } => Vec::from([Part::Conveyor(*a),Part::Pipe(*b)]),
            Building::OilPlant      {input:(a,                          ), .. } => Vec::from([Part::Pipe(*a)]),
            Building::NuclearPlant  {input:(a,      b                   ), .. } => Vec::from([Part::Conveyor(*a),Part::Pipe(*b)]),
            Building::Miner1        {input:(a,                          ), .. } => Vec::from([Part::Mine(*a)]),
            Building::Miner2        {input:(a,                          ), .. } => Vec::from([Part::Mine(*a)]),
            Building::Miner3        {input:(a,                          ), .. } => Vec::from([Part::Mine(*a)]),
            Building::WaterExtractor{input:(a,                          ), .. } => Vec::from([Part::Pump(*a)]),
            Building::OilExtractor  {input:(a,                          ), .. } => Vec::from([Part::Pump(*a)])
        }
    }

    pub(crate) fn get_output(self: &Self) -> Vec<Part>{
        match self {
            Building::Smelter       {output:(x,             ), .. } => Vec::from([Part::Conveyor(*x)]),
            Building::Foundry       {output:(a,             ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Constructor   {output:(a,             ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Assembler     {output:(a,             ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Manufacturer  {output:(a,             ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Refinery      {output:(None,   None   ), .. } => Vec::<Part>::new(),
            Building::Refinery      {output:(Some(a),Some(b)), .. } => Vec::from([Part::Conveyor(*a),Part::Pipe(*b)]),
            Building::Refinery      {output:(Some(a),None   ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Refinery      {output:(None,   Some(b)), .. } => Vec::from([Part::Pipe(*b)]),
            Building::Blender       {output:(Some(a),Some(b)), .. } => Vec::from([Part::Conveyor(*a),Part::Pipe(*b)]),
            Building::Blender       {output:(Some(a),None   ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Blender       {output:(None,   Some(b)), .. } => Vec::from([Part::Pipe(*b),]),
            Building::Blender       {output:(None,   None   ), .. } => Vec::<Part>::new(),
            Building::Packager      {output:(a,      None   ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Packager      {output:(a,      Some(b)), .. } => Vec::from([Part::Conveyor(*a),Part::Pipe(*b)]),
            Building::BioPlant      {                          .. } => Vec::<Part>::new(),
            Building::CoalPlant     {                          .. } => Vec::<Part>::new(),
            Building::OilPlant      {                          .. } => Vec::<Part>::new(),
            Building::NuclearPlant  {output:(a,             ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Miner1        {output:(a,             ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Miner2        {output:(a,             ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::Miner3        {output:(a,             ), .. } => Vec::from([Part::Conveyor(*a)]),
            Building::WaterExtractor{output:(a,             ), .. } => Vec::from([Part::Pipe(*a)]),
            Building::OilExtractor  {output:(a,             ), .. } => Vec::from([Part::Pipe(*a)])
        }
    }
}


//enum NodeQuality{Low,Med,High}

#[derive(Eq, Debug, Clone, Copy, PartialEq, Hash)]
pub(crate) enum Part{
    Conveyor(Conveyable),
    Pipe(Pipeable),
    Mine(Mineable),
    Pump(Pumpable)
}

#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Conveyable{
    FeOre (usize),     CuOre(usize),  Limestone(usize),
    Coal(usize),       Sulfur(usize), RawQuartz(usize),
    CateriumOre(usize),UOre(usize),   Bauxite(usize),
    SAM(usize),//raw ores
    PkgdWater(usize),   PkgdOil(usize),  PkgdN(usize),
    PkgdHOil(usize),    PkgdFuel(usize), PkgdTurbofuel(usize),
    PkgdLBiofuel(usize),PkgdNAcid(usize),PkgdAlSol(usize),
    PkgdSAcid(usize),//packaged pipes
    Leaves(usize),      Wood(usize),     Mycelia(usize),
    HatcherProtein(usize),HogProtein(usize),SpitterProtein(usize),
    StingerProtein(usize),//raw biomass
    AlienProtein(usize),AlienDNA(usize), Biomass(usize),
    SolidBiofuel(usize),Fabric(usize),   CompactedCoal(usize),//processed biomatter
    FlowerPetals(usize),ColorCartridge(usize),//color stuff
    FicsmasGift(usize), ActualSnow(usize),CandyCane(usize),
    FicsmasBow(usize),  FicsmasTree(usize),Snowball(usize),//constructor fiscmas
    FancyFireworks(usize),SparklyFireworks(usize),SweetFireworks(usize),//ficsmas fireworks
    FeOrnament (usize),CuOrnament (usize),RedOrnament(usize),
    BlueOrnament(usize),FicsmasBranch(usize),OrnamentBundle(usize),
    FicsmasDecoration(usize),FicsmasStar(usize),//assembler fixmas
    FeIngot (usize),   FeRod (usize),    FePlate (usize),
    Screws(usize),//constructor iron
    CuIngot (usize), CuWire (usize),  Cable(usize),
    CuSheet (usize), CuPowder (usize),//constructor copper
    Concrete(usize),//constructor limestone
    CrushedQuartz(usize),Silica(usize),    CrystalOscillator(usize),//constructor quartz
    CateriumIngot(usize),Quickwire(usize),//constructor caterium
    Plastic(usize),    Rubber(usize),      PolymerResin(usize),
    EmptyCanister(usize),PetroleumCoke(usize),//constructor plastic
    PowerSlugBlue(usize),PowerSlugYellow(usize),PowerSlugPurple(usize),
    PowerShard(usize),//power slugs
    SteelIngot(usize), SteelBeam(usize),   SteelPipe(usize),
    IndustrialBeam(usize),//constructor steel
    ReinforcedIronPlate(usize),ModularFrame(usize),HeavyModularFrame(usize),
    FusedModularFrame(usize),//assembler iron
    Rotor(usize),      Stator(usize),      Motor(usize),//assembler motors
    CircuitBoard(usize),HighSpeedConnector(usize),Computer(usize),
    AILimiter(usize),  RadioControlUnit(usize),SuperComputer(usize),//assembler computer
    AlIngot(usize),    AlcladSheet(usize),AlCasing(usize),    Heatsink(usize),
    Battery(usize),    CoolingSystem(usize),AlScrap(usize),
    EmptyFluidTank (usize),//assembler aluminum
    BaseRebar(usize),  ShatterRebar(usize), PulseRebar(usize),
    ExplosiveRebar(usize),//rebar ammo
    BlackPowder(usize),SmokelessPowder(usize),RifleAmmo(usize),
    HomingRifleAmmo(usize),TurboRifleAmmo(usize),//assembler rifle
    Nobelisk(usize),   GasNobelisk(usize),ClusterNobelisk(usize),
    PulseNobelisk(usize),NuclearNobelisk(usize),//assembler nobelisk
    SmartPlating(usize),VersatileFramework(usize),AutomatedWiring(usize),
    ModularEngine(usize),AdaptiveControlUnit(usize),AssemblyDirectorSystem(usize),//assembler elevator
    EMControlRod(usize),PressureConversionCube(usize),//assembler pre-nuclear
    EncasedUCell(usize),URod(usize),UWaste(usize),
    NonFissileU(usize),//uranium
    PuPellet(usize),    PuRod(usize),                 EncasedPuCell(usize),
    NuclearPasta(usize),PWaste(usize),//assembler nuclear
    TurboMotor(usize),  MagneticFieldGenerator(usize),ThermalPropulsionRocket(usize),
    Beacon(usize),      PortableMiner(usize),         GasFilter(usize),
    IodineFilter(usize),//holdable
}


#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Pipeable{
    Water(usize), CrudeOil(usize), NGas(usize),      HeavyOil(usize),
    Fuel(usize),  Turbofuel(usize), LBioFuel(usize),
    NAcid(usize), AlSol(usize),     SAcid(usize)
}

#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Mineable{
    FeNode (usize),     CuNode (usize),   LimestoneNode(usize),//nodes
    CoalNode(usize),    SulfurNode(usize),QuartzNode(usize),
    CateriumNode(usize),UNode(usize),     BauxiteNode(usize),
    SAMNode(usize),
}

#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash)]
pub(crate) enum Pumpable{
    WaterSource(usize), OilSource(usize),   NNode(usize),
}
