use std::collections::HashMap;
use crate::objects::{Conveyable::*, Part, Part::Conveyor};

struct RunOptions{

}


pub fn get_proper_names<'a>() -> HashMap<&'a str, Part>{HashMap::from([
    ("Iron Ore", Conveyor(FeOre)), ("Copper Ore",Conveyor(CuOre)),("Limestone",Conveyor(Limestone)),
    ("Coal",Conveyor(Coal)), ("Sulfur",Conveyor(Sulfur)), ("Raw Quartz",Conveyor(RawQuartz)),
    ("Caterium Ore", Conveyor(CateriumOre)), ("Uranium Ore",Conveyor(UOre)),
    ("Bauxite",Conveyor(Bauxite)), ("SAM",Conveyor(SAM)),
    ("Packaged Water",Conveyor(PkgdWater)), ("Packaged Oil",Conveyor(PkgdOil)),
    ("Packaged Nitrogen",Conveyor(PkgdN)),  ("Packaged Heavy Oil Residue",Conveyor(PkgdHOil)),
    ("Packaged Fuel",Conveyor(PkgdFuel)),  ("Packaged Turbofuel",Conveyor(PkgdTurbofuel)),
    ("Packaged Liquid Biofuel",Conveyor(PkgdLBiofuel)),("Packaged Nitric Acid",Conveyor(PkgdNAcid)),
    ("Packaged Alumina Solution",Conveyor(PkgdAlSol)),("Packaged Sulfuric Acid",Conveyor(PkgdSAcid)),
    ("Leaves",Conveyor(WoodOrLeaves)), ("Wood",Conveyor(WoodOrLeaves)), ("Mycelia",Conveyor(Mycelia)),
    ("Alien Protein",Conveyor(AlienProtein)), ("Alien DNA Capsule",Conveyor(AlienDNA)),
    ("Biomass",Conveyor(Biomass)), ("Solid Biofuel",Conveyor(SolidBiofuel)),
    ("Fabric",Conveyor(Fabric)), ("Compacted Coal",Conveyor(CompactedCoal)),
    ("Flower Petals",Conveyor(FlowerPetals)), ("Color Cartridge",Conveyor(ColorCartridge)),
    ("Ficsmas Gift",Conveyor(FicsmasGift)), ("Actual Snow",Conveyor(ActualSnow)),
    ("Candy Cane",Conveyor(CandyCane)), ("Ficsmas Bow",Conveyor(FicsmasBow)),
    ("Ficsmas Tree",Conveyor(FicsmasTree)), ("Snowball",Conveyor(Snowball)),
    ("Fancy Fireworks",Conveyor(FancyFireworks)), ("Sparkly Fireworks",Conveyor(SparklyFireworks)),
    ("Sweet Fireworks",Conveyor(SweetFireworks)), ("Iron Ornament",Conveyor(FeOrnament)),
    ("Copper Ornament",Conveyor(CuOrnament)), ("Red Ornament",Conveyor(RedOrnament)),
    ("Blue Ornament",Conveyor(BlueOrnament)), ("Ficsmas Branch",Conveyor(FicsmasBranch)),
    ("Ornament Bundle",Conveyor(OrnamentBundle)),("Ficsmas Decoration",Conveyor(FicsmasDecoration)),
    ("Ficsmas Star",Conveyor(FicsmasStar)), ("Iron Ingot",Conveyor(FeIngot)),
    ("Iron Rod",Conveyor(FeRod)), ("Iron Plate",Conveyor(FePlate)), ("Screws",Conveyor(Screws)),
    ("Copper Ingot",Conveyor(CuIngot)), ("Copper Wire",Conveyor(CuWire)), ("Cable",Conveyor(Cable)),
    ("Copper Sheet",Conveyor(CuSheet)), ("Copper Powder",Conveyor(CuPowder)),
    ("Concrete",Conveyor(Concrete)), ("Crushed Quartz",Conveyor(CrushedQuartz)),
    ("Silica",Conveyor(Silica)), ("Crystal Oscillator",Conveyor(CrystalOscillator)),
    ("Caterium Ingot",Conveyor(CateriumIngot)), ("Quickwire",Conveyor(Quickwire)),
    ("Plastic",Conveyor(Plastic)), ("Rubber",Conveyor(Rubber)),
    ("Polymer Resin",Conveyor(PolymerResin)), ("Empty Canister",Conveyor(EmptyCanister)),
    ("Petroleum Coke",Conveyor(PetroleumCoke)),("Blue Power Slug",Conveyor(PowerSlugBlue)),
    ("Yellow Power Slug",Conveyor(PowerSlugYellow)),("Purple Power Slug",Conveyor(PowerSlugPurple)),
    ("Power Shard",Conveyor(PowerShard)), ("Steel Ingot",Conveyor(SteelIngot)),
    ("Steel Beam",Conveyor(SteelBeam)), ("Steel Pipe",Conveyor(SteelPipe)),
    ("Industrial Beam",Conveyor(IndustrialBeam)),
    ("Reinforced Iron Plate",Conveyor(ReinforcedIronPlate)),
    ("Modular Frame",Conveyor(ModularFrame)), ("Heavy Modular Frame",Conveyor(HeavyModularFrame)),
    ("Fused Modular Frame",Conveyor(FusedModularFrame)),
    ("Rotor",Conveyor(Rotor)), ("Stator",Conveyor(Stator)), ("Motor",Conveyor(Motor)),
    ("Circuit Board",Conveyor(CircuitBoard)), ("High-Speed Connector",Conveyor(HighSpeedConnector)),
    ("Computer",Conveyor(Computer)), ("AI Limiter",Conveyor(AILimiter)),
    ("Radio Control Unit",Conveyor(RadioControlUnit)), ("Supercomputer",Conveyor(SuperComputer)),
    ("Aluminum Ingot",Conveyor(AlIngot)), ("Alclad Sheet",Conveyor(AlcladSheet)),
    ("Aluminum Casing",Conveyor(AlCasing)), ("Heatsink",Conveyor(Heatsink)),
    ("Battery",Conveyor(Battery)), ("Cooling System",Conveyor(CoolingSystem)),
    ("Aluminum Scrap",Conveyor(AlScrap)), ("Empty Fluid Tank",Conveyor(EmptyFluidTank)),
    ("Iron Rebar",Conveyor(BaseRebar)), ("Shatter Rebar",Conveyor(ShatterRebar)),
    ("Pulse Rebar",Conveyor(PulseRebar)), ("Explosive Rebar",Conveyor(ExplosiveRebar)),
    ("Black Powder",Conveyor(BlackPowder)), ("Smokeless Powder",Conveyor(SmokelessPowder)),
    ("Rifle Ammo",Conveyor(RifleAmmo)), ("Homing Rifle Ammo",Conveyor(HomingRifleAmmo)),
    ("Turbo Rifle Ammo",Conveyor(TurboRifleAmmo)), ("Nobelisk",Conveyor(Nobelisk)),
    ("Gas Nobelisk",Conveyor(GasNobelisk)), ("Cluster Nobelisk",Conveyor(ClusterNobelisk)),
    ("Pulse Nobelisk",Conveyor(PulseNobelisk)), ("Nuclear Nobelisk",Conveyor(NuclearNobelisk)),
    ("Smart Plating",Conveyor(SmartPlating)), ("Versatile Framework",Conveyor(VersatileFramework)),
    ("Automated Wiring",Conveyor(AutomatedWiring)), ("Modular Engine",Conveyor(ModularEngine)),
    ("Adaptive Control Unit",Conveyor(AdaptiveControlUnit)),
    ("Assembly Director System",Conveyor(AssemblyDirectorSystem)),
    ("Electromagnetic Control Rod",Conveyor(EMControlRod)),
    ("Pressure Conversion Cube",Conveyor(PressureConversionCube)),
    ("Encased Uranium Cell",Conveyor(EncasedUCell)), ("Uranium Fuel Rod",Conveyor(URod)),
    ("Uranium Waste",Conveyor(UWaste)),  ("Non-fissile Uranium",Conveyor(NonFissileU)),
    ("Plutonium Pellet",Conveyor(PuPellet)), ("Plutonium Fuel Rod",Conveyor(PuRod)),
    ("Encased Plutonium Cell",Conveyor(EncasedPuCell)), ("Nuclear Pasta",Conveyor(NuclearPasta)),
    ("Plutonium Waste",Conveyor(PWaste)), ("Turbo Motor",Conveyor(TurboMotor)),
    ("Magnetic Field Generator",Conveyor(MagneticFieldGenerator)),
    ("Thermal Propulsion Rocket",Conveyor(ThermalPropulsionRocket)),  ("Beacon",Conveyor(Beacon)),
    ("Portable Miner",Conveyor(PortableMiner)), ("Gas Filter",Conveyor(GasFilter)),
    ("Iodine Filter", Conveyor(IodineFilter))
])}