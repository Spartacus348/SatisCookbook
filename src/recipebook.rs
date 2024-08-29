use crate::objects::{Process, Building::*, Conveyable::*, Mineable::*, Pipeable::*, Pumpable::*};
use crate::tiers::  {Tier::*, ProgressTier::*,
                        Tier0::*, Tier2::*, Tier3::*, Tier4::*, Tier5::*, Tier7::*, Tier8::*,
                        HardDriveTier::*, MamTrees::*, CateriumTier::*, FicsmasTier::*,
                        SulfurTier::*, FlowerTier::*, FungusTier::*, OrgoTier::*, QuartzTier::*,
                        SlugTier::*};

pub static RECIPES: [Process;145] = [
    Process {
        name:"Mine Iron",
        time:1,
        building: Miner1{
            input:(FeNode (1),),
            output:(FeOre (1),)
        },
        tier:MainProgression(Tier0(HubUpgrade2))
    },
    Process{
        name:"Mine Copper",
        time:1,
        building: Miner1 {
            input:(CuNode (1),),
            output:(CuOre (1),),
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Mine Caterium",
        time:1,
        building: Miner1{
            input:(CateriumNode (1),),
            output:(CateriumOre (1),)
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Mine Coal",
        time:1,
        building: Miner1{
            input:(CoalNode (1),),
            output:(Coal (1),)
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Mine Sulfur",
        time:1,
        building: Miner1{
            input:(SulfurNode (1),),
            output:(Sulfur    (1),)
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Mine Raw Quartz",
        time:1,
        building: Miner1 {
            input:(QuartzNode (1),),
            output:(RawQuartz (1),)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Blue Ficsmas Ornament",
        time:12,
        building: Smelter {
            input:(FicsmasGift (1),),
            output:(BlueOrnament (2),)
        },
        tier:MAM(Ficsmas(TreeUpgrade1))
    },
    Process{
        name:"Caterium Ingot",
        time:4,
        building: Smelter {
            input:(CateriumOre (3),),
            output:(CateriumIngot (1),)
        },
        tier:MAM(Caterium(CateriumIngotResearch))
    },
    Process{
        name:"Copper Ingot",
        time:2,
        building: Smelter {
            input:(CuOre (1),),
            output:(CuIngot (1),)
        },
        tier:MainProgression(Tier0(HubUpgrade2))
    },
    Process{
        name:"Iron Ingot",
        time:2,
        building: Smelter {
            input:(FeOre (1),),
            output:(FeIngot (1),),
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process{
        name:"Red Ficsmas Ornament",
        time:12,
        building:Smelter{
            input:(FicsmasGift (1),),
            output:(RedOrnament (1),)
        },
        tier:MAM(Ficsmas(TreeUpgrade1))
    },
    Process{
        name: "Pure Aluminum Ingot",
        time: 2,
        building:Smelter {
            input:(AlScrap (2),),
            output:(AlIngot(1),)
        },
        tier:HardDrive(MainUnlock(Tier7(BauxiteRefinement)))
    },
    Process{
        name:"Extract Water",
        time:1,
        building:WaterExtractor {
            input:(WaterSource(1),),
            output:(Water(120),)
        },
        tier:MainProgression(Tier3(CoalPower))
    },
    Process{
        name:"Extract Oil",
        time:1,
        building: OilExtractor {
            input:(OilSource(1),),
            output:(CrudeOil(1),)
        },
        tier:MainProgression(Tier5(OilProcessing))
    },
    Process{
        name:"Aluminum Ingot",
        time:4,
        building:Foundry{
            input:(AlScrap(6), Silica (5)),
            output:(AlIngot(4),)
        },
        tier:MainProgression(Tier7(BauxiteRefinement))
    },
    Process{
        name:"Copper Ficsmas Ornament",
        time:12,
        building:Foundry{
            input:(RedOrnament (2), CuIngot (2)),
            output:(CuOrnament (1),)
        },
        tier:MAM(Ficsmas(FicsmasGiftTree))
    },
    Process{
        name:"Iron Ficsmas Ornament",
        time:12,
        building:Foundry{
            input:(BlueOrnament (3), FeIngot (3)),
            output:(FeOrnament (1),)
        },
        tier:MAM(Ficsmas(FicsmasGiftTree))
    },
    Process{
        name:"Steel Ingot",
        time:4,
        building:Foundry{
            input:(FeOre (3),Coal (3)),
            output:(SteelIngot (3),)
        },
        tier:MainProgression(Tier3(BasicSteel))
    },
    Process{
        name:"Coke Steel Ingot",
        time:12,
        building:Foundry{
            input:(FeOre (15), PetroleumCoke (15)),
            output:(SteelIngot (20),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Compacted Steel Ingot",
        time:16,
        building:Foundry{
            input:(FeOre (15), CompactedCoal (3)),
            output:(SteelIngot (10),)
        },
        tier:HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"Compacted Steel Ingot",
        time:16,
        building:Foundry{
            input:(FeOre (15), CompactedCoal (3)),
            output:(SteelIngot (10),)
        },
        tier:HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process{
        name:"Copper Alloy Ingot",
        time:12,
        building:Foundry{
            input:(CuOre (10),FeOre (5)),
            output:(CuIngot (20),)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Iron Alloy Ingot",
        time: 6,
        building:Foundry {
            input:(FeOre (2),CuOre (2)),
            output:(FeIngot (5),)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Solid Steel Ingot",
        time: 3,
        building:Foundry{
            input:(FeIngot (2),Coal (2)),
            output:(SteelIngot (3),)
        },
        tier:HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"Actual Snow",
        time:12,
        building:Constructor {
            input:(FicsmasGift (5),),
            output:(ActualSnow (2),)
        },
        tier:MAM(Ficsmas(AFriend))
    },
    Process{
        name:"Alien DNA Capsule",
        time:6,
        building:Constructor{
            input:(AlienProtein (1),),
            output:(AlienDNA (1),)
        },
        tier:MAM(Organisms(BioOrganicProperties))
    },
    Process{
        name:"Aluminum Casing",
        time: 2,
        building:Constructor {
            input:(AlIngot (3),),
            output:(AlCasing (2),)
        },
        tier:MainProgression(Tier7(BauxiteRefinement))
    },
    Process{
        name:"Biomass (Alien Protein)",
        time:4,
        building: Constructor {
            input:(AlienProtein (1),),
            output:(Biomass (100),)
        },
        tier:MAM(Organisms(BioOrganicProperties))
    },
    Process{
        name:"Biomass (Leaves)",
        time:5,
        building:Constructor {
            input:(Leaves (10),),
            output:(Biomass (5),)
        },
        tier:MainProgression(Tier0(HubUpgrade6))
    },
    Process{
        name:"Biomass (Mycelia)",
        time:4,
        building:Constructor {
            input:(Mycelia (1),),
            output:(Biomass (10),)
        },
        tier: MAM(Fungi(MyceliaResearch))
    },
    Process{
        name:"Biomass (Wood)",
        time:4,
        building:Constructor {
            input:(Wood (4),),
            output:(Biomass ( 20),)
        },
        tier:MainProgression(Tier0(HubUpgrade6))
    },
    Process{
        name:"Cable",
        time:2,
        building:Constructor {
            input:(CuWire (2),),
            output:(Cable (1),)
        },
        tier:MainProgression(Tier0(HubUpgrade2))
    },
    Process{
        name:"Candy Cane",
        time:12,
        building:Constructor {
            input:(FicsmasGift(3),),
            output:(CandyCane(5),)
        },
        tier:MAM(Ficsmas(CandyCaneBasher))
    },
    Process{
        name:"Color Cartridge",
        time:6,
        building:Constructor {
            input:(FlowerPetals (5),),
            output:(ColorCartridge (10),)
        },
        tier:MAM(Flowers(ColorCartridges))
    },
    Process{
        name:"Color Cartridge",
        time:6,
        building:Constructor {
            input:(FlowerPetals (5),),
            output:(ColorCartridge (10),)
        },
        tier:MainProgression(Tier2(ResourceSinkBonus))
    },
    Process{
        name:"Concrete",
        time:4,
        building:Constructor {
            input:(Limestone (3),),
            output:(Concrete (1),)
        },
        tier:MainProgression(Tier0(HubUpgrade3))
    },
    Process{
        name:"Copper Powder",
        time:6,
        building:Constructor {
            input:(CuIngot (30),),
            output:(CuPowder (5),)
        },
        tier:MainProgression(Tier8(ParticleEnrichment))
    },
    Process{
        name:"Copper Sheet",
        time:6,
        building:Constructor {
            input:(CuIngot (2),),
            output:(CuSheet (1),)
        },
        tier:MainProgression(Tier2(PartAssembly))
    },
    Process{
        name:"Empty Canister",
        time:4,
        building:Constructor {
            input:(Plastic (2),),
            output:(EmptyCanister (4),)
        },
        tier:MainProgression(Tier5(AlternativeFuelTransport))
    },
    Process{
        name:"Empty Fluid Tank",
        time:1,
        building:Constructor {
            input:(AlIngot (1),),
            output:(EmptyFluidTank (1),)
        },
        tier:MainProgression(Tier8(AdvancedAluminumProduction))
    },
    Process{
        name:"Ficsmas Bow",
        time:12,
        building:Constructor {
            input:(FicsmasGift (2),),
            output:(FicsmasBow (1),)
        },
        tier:MAM(Ficsmas(CandyCaneDecor))
    },
    Process{
        name:"Ficsmas Tree Branch",
        time:6,
        building:Constructor {
            input:(FicsmasGift (1),),
            output:(FicsmasBranch (1),)
        },
        tier:MAM(Ficsmas(TreeUpgrade0))
    },
    Process{
        name:"Hatcher Protein",
        time:3,
        building:Constructor {
            input:(HatcherProtein (1),),
            output:(AlienProtein (1),)
        },
        tier:MAM(Organisms(HatcherResearch))
    },
    Process{
        name:"Hog Protein",
        time:3,
        building:Constructor {
            input:(HogProtein (1),),
            output:(AlienProtein (1),)
        },
        tier:MAM(Organisms(HogResearch))
    },
    Process{
        name:"Iron Plate",
        time:6,
        building:Constructor {
            input:(FeIngot (3),),
            output:(FePlate (2),)
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Iron Rebar",
        time:4,
        building:Constructor {
            input:(FeRod (1),),
            output:(BaseRebar (1),)
        },
        tier:MAM(Organisms(RebarGun))
    },
    Process{
        name:"Iron Rod",
        time:4,
        building:Constructor {
            input:(FeIngot (1),),
            output:(FeRod (1),)
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Power Shard (1)",
        time:8,
        building:Constructor {
            input:(PowerSlugBlue (1),),
            output:(PowerShard (1),)
        },
        tier: MAM(PowerSlugs(BluePowerSlugs))
    },
    Process{
        name:"Power Shard (2)",
        time:12,
        building:Constructor {
            input:(PowerSlugYellow (1),),
            output:(PowerShard (2),)
        },
        tier:MAM(PowerSlugs(YellowPowerShards))
    },
    Process{
        name:"Power Shard (5)",
        time:24,
        building:Constructor {
            input:(PowerSlugPurple (1),),
            output:(PowerShard (5),)
        },
        tier:MAM(PowerSlugs(PurplePowerShards))
    },
    Process{
        name:"Quartz Crystal",
        time:8,
        building:Constructor {
            input:(RawQuartz (5),),
            output:(CrushedQuartz (3),)
        },
        tier:MAM(Quartz(QuartzResearch))
    },
    Process{
        name:"Quickwire",
        time:5,
        building:Constructor {
            input:(CateriumIngot (1),),
            output:(Quickwire (5),)
        },
        tier:MAM(Caterium(QuickwireResearch))
    },
    Process{
        name:"Screw",
        time:6,
        building:Constructor {
            input:(FeRod (1),),
            output:(Screws (4),)
        },
        tier:MainProgression(Tier0(HubUpgrade3))
    },
    Process{
        name:"Silica",
        time:8,
        building:Constructor {
            input:(RawQuartz (3),),
            output:(Silica (5),)
        },
        tier:MAM(Quartz(SilicaResearch))
    },
    Process{
        name:"Silica",
        time:8,
        building:Constructor {
            input:(RawQuartz (3),),
            output:(Silica (5),)
        },
        tier:MainProgression(Tier7(BauxiteRefinement))
    },
    Process{
        name:"Snowball",
        time:12,
        building:Constructor {
            input:(ActualSnow (3),),
            output:(Snowball (1),)
        },
        tier:MAM(Ficsmas(Snowfight))
    },
    Process{
        name:"Solid Biofuel",
        time:4,
        building:Constructor {
            input:(Biomass (8),),
            output:(SolidBiofuel (4),)
        },
        tier:MainProgression(Tier2(ObstacleClearing))
    },
    Process{
        name:"Splitter Protein",
        time:3,
        building:Constructor {
            input:(SpitterProtein (1),),
            output:(AlienProtein (1),)
        },
        tier:MAM(Organisms(SpitterResearch))
    },
    Process{
        name:"Steel Beam",
        time:4,
        building:Constructor {
            input:(SteelIngot (4),),
            output:(SteelBeam (1),)
        },
        tier:MainProgression(Tier3(BasicSteel))
    },
    Process{
        name:"Steel Pipe",
        time:6,
        building:Constructor {
            input:(SteelIngot (3),),
            output:(SteelPipe (2),)
        },
        tier:MainProgression(Tier3(BasicSteel))
    },
    Process{
        name:"Stinger Protein",
        time:3,
        building:Constructor {
            input:(StingerProtein (1),),
            output:(AlienProtein (1),)
        },
        tier:MAM(Organisms(StingerResearch))
    },
    Process{
        name:"Wire",
        time:4,
        building:Constructor {
            input:(CuIngot (1),),
            output:(CuWire (2),)
        },
        tier:MainProgression(Tier0(HubUpgrade2))
    },
    Process{
        name:"Biocoal",
        time:8,
        building:Constructor {
            input:(Biomass (5),),
            output:(Coal (6),)
        },
        tier:HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process{
        name:"Biocoal",
        time:8,
        building:Constructor {
            input:(Biomass (5),),
            output:(Coal (6),)
        },
        tier:HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process{
        name:"Cast Screw",
        time:24,
        building:Constructor {
            input:(FeIngot (5),),
            output:(Screws (20),)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Caterium Wire",
        time:4,
        building:Constructor {
            input:(CateriumIngot (1),),
            output:(CuWire (8),)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumResearch)))
    },
    Process{
        name:"Charcoal",
        time:4,
        building:Constructor {
            input:(Wood (1),),
            output:(Coal (10),)
        },
        tier:HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process{
        name:"Charcoal",
        time:4,
        building:Constructor {
            input:(Wood (1),),
            output:(Coal (10),)
        },
        tier:HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process{
        name:"Iron Wire",
        time:24,
        building:Constructor {
            input:(FeIngot (5),),
            output:(CuWire (9),)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Steel Canister",
        time:3,
        building:Constructor {
            input:(SteelIngot (3),),
            output:(EmptyCanister (2),)
        },
        tier:HardDrive(MainUnlock(Tier5(AlternativeFuelTransport)))
    },
    Process{
        name:"Steel Rod",
        time:5,
        building:Constructor {
            input:(SteelIngot (1),),
            output:(FeRod (4),)
        },
        tier:HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"Steel Screw",
        time:12,
        building:Constructor {
            input:(SteelBeam (1),),
            output:(Screws (52),)
        },
        tier:HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"AI Limiter",
        time:12,
        building:Assembler {
            input:(CuSheet (5),Quickwire (20)),
            output:(AILimiter (5),)
        },
        tier:MAM(Caterium(AILimiterResearch))
    },
    Process{
        name:"AI Limiter",
        time:12,
        building:Assembler {
            input:(CuSheet (5),Quickwire (20)),
            output:(AILimiter (5),)
        },
        tier:MainProgression(Tier7(AeronauticalEngineering))
    },
    Process{
        name:"Alclad Aluminum Sheet",
        time:6,
        building:Assembler {
            input:(AlIngot (3),CuIngot (1)),
            output:(AlcladSheet (3),)
        },
        tier:MainProgression(Tier7(BauxiteRefinement))
    },
    Process{
        name:"Assembly Director System",
        time:80,
        building:Assembler {
            input:(AdaptiveControlUnit (2),SuperComputer (1)),
            output:(AssemblyDirectorSystem (1),)
        },
        tier:MainProgression(Tier7(AeronauticalEngineering))
    },
    Process{
        name:"Automated Wiring",
        time:24,
        building:Assembler {
            input:(Stator (1),Cable (20)),
            output:(AutomatedWiring (1),)
        },
        tier:MainProgression(Tier4(AdvancedSteel))
    },
    Process{
        name:"Black Powder",
        time:4,
        building:Assembler {
            input:(Coal (1),Sulfur (1)),
            output:(BlackPowder (2),)
        },
        tier:MAM(SulfurTier(BlackPowderResearch))
    },
    Process{
        name:"Circuit Board",
        time:8,
        building:Assembler {
            input:(CuSheet (2),Plastic (4)),
            output:(CircuitBoard (1),)
        },
        tier:MainProgression(Tier5(OilProcessing))
    },
    Process{
        name:"Cluster Nobelisk",
        time:24,
        building:Assembler {
            input:(Nobelisk (3),SmokelessPowder (4)),
            output:(ClusterNobelisk (1),)
        },
        tier:MAM(SulfurTier(ClusterNobeliskResearch))
    },
    Process{
        name:"Electromagnetic Control Rod",
        time:30,
        building:Assembler {
            input:(Stator (3),AILimiter (2)),
            output:(EMControlRod (2),)
        },
        tier:MainProgression(Tier8(NuclearPower))
    },
    Process{
        name:"Encased Industrial Beam",
        time:10,
        building:Assembler {
            input:(SteelBeam (4),Concrete (5)),
            output:(IndustrialBeam (1),)
        },
        tier:MainProgression(Tier4(AdvancedSteel))
    },
    Process{
        name:"Encased Plutonium Cell",
        time:12,
        building:Assembler {
            input:(PuPellet (2),Concrete (4)),
            output:(EncasedPuCell (1),)
        },
        tier:MainProgression(Tier8(ParticleEnrichment))
    },
    Process{
        name:"Ficsmas Decoration",
        time:60,
        building:Assembler {
            input:(FicsmasBranch (15),OrnamentBundle (6)),
            output:(FicsmasDecoration (2),)
        },
        tier:MAM(Ficsmas(ItsSnowing))
    },
    Process{
        name:"Ficsmas Wonder Star",
        time:60,
        building:Assembler {
            input:(FicsmasDecoration (5),CandyCane (20)),
            output:(FicsmasStar (1),)
        },
        tier:MAM(Ficsmas(Wreath))
    },
    Process{
        name:"Ficsmas Ornament Bundle",
        time:12,
        building:Assembler {
            input:(CuOrnament (1),FeOrnament (1)),
            output:(OrnamentBundle (1),)
        },
        tier:MAM(Ficsmas(Lights))
    },
    Process{
        name:"Fabric",
        time:4,
        building:Assembler {
            input:(Mycelia (1),Biomass (5)),
            output:(Fabric (1),)
        },
        tier:MAM(Fungi(FabricResearch))
    },
    Process{
        name:"Fancy Fireworks",
        time:24,
        building:Assembler {
            input:(FicsmasBranch (4),FicsmasBow (3)),
            output:(FancyFireworks (1),)
        },
        tier:MAM(Ficsmas(Fireworks))
    },
    Process{
        name:"Gas Nobelisk",
        time:12,
        building:Assembler {
            input:(Nobelisk (1),Biomass (10)),
            output:(GasNobelisk (1),)
        },
        tier:MAM(Fungi(ToxicCellularModification))
    },
    Process{
        name:"Heat Sink",
        time:8,
        building:Assembler {
            input:(AlcladSheet (5),CuSheet (3)),
            output:(Heatsink (1),)
        },
        tier:MainProgression(Tier8(AdvancedAluminumProduction))
    },
    Process{
        name:"Homing Rifle Ammo",
        time:24,
        building:Assembler {
            input:(RifleAmmo (20),HighSpeedConnector (1)),
            output:(HomingRifleAmmo (10),)
        },
        tier:MAM(Caterium(BulletGuidanceSystem))
    },
    Process{
        name:"Modular Frame",
        time:60,
        building:Assembler {
            input:(ReinforcedIronPlate (3),FeRod (12)),
            output:(ModularFrame (2),)
        },
        tier:MainProgression(Tier2(PartAssembly))
    },
    Process{
        name:"Motor",
        time:12,
        building:Assembler {
            input:(Rotor (2),Stator (2)),
            output:(Motor (1),)
        },
        tier: MainProgression(Tier4(AdvancedSteel))
    },
    Process{
        name:"Nobelisk",
        time:6,
        building:Assembler {
            input:(BlackPowder (2),SteelPipe (2)),
            output:(Nobelisk (1),)
        },
        tier:MAM(SulfurTier(NobeliskDetonator))
    },
    Process{
        name:"Pressure Conversion Cube",
        time:60,
        building:Assembler {
            input:(FusedModularFrame (1),RadioControlUnit (2)),
            output:(PressureConversionCube (1),)
        },
        tier:MainProgression(Tier8(ParticleEnrichment))
    },
    Process {
        name: "Pulse Nobelisk",
        time: 60,
        building: Assembler {
            input: (Nobelisk(5),CrystalOscillator(1)),
            output: (PulseNobelisk(5 ),)
        },
        tier:MAM(Quartz(ExplosiveResonanceApplication))
    },
    Process{
        name:"Reinforced Iron Plate",
        time:12,
        building:Assembler {
            input:(FePlate (6),Screws (12)),
            output:(ReinforcedIronPlate (1),)
        },
        tier:MainProgression(Tier0(HubUpgrade3))
    },
    Process{
        name:"Rifle Ammo",
        time:12,
        building:Assembler {
            input:(CuSheet (3),SmokelessPowder (2)),
            output:(RifleAmmo (15),)
        },
        tier:MAM(SulfurTier(Rifle))
    },
    Process{
        name:"Rotor",
        time:15,
        building:Assembler {
            input:(FeRod (5),Screws (25)),
            output:(Rotor (1),)
        },
        tier:MainProgression(Tier2(PartAssembly))
    },
    Process{
        name:"Shatter Rebar",
        time:12,
        building:Assembler {
            input:(BaseRebar (2),CrushedQuartz (3)),
            output:(ShatterRebar (1),)
        },
        tier:MAM(Quartz(ShatterRebarResearch))
    },
    Process{
        name:"Smart Plating",
        time:30,
        building:Assembler {
            input:(ReinforcedIronPlate (1),Rotor (1)),
            output:(SmartPlating (1),)
        },
        tier:MainProgression(Tier2(PartAssembly))
    },
    Process{
        name:"Sparkly Fireworks",
        time:24,
        building:Assembler {
            input:(FicsmasBranch (3),ActualSnow (2)),
            output:(SparklyFireworks (1),)
        },
        tier:MAM(Ficsmas(Fireworks))
    },
    Process{
        name:"Stator",
        time:12,
        building:Assembler {
            input:(SteelPipe (3),CuWire (8)),
            output:(Stator (1),)
        },
        tier:MainProgression(Tier4(AdvancedSteel))
    },
    Process{
        name:"Stun Rebar",
        time:6,
        building:Assembler {
            input:(BaseRebar (1),Quickwire (5)),
            output:(PulseRebar (1),)
        },
        tier:MAM(Caterium(StunRebarResearch))
    },
    Process{
        name:"Sweet Fireworks",
        time:24,
        building:Assembler {
            input:(FicsmasBranch (6),CandyCane (3)),
            output:(SweetFireworks (1),)
        },
        tier:MAM(Ficsmas(Fireworks))
    },
    Process{
        name:"Versatile Framework",
        time:24,
        building:Assembler {
            input:(ModularFrame (1),SteelBeam (12)),
            output:(VersatileFramework (2),)
        },
        tier:MainProgression(Tier3(BasicSteel))
    },
    Process{
        name:"Adhered Iron Plate",
        time:16,
        building:Assembler {
            input:(FePlate (3),Rubber (1)),
            output:(ReinforcedIronPlate (1),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Alclad Casing",
        time:8,
        building:Assembler {
            input:(AlIngot (20),CuIngot (10)),
            output:(AlCasing (15),)
        },
        tier:HardDrive(MainUnlock(Tier7(BauxiteRefinement)))
    },
    Process{
        name:"Bolted Frame",
        time:24,
        building:Assembler {
            input:(ReinforcedIronPlate (3),Screws (56)),
            output:(ModularFrame (2),)
        },
        tier:HardDrive(MainUnlock(Tier2(PartAssembly)))
    },
    Process{
        name:"Bolted Iron Plate",
        time:12,
        building:Assembler {
            input:(FePlate (18),Screws (50)),
            output:(ReinforcedIronPlate (3),)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Caterium Circuit Board",
        time:48,
        building:Assembler {
            input:(Plastic (10),Quickwire (30)),
            output:(CircuitBoard (7),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Caterium Circuit Board",
        time:48,
        building:Assembler {
            input:(Plastic (10),Quickwire (30)),
            output:(CircuitBoard (7),)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process{
        name:"Cheap Silica",
        time:16,
        building:Assembler {
            input:(RawQuartz (3),Limestone (5)),
            output:(Silica (7),)
        },
        tier:HardDrive(MAMUnlock(Quartz(SilicaResearch)))
    },
    Process{
        name:"Coated Iron Canister",
        time:4,
        building:Assembler {
            input:(FePlate (2),CuSheet (1)),
            output:(EmptyCanister (4),)
        },
        tier:HardDrive(MainUnlock(Tier5(AlternativeFuelTransport)))
    },
    Process{
        name:"Coated Iron Plate",
        time:12,
        building:Assembler {
            input:(FeIngot (10),Plastic (2)),
            output:(FePlate (15),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Compacted Coal",
        time:12,
        building:Assembler {
            input:(Coal (5),Sulfur (5)),
            output:(CompactedCoal (5),)
        },
        tier:HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process{
        name:"Copper Rotor",
        time:16,
        building:Assembler {
            input:(CuSheet (6),Screws (52)),
            output:(Rotor(3),)
        },
        tier:HardDrive(MainUnlock(Tier2(PartAssembly)))
    },
    Process{
        name:"Crystal Computer",
        time:64,
        building:Assembler {
            input:(CircuitBoard (8),CrystalOscillator (3)),
            output:(Computer (3),)
        },
        tier:HardDrive(MainUnlock(Tier5(IndustrialManufacturing)))
    },
    Process{
        name:"Crystal Computer",
        time:64,
        building:Assembler {
            input:(CircuitBoard (8),CrystalOscillator (3)),
            output:(Computer (3),)
        },
        tier:HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process{
        name:"Electric Motor",
        time:16,
        building:Assembler {
            input:(EMControlRod (1),Rotor (2)),
            output:(Motor (2),)
        },
        tier:HardDrive(MainUnlock(Tier7(AeronauticalEngineering)))
    },
    Process{
        name:"Electrode Circuit Board",
        time:12,
        building:Assembler {
            input:(Rubber (6),PetroleumCoke (9)),
            output:(CircuitBoard (1),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Electromagnetic Connection Rod",
        time:15,
        building:Assembler {
            input:(Stator (2),HighSpeedConnector (1)),
            output:(EMControlRod (2),)
        },
        tier:HardDrive(MainUnlock(Tier8(NuclearPower)))
    },
    Process{
        name:"Electromagnetic Connection Rod",
        time:15,
        building:Assembler {
            input:(Stator (2),HighSpeedConnector (1)),
            output:(EMControlRod (2),)
        },
        tier:HardDrive(MAMUnlock(Caterium(AILimiterResearch)))
    },
    Process{
        name:"Encased Industrial Pipe",
        time:15,
        building:Assembler {
            input:(SteelPipe (7),Concrete (5)),
            output:(IndustrialBeam (1),)
        },
        tier:HardDrive(MainUnlock(Tier4(AdvancedSteel)))
    },
    Process{
        name:"Fine Black Powder",
        time:16,
        building:Assembler {
            input:(Sulfur (2),CompactedCoal (1)),
            output:(BlackPowder (4),)
        },
        tier:HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process{
        name:"Fine Concrete",
        time:24,
        building:Assembler {
            input:(Silica (3),Limestone (12)),
            output:(Concrete (10),)
        },
        tier:HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process{
        name:"Fused Quickwire",
        time:8,
        building:Assembler {
            input:(CateriumIngot (1),CuIngot (5)),
            output:(Quickwire (12),)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process{
        name:"Fused Wire",
        time:20,
        building:Assembler {
            input:(CuIngot (4),CateriumIngot (1)),
            output:(CuWire (30),)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumResearch)))
    },
    Process{
        name:"Heat Exchanger",
        time:6,
        building:Assembler {
            input:(AlCasing (3),Rubber (3)),
            output:(Heatsink (1),)
        },
        tier:HardDrive(MainUnlock(Tier8(AdvancedAluminumProduction)))
    },
    Process{
        name:"Insulated Cable",
        time:12,
        building:Assembler {
            input:(CuWire (9),Rubber (6)),
            output:(Cable (20),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"OC Supercomputer",
        time:20,
        building:Assembler {
            input:(RadioControlUnit (3),CoolingSystem (3)),
            output:(SuperComputer (1),)
        },
        tier:HardDrive(MainUnlock(Tier7(AeronauticalEngineering)))
    },
    Process{
        name:"OC Supercomputer",
        time:20,
        building:Assembler {
            input:(RadioControlUnit (3),CoolingSystem (3)),
            output:(SuperComputer (1),)
        },
        tier:HardDrive(MainUnlock(Tier8(AdvancedAluminumProduction)))
    },
    Process{
        name:"Plutonium Fuel Unit",
        time:120,
        building:Assembler {
            input:(EncasedPuCell (20),PressureConversionCube (1)),
            output:(PuRod (1),)
        },
        tier:HardDrive(MainUnlock(Tier8(ParticleEnrichment)))
    },
    Process{
        name:"Quickwire Cable",
        time:24,
        building:Assembler {
            input:(Quickwire (3),Rubber(2)),
            output:(Cable (11),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Quickwire Cable",
        time:24,
        building:Assembler {
            input:(Quickwire (3),Rubber(2)),
            output:(Cable (11),)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process{
        name:"Quickwire Stator",
        time:15,
        building:Assembler {
            input:(SteelPipe (4),Quickwire (15)),
            output:(Stator(2),)
        },
        tier:HardDrive(MainUnlock(Tier4(AdvancedSteel)))
    },
    Process{
        name:"Quickwire Stator",
        time:15,
        building:Assembler {
            input:(SteelPipe (4),Quickwire (15)),
            output:(Stator(2),)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process{
        name:"Rubber Concrete",
        time:12,
        building:Assembler {
            input:(Limestone (10),Rubber (2)),
            output:(Concrete (9),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Silicon Circuit Board",
        time:24,
        building:Assembler {
            input:(CuSheet (11),Silica (11)),
            output:(CircuitBoard (5),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Silicon Circuit Board",
        time:24,
        building:Assembler {
            input:(CuSheet (11),Silica (11)),
            output:(CircuitBoard (5),)
        },
        tier:HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process{
        name:"Steel Coated Plate",
        time:24,
        building:Assembler {
            input:(SteelIngot (2),Plastic(2)),
            output:(FePlate (18),)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Steel Rotor",
        time:12,
        building:Assembler {
            input:(SteelPipe (2),CuWire (6)),
            output:(Rotor(1),)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"Steeled Frame",
        time:60,
        building:Assembler {
            input:(ReinforcedIronPlate (2),SteelPipe (10)),
            output:(ModularFrame (3),)
        },
        tier:HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"Stitched Iron Plate",
        time:32,
        building:Assembler {
            input:(FePlate (10),CuWire (20)),
            output:(ReinforcedIronPlate (3),)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Adaptive Control Unit",
        time:120,
        building:Manufacturer {
            input:(AutomatedWiring (15),CircuitBoard (10), HeavyModularFrame (2), Some(Computer (2))),
            output:(AdaptiveControlUnit(2),)
        },
        tier:MainProgression(Tier5(IndustrialManufacturing))
    },
];
