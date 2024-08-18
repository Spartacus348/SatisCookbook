use crate::objects::{Process, Building::*, Conveyable::*, Mineable::*, Pipeable::*, Pumpable::*};
use crate::tiers::{Tier::*, ProgressTier::*, Tier0::*, Tier1::*, Tier2::*, Tier3::*, Tier4::*,
                   Tier5::*, Tier6::*, Tier7::*,Tier8::*,
                   HardDriveTier::*, MamTrees::*, CateriumTier::*, FicsmasTier::*, SulfurTier::*,
                    FlowerTier::*, FungusTier::*, OrgoTier::*, QuartzTier::*, SlugTier::*};

pub static RECIPES: [Process;145] = [
    Process {
        name:"Mine Iron",
        time:1,
        building: Miner1{
            input:(FeNode {amount:1},),
            output:(FeOre {amount:1},)
        },
        tier:MainProgression(Tier0(HubUpgrade2))
    },
    Process{
        name:"Mine Copper",
        time:1,
        building: Miner1 {
            input:(CuNode {amount:1},),
            output:(CuOre {amount:1},),
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Mine Caterium",
        time:1,
        building: Miner1{
            input:(CateriumNode {amount:1},),
            output:(CateriumOre {amount:1},)
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Mine Coal",
        time:1,
        building: Miner1{
            input:(CoalNode {amount:1},),
            output:(Coal {amount:1},)
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Mine Sulfur",
        time:1,
        building: Miner1{
            input:(SulfurNode {amount:1},),
            output:(Sulfur    {amount:1},)
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Mine Raw Quartz",
        time:1,
        building: Miner1 {
            input:(QuartzNode {amount:1},),
            output:(RawQuartz {amount:1},)
        },
        tier: MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Blue Ficsmas Ornament",
        time:12,
        building: Smelter {
            input:(FicsmasGift {amount:1},),
            output:(BlueOrnament {amount:2},)
        },
        tier:MAM(Ficsmas(TreeUpgrade1))
    },
    Process{
        name:"Caterium Ingot",
        time:4,
        building: Smelter {
            input:(CateriumOre {amount:3},),
            output:(CateriumIngot {amount:1},)
        },
        tier:MAM(Caterium(CateriumIngotResearch))
    },
    Process{
        name:"Copper Ingot",
        time:2,
        building: Smelter {
            input:(CuOre {amount:1},),
            output:(CuIngot {amount:1},)
        },
        tier:MainProgression(Tier0(HubUpgrade2))
    },
    Process{
        name:"Iron Ingot",
        time:2,
        building: Smelter {
            input:(FeOre {amount:1},),
            output:(FeIngot {amount:1},),
        },
        tier: MainProgression(Tier0(HubUpgrade2))
    },
    Process{
        name:"Red Ficsmas Ornament",
        time:12,
        building:Smelter{
            input:(FicsmasGift {amount:1},),
            output:(RedOrnament {amount:1},)
        },
        tier:MAM(Ficsmas(TreeUpgrade1))
    },
    Process{
        name: "Pure Aluminum Ingot",
        time: 2,
        building:Smelter {
            input:(AlScrap {amount:2},),
            output:(AlIngot{amount:1},)
        },
        tier:HardDrive(MainUnlock(Tier7(BauxiteRefinement)))
    },
    Process{
        name:"Extract Water",
        time:1,
        building:WaterExtractor {
            input:(WaterSource {amount:1},),
            output:(Water {amount:120},)
        },
        tier:MainProgression(Tier3(CoalPower))
    },
    Process{
        name:"Extract Oil",
        time:1,
        building: OilExtractor {
            input:(OilSource{amount:1},),
            output:(CrudeOil {amount:1},)
        },
        tier:MainProgression(Tier5(OilProcessing))
    },
    Process{
        name:"Aluminum Ingot",
        time:4,
        building:Foundry{
            input:(AlScrap {amount:6}, Some(Silica {amount:5})),
            output:(AlIngot {amount:4},)
        },
        tier:MainProgression(Tier7(BauxiteRefinement))
    },
    Process{
        name:"Copper Ficsmas Ornament",
        time:12,
        building:Foundry{
            input:(RedOrnament {amount:2},Some(CuIngot {amount:2})),
            output:(CuOrnament {amount:1},)
        },
        tier:MAM(Ficsmas(FicsmasGiftTree))
    },
    Process{
        name:"Iron Ficsmas Ornament",
        time:12,
        building:Foundry{
            input:(BlueOrnament {amount:3}, Some(FeIngot {amount:3})),
            output:(FeOrnament {amount:1},)
        },
        tier:MAM(Ficsmas(FicsmasGiftTree))
    },
    Process{
        name:"Steel Ingot",
        time:4,
        building:Foundry{
            input:(FeOre {amount:3},Some(Coal {amount:3})),
            output:(SteelIngot {amount:3},)
        },
        tier:MainProgression(Tier3(BasicSteel))
    },
    Process{
        name:"Coke Steel Ingot",
        time:12,
        building:Foundry{
            input:(FeOre {amount:15}, Some(PetroleumCoke {amount:15})),
            output:(SteelIngot {amount:20},)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Compacted Steel Ingot",
        time:16,
        building:Foundry{
            input:(FeOre {amount:15}, Some(CompactedCoal {amount:3})),
            output:(SteelIngot {amount:10},)
        },
        tier:HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"Compacted Steel Ingot",
        time:16,
        building:Foundry{
            input:(FeOre {amount:15}, Some(CompactedCoal {amount:3})),
            output:(SteelIngot {amount:10},)
        },
        tier:HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process{
        name:"Copper Alloy Ingot",
        time:12,
        building:Foundry{
            input:(CuOre {amount:10}, Some(FeOre {amount:5})),
            output:(CuIngot {amount:20},)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Iron Alloy Ingot",
        time: 6,
        building:Foundry {
            input:(FeOre {amount:2}, Some(CuOre {amount:2})),
            output:(FeIngot {amount:5},)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Solid Steel Ingot",
        time: 3,
        building:Foundry{
            input:(FeIngot {amount:2}, Some(Coal {amount:2})),
            output:(SteelIngot {amount:3},)
        },
        tier:HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"Actual Snow",
        time:12,
        building:Constructor {
            input:(FicsmasGift {amount:5},),
            output:(ActualSnow {amount:2},)
        },
        tier:MAM(Ficsmas(AFriend))
    },
    Process{
        name:"Alien DNA Capsule",
        time:6,
        building:Constructor{
            input:(AlienProtein {amount:1},),
            output:(AlienDNA {amount:1},)
        },
        tier:MAM(Organisms(BioOrganicProperties))
    },
    Process{
        name:"Aluminum Casing",
        time: 2,
        building:Constructor {
            input:(AlIngot {amount:3},),
            output:(AlCasing {amount:2},)
        },
        tier:MainProgression(Tier7(BauxiteRefinement))
    },
    Process{
        name:"Biomass (Alien Protein)",
        time:4,
        building: Constructor {
            input:(AlienProtein {amount:1},),
            output:(Biomass {amount:100},)
        },
        tier:MAM(Organisms(BioOrganicProperties))
    },
    Process{
        name:"Biomass (Leaves)",
        time:5,
        building:Constructor {
            input:(Leaves {amount:10},),
            output:(Biomass {amount:5},)
        },
        tier:MainProgression(Tier0(HubUpgrade6))
    },
    Process{
        name:"Biomass (Mycelia)",
        time:4,
        building:Constructor {
            input:(Mycelia {amount:1},),
            output:(Biomass {amount:10},)
        },
        tier: MAM(Fungi(MyceliaResearch))
    },
    Process{
        name:"Biomass (Wood)",
        time:4,
        building:Constructor {
            input:(Wood {amount:4},),
            output:(Biomass {amount: 20},)
        },
        tier:MainProgression(Tier0(HubUpgrade6))
    },
    Process{
        name:"Cable",
        time:2,
        building:Constructor {
            input:(CuWire {amount:2},),
            output:(Cable {amount:1},)
        },
        tier:MainProgression(Tier0(HubUpgrade2))
    },
    Process{
        name:"Candy Cane",
        time:12,
        building:Constructor {
            input:(FicsmasGift {amount:3},),
            output:(CandyCane {amount:5},)
        },
        tier:MAM(Ficsmas(CandyCaneBasher))
    },
    Process{
        name:"Color Cartridge",
        time:6,
        building:Constructor {
            input:(FlowerPetals {amount:5},),
            output:(ColorCartridge {amount:10},)
        },
        tier:MAM(Flowers(ColorCartridges))
    },
    Process{
        name:"Color Cartridge",
        time:6,
        building:Constructor {
            input:(FlowerPetals {amount:5},),
            output:(ColorCartridge {amount:10},)
        },
        tier:MainProgression(Tier2(ResourceSinkBonus))
    },
    Process{
        name:"Concrete",
        time:4,
        building:Constructor {
            input:(Limestone {amount:3},),
            output:(Concrete {amount:1},)
        },
        tier:MainProgression(Tier0(HubUpgrade3))
    },
    Process{
        name:"Copper Powder",
        time:6,
        building:Constructor {
            input:(CuIngot {amount:30},),
            output:(CuPowder {amount:5},)
        },
        tier:MainProgression(Tier8(ParticleEnrichment))
    },
    Process{
        name:"Copper Sheet",
        time:6,
        building:Constructor {
            input:(CuIngot {amount:2},),
            output:(CuSheet {amount:1},)
        },
        tier:MainProgression(Tier2(PartAssembly))
    },
    Process{
        name:"Empty Canister",
        time:4,
        building:Constructor {
            input:(Plastic {amount:2},),
            output:(EmptyCanister {amount:4},)
        },
        tier:MainProgression(Tier5(AlternativeFuelTransport))
    },
    Process{
        name:"Empty Fluid Tank",
        time:1,
        building:Constructor {
            input:(AlIngot {amount:1},),
            output:(EmptyFluidTank {amount:1},)
        },
        tier:MainProgression(Tier8(AdvancedAluminumProduction))
    },
    Process{
        name:"Ficsmas Bow",
        time:12,
        building:Constructor {
            input:(FicsmasGift {amount:2},),
            output:(FicsmasBow {amount:1},)
        },
        tier:MAM(Ficsmas(CandyCaneDecor))
    },
    Process{
        name:"Ficsmas Tree Branch",
        time:6,
        building:Constructor {
            input:(FicsmasGift {amount:1},),
            output:(FicsmasBranch {amount:1},)
        },
        tier:MAM(Ficsmas(TreeUpgrade0))
    },
    Process{
        name:"Hatcher Protein",
        time:3,
        building:Constructor {
            input:(HatcherProtein {amount:1},),
            output:(AlienProtein {amount:1},)
        },
        tier:MAM(Organisms(HatcherResearch))
    },
    Process{
        name:"Hog Protein",
        time:3,
        building:Constructor {
            input:(HogProtein {amount:1},),
            output:(AlienProtein {amount:1},)
        },
        tier:MAM(Organisms(HogResearch))
    },
    Process{
        name:"Iron Plate",
        time:6,
        building:Constructor {
            input:(FeIngot {amount:3},),
            output:(FePlate {amount:2},)
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Iron Rebar",
        time:4,
        building:Constructor {
            input:(FeRod {amount:1},),
            output:(BaseRebar {amount:1},)
        },
        tier:MAM(Organisms(RebarGun))
    },
    Process{
        name:"Iron Rod",
        time:4,
        building:Constructor {
            input:(FeIngot {amount:1},),
            output:(FeRod {amount:1},)
        },
        tier:MainProgression(Tier0(Onboarding))
    },
    Process{
        name:"Power Shard (1)",
        time:8,
        building:Constructor {
            input:(PowerSlugBlue {amount:1},),
            output:(PowerShard {amount:1},)
        },
        tier: MAM(PowerSlugs(BluePowerSlugs))
    },
    Process{
        name:"Power Shard (2)",
        time:12,
        building:Constructor {
            input:(PowerSlugYellow {amount:1},),
            output:(PowerShard {amount:2},)
        },
        tier:MAM(PowerSlugs(YellowPowerShards))
    },
    Process{
        name:"Power Shard (5)",
        time:24,
        building:Constructor {
            input:(PowerSlugPurple {amount:1},),
            output:(PowerShard {amount:5},)
        },
        tier:MAM(PowerSlugs(PurplePowerShards))
    },
    Process{
        name:"Quartz Crystal",
        time:8,
        building:Constructor {
            input:(RawQuartz {amount:5},),
            output:(CrushedQuartz {amount:3},)
        },
        tier:MAM(Quartz(QuartzResearch))
    },
    Process{
        name:"Quickwire",
        time:5,
        building:Constructor {
            input:(CateriumIngot {amount:1},),
            output:(Quickwire {amount:5},)
        },
        tier:MAM(Caterium(QuickwireResearch))
    },
    Process{
        name:"Screw",
        time:6,
        building:Constructor {
            input:(FeRod {amount:1},),
            output:(Screws {amount:4},)
        },
        tier:MainProgression(Tier0(HubUpgrade3))
    },
    Process{
        name:"Silica",
        time:8,
        building:Constructor {
            input:(RawQuartz {amount:3},),
            output:(Silica {amount:5},)
        },
        tier:MAM(Quartz(SilicaResearch))
    },
    Process{
        name:"Silica",
        time:8,
        building:Constructor {
            input:(RawQuartz {amount:3},),
            output:(Silica {amount:5},)
        },
        tier:MainProgression(Tier7(BauxiteRefinement))
    },
    Process{
        name:"Snowball",
        time:12,
        building:Constructor {
            input:(ActualSnow {amount:3},),
            output:(Snowball {amount:1},)
        },
        tier:MAM(Ficsmas(Snowfight))
    },
    Process{
        name:"Solid Biofuel",
        time:4,
        building:Constructor {
            input:(Biomass {amount:8},),
            output:(SolidBiofuel {amount:4},)
        },
        tier:MainProgression(Tier2(ObstacleClearing))
    },
    Process{
        name:"Splitter Protein",
        time:3,
        building:Constructor {
            input:(SpitterProtein {amount:1},),
            output:(AlienProtein {amount:1},)
        },
        tier:MAM(Organisms(SpitterResearch))
    },
    Process{
        name:"Steel Beam",
        time:4,
        building:Constructor {
            input:(SteelIngot {amount:4},),
            output:(SteelBeam {amount:1},)
        },
        tier:MainProgression(Tier3(BasicSteel))
    },
    Process{
        name:"Steel Pipe",
        time:6,
        building:Constructor {
            input:(SteelIngot {amount:3},),
            output:(SteelPipe {amount:2},)
        },
        tier:MainProgression(Tier3(BasicSteel))
    },
    Process{
        name:"Stinger Protein",
        time:3,
        building:Constructor {
            input:(StingerProtein {amount:1},),
            output:(AlienProtein {amount:1},)
        },
        tier:MAM(Organisms(StingerResearch))
    },
    Process{
        name:"Wire",
        time:4,
        building:Constructor {
            input:(CuIngot {amount:1},),
            output:(CuWire {amount:2},)
        },
        tier:MainProgression(Tier0(HubUpgrade2))
    },
    Process{
        name:"Biocoal",
        time:8,
        building:Constructor {
            input:(Biomass {amount:5},),
            output:(Coal {amount:6},)
        },
        tier:HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process{
        name:"Biocoal",
        time:8,
        building:Constructor {
            input:(Biomass {amount:5},),
            output:(Coal {amount:6},)
        },
        tier:HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process{
        name:"Cast Screw",
        time:24,
        building:Constructor {
            input:(FeIngot {amount:5},),
            output:(Screws {amount:20},)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Caterium Wire",
        time:4,
        building:Constructor {
            input:(CateriumIngot {amount:1},),
            output:(CuWire {amount:8},)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumResearch)))
    },
    Process{
        name:"Charcoal",
        time:4,
        building:Constructor {
            input:(Wood {amount:1},),
            output:(Coal {amount:10},)
        },
        tier:HardDrive(MainUnlock(Tier3(CoalPower)))
    },
    Process{
        name:"Charcoal",
        time:4,
        building:Constructor {
            input:(Wood {amount:1},),
            output:(Coal {amount:10},)
        },
        tier:HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process{
        name:"Iron Wire",
        time:24,
        building:Constructor {
            input:(FeIngot {amount:5},),
            output:(CuWire {amount:9},)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Steel Canister",
        time:3,
        building:Constructor {
            input:(SteelIngot {amount:3},),
            output:(EmptyCanister {amount:2},)
        },
        tier:HardDrive(MainUnlock(Tier5(AlternativeFuelTransport)))
    },
    Process{
        name:"Steel Rod",
        time:5,
        building:Constructor {
            input:(SteelIngot {amount:1},),
            output:(FeRod {amount:4},)
        },
        tier:HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"Steel Screw",
        time:12,
        building:Constructor {
            input:(SteelBeam {amount:1},),
            output:(Screws {amount:52},)
        },
        tier:HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"AI Limiter",
        time:12,
        building:Assembler {
            input:(CuSheet {amount:5},Some(Quickwire {amount:20})),
            output:(AILimiter {amount:5},)
        },
        tier:MAM(Caterium(AILimiterResearch))
    },
    Process{
        name:"AI Limiter",
        time:12,
        building:Assembler {
            input:(CuSheet {amount:5},Some(Quickwire {amount:20})),
            output:(AILimiter {amount:5},)
        },
        tier:MainProgression(Tier7(AeronauticalEngineering))
    },
    Process{
        name:"Alclad Aluminum Sheet",
        time:6,
        building:Assembler {
            input:(AlIngot {amount:3},Some(CuIngot {amount:1})),
            output:(AlcladSheet {amount:3},)
        },
        tier:MainProgression(Tier7(BauxiteRefinement))
    },
    Process{
        name:"Assembly Director System",
        time:80,
        building:Assembler {
            input:(AdaptiveControlUnit {amount:2},Some(SuperComputer {amount:1})),
            output:(AssemblyDirectorSystem {amount:1},)
        },
        tier:MainProgression(Tier7(AeronauticalEngineering))
    },
    Process{
        name:"Automated Wiring",
        time:24,
        building:Assembler {
            input:(Stator {amount:1},Some(Cable {amount:20})),
            output:(AutomatedWiring {amount:1},)
        },
        tier:MainProgression(Tier4(AdvancedSteel))
    },
    Process{
        name:"Black Powder",
        time:4,
        building:Assembler {
            input:(Coal {amount:1},Some(Sulfur {amount:1})),
            output:(BlackPowder {amount:2},)
        },
        tier:MAM(SulfurTier(BlackPowderResearch))
    },
    Process{
        name:"Circuit Board",
        time:8,
        building:Assembler {
            input:(CuSheet {amount:2},Some(Plastic {amount:4})),
            output:(CircuitBoard {amount:1},)
        },
        tier:MainProgression(Tier5(OilProcessing))
    },
    Process{
        name:"Cluster Nobelisk",
        time:24,
        building:Assembler {
            input:(Nobelisk {amount:3},Some(SmokelessPowder {amount:4})),
            output:(ClusterNobelisk {amount:1},)
        },
        tier:MAM(SulfurTier(ClusterNobeliskResearch))
    },
    Process{
        name:"Electromagnetic Control Rod",
        time:30,
        building:Assembler {
            input:(Stator {amount:3},Some(AILimiter {amount:2})),
            output:(EMControlRod {amount:2},)
        },
        tier:MainProgression(Tier8(NuclearPower))
    },
    Process{
        name:"Encased Industrial Beam",
        time:10,
        building:Assembler {
            input:(SteelBeam {amount:4},Some(Concrete {amount:5})),
            output:(IndustrialBeam {amount:1},)
        },
        tier:MainProgression(Tier4(AdvancedSteel))
    },
    Process{
        name:"Encased Plutonium Cell",
        time:12,
        building:Assembler {
            input:(PuPellet {amount:2},Some(Concrete {amount:4})),
            output:(EncasedPuCell {amount:1},)
        },
        tier:MainProgression(Tier8(ParticleEnrichment))
    },
    Process{
        name:"Ficsmas Decoration",
        time:60,
        building:Assembler {
            input:(FicsmasBranch {amount:15},Some(OrnamentBundle {amount:6})),
            output:(FicsmasDecoration {amount:2},)
        },
        tier:MAM(Ficsmas(ItsSnowing))
    },
    Process{
        name:"Ficsmas Wonder Star",
        time:60,
        building:Assembler {
            input:(FicsmasDecoration {amount:5},Some(CandyCane {amount:20})),
            output:(FicsmasStar {amount:1},)
        },
        tier:MAM(Ficsmas(Wreath))
    },
    Process{
        name:"Ficsmas Ornament Bundle",
        time:12,
        building:Assembler {
            input:(CuOrnament {amount:1},Some(FeOrnament {amount:1})),
            output:(OrnamentBundle {amount:1},)
        },
        tier:MAM(Ficsmas(Lights))
    },
    Process{
        name:"Fabric",
        time:4,
        building:Assembler {
            input:(Mycelia {amount:1},Some(Biomass {amount:5})),
            output:(Fabric {amount:1},)
        },
        tier:MAM(Fungi(FabricResearch))
    },
    Process{
        name:"Fancy Fireworks",
        time:24,
        building:Assembler {
            input:(FicsmasBranch {amount:4},Some(FicsmasBow {amount:3})),
            output:(FancyFireworks {amount:1},)
        },
        tier:MAM(Ficsmas(Fireworks))
    },
    Process{
        name:"Gas Nobelisk",
        time:12,
        building:Assembler {
            input:(Nobelisk {amount:1},Some(Biomass {amount:10})),
            output:(GasNobelisk {amount:1},)
        },
        tier:MAM(Fungi(ToxicCellularModification))
    },
    Process{
        name:"Heat Sink",
        time:8,
        building:Assembler {
            input:(AlcladSheet {amount:5},Some(CuSheet {amount:3})),
            output:(Heatsink {amount:1},)
        },
        tier:MainProgression(Tier8(AdvancedAluminumProduction))
    },
    Process{
        name:"Homing Rifle Ammo",
        time:24,
        building:Assembler {
            input:(RifleAmmo {amount:20},Some(HighSpeedConnector {amount:1})),
            output:(HomingRifleAmmo {amount:10},)
        },
        tier:MAM(Caterium(BulletGuidanceSystem))
    },
    Process{
        name:"Modular Frame",
        time:60,
        building:Assembler {
            input:(ReinforcedIronPlate {amount:3},Some(FeRod {amount:12})),
            output:(ModularFrame {amount:2},)
        },
        tier:MainProgression(Tier2(PartAssembly))
    },
    Process{
        name:"Motor",
        time:12,
        building:Assembler {
            input:(Rotor {amount:2},Some(Stator {amount:2})),
            output:(Motor {amount:1},)
        },
        tier: MainProgression(Tier4(AdvancedSteel))
    },
    Process{
        name:"Nobelisk",
        time:6,
        building:Assembler {
            input:(BlackPowder {amount:2},Some(SteelPipe {amount:2})),
            output:(Nobelisk {amount:1},)
        },
        tier:MAM(SulfurTier(NobeliskDetonator))
    },
    Process{
        name:"Pressure Conversion Cube",
        time:60,
        building:Assembler {
            input:(FusedModularFrame {amount:1},Some(RadioControlUnit {amount:2})),
            output:(PressureConversionCube {amount:1},)
        },
        tier:MainProgression(Tier8(ParticleEnrichment))
    },
    Process {
        name: "Pulse Nobelisk",
        time: 60,
        building: Assembler {
            input: (Nobelisk { amount: 5 }, Some(CrystalOscillator { amount: 1 })),
            output: (PulseNobelisk { amount: 5 },)
        },
        tier:MAM(Quartz(ExplosiveResonanceApplication))
    },
    Process{
        name:"Reinforced Iron Plate",
        time:12,
        building:Assembler {
            input:(FePlate {amount:6},Some(Screws {amount:12})),
            output:(ReinforcedIronPlate {amount:1},)
        },
        tier:MainProgression(Tier0(HubUpgrade3))
    },
    Process{
        name:"Rifle Ammo",
        time:12,
        building:Assembler {
            input:(CuSheet {amount:3},Some(SmokelessPowder {amount:2})),
            output:(RifleAmmo {amount:15},)
        },
        tier:MAM(SulfurTier(Rifle))
    },
    Process{
        name:"Rotor",
        time:15,
        building:Assembler {
            input:(FeRod {amount:5},Some(Screws {amount:25})),
            output:(Rotor {amount:1},)
        },
        tier:MainProgression(Tier2(PartAssembly))
    },
    Process{
        name:"Shatter Rebar",
        time:12,
        building:Assembler {
            input:(BaseRebar {amount:2},Some(CrushedQuartz {amount:3})),
            output:(ShatterRebar {amount:1},)
        },
        tier:MAM(Quartz(ShatterRebarResearch))
    },
    Process{
        name:"Smart Plating",
        time:30,
        building:Assembler {
            input:(ReinforcedIronPlate {amount:1},Some(Rotor {amount:1})),
            output:(SmartPlating {amount:1},)
        },
        tier:MainProgression(Tier2(PartAssembly))
    },
    Process{
        name:"Sparkly Fireworks",
        time:24,
        building:Assembler {
            input:(FicsmasBranch {amount:3},Some(ActualSnow {amount:2})),
            output:(SparklyFireworks {amount:1},)
        },
        tier:MAM(Ficsmas(Fireworks))
    },
    Process{
        name:"Stator",
        time:12,
        building:Assembler {
            input:(SteelPipe {amount:3},Some(CuWire {amount:8})),
            output:(Stator {amount:1},)
        },
        tier:MainProgression(Tier4(AdvancedSteel))
    },
    Process{
        name:"Stun Rebar",
        time:6,
        building:Assembler {
            input:(BaseRebar {amount:1},Some(Quickwire {amount:5})),
            output:(PulseRebar {amount:1},)
        },
        tier:MAM(Caterium(StunRebarResearch))
    },
    Process{
        name:"Sweet Fireworks",
        time:24,
        building:Assembler {
            input:(FicsmasBranch {amount:6},Some(CandyCane {amount:3})),
            output:(SweetFireworks {amount:1},)
        },
        tier:MAM(Ficsmas(Fireworks))
    },
    Process{
        name:"Versatile Framework",
        time:24,
        building:Assembler {
            input:(ModularFrame {amount:1},Some(SteelBeam {amount:12})),
            output:(VersatileFramework {amount:2},)
        },
        tier:MainProgression(Tier3(BasicSteel))
    },
    Process{
        name:"Adhered Iron Plate",
        time:16,
        building:Assembler {
            input:(FePlate {amount:3},Some(Rubber {amount:1})),
            output:(ReinforcedIronPlate {amount:1},)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Alclad Casing",
        time:8,
        building:Assembler {
            input:(AlIngot {amount:20},Some(CuIngot {amount:10})),
            output:(AlCasing {amount:15},)
        },
        tier:HardDrive(MainUnlock(Tier7(BauxiteRefinement)))
    },
    Process{
        name:"Bolted Frame",
        time:24,
        building:Assembler {
            input:(ReinforcedIronPlate {amount:3},Some(Screws {amount:56})),
            output:(ModularFrame {amount:2},)
        },
        tier:HardDrive(MainUnlock(Tier2(PartAssembly)))
    },
    Process{
        name:"Bolted Iron Plate",
        time:12,
        building:Assembler {
            input:(FePlate {amount:18},Some(Screws {amount:50})),
            output:(ReinforcedIronPlate {amount:3},)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Caterium Circuit Board",
        time:48,
        building:Assembler {
            input:(Plastic {amount:10},Some(Quickwire {amount:30})),
            output:(CircuitBoard {amount:7},)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Caterium Circuit Board",
        time:48,
        building:Assembler {
            input:(Plastic {amount:10},Some(Quickwire {amount:30})),
            output:(CircuitBoard {amount:7},)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process{
        name:"Cheap Silica",
        time:16,
        building:Assembler {
            input:(RawQuartz {amount:3},Some(Limestone {amount:5})),
            output:(Silica {amount:7},)
        },
        tier:HardDrive(MAMUnlock(Quartz(SilicaResearch)))
    },
    Process{
        name:"Coated Iron Canister",
        time:4,
        building:Assembler {
            input:(FePlate {amount:2},Some(CuSheet {amount:1})),
            output:(EmptyCanister {amount:4},)
        },
        tier:HardDrive(MainUnlock(Tier5(AlternativeFuelTransport)))
    },
    Process{
        name:"Coated Iron Plate",
        time:12,
        building:Assembler {
            input:(FeIngot {amount:10},Some(Plastic {amount:2})),
            output:(FePlate {amount:15},)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Compacted Coal",
        time:12,
        building:Assembler {
            input:(Coal {amount:5},Some(Sulfur {amount:5})),
            output:(CompactedCoal {amount:5},)
        },
        tier:HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process{
        name:"Copper Rotor",
        time:16,
        building:Assembler {
            input:(CuSheet {amount:6},Some(Screws {amount:52})),
            output:(Rotor{amount:3},)
        },
        tier:HardDrive(MainUnlock(Tier2(PartAssembly)))
    },
    Process{
        name:"Crystal Computer",
        time:64,
        building:Assembler {
            input:(CircuitBoard {amount:8},Some(CrystalOscillator {amount:3})),
            output:(Computer {amount:3},)
        },
        tier:HardDrive(MainUnlock(Tier5(IndustrialManufacturing)))
    },
    Process{
        name:"Crystal Computer",
        time:64,
        building:Assembler {
            input:(CircuitBoard {amount:8},Some(CrystalOscillator {amount:3})),
            output:(Computer {amount:3},)
        },
        tier:HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process{
        name:"Electric Motor",
        time:16,
        building:Assembler {
            input:(EMControlRod {amount:1},Some(Rotor {amount:2})),
            output:(Motor {amount:2},)
        },
        tier:HardDrive(MainUnlock(Tier7(AeronauticalEngineering)))
    },
    Process{
        name:"Electrode Circuit Board",
        time:12,
        building:Assembler {
            input:(Rubber {amount:6},Some(PetroleumCoke {amount:9})),
            output:(CircuitBoard {amount:1},)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Electromagnetic Connection Rod",
        time:15,
        building:Assembler {
            input:(Stator {amount:2},Some(HighSpeedConnector {amount:1})),
            output:(EMControlRod {amount:2},)
        },
        tier:HardDrive(MainUnlock(Tier8(NuclearPower)))
    },
    Process{
        name:"Electromagnetic Connection Rod",
        time:15,
        building:Assembler {
            input:(Stator {amount:2},Some(HighSpeedConnector {amount:1})),
            output:(EMControlRod {amount:2},)
        },
        tier:HardDrive(MAMUnlock(Caterium(AILimiterResearch)))
    },
    Process{
        name:"Encased Industrial Pipe",
        time:15,
        building:Assembler {
            input:(SteelPipe {amount:7},Some(Concrete {amount:5})),
            output:(IndustrialBeam {amount:1},)
        },
        tier:HardDrive(MainUnlock(Tier4(AdvancedSteel)))
    },
    Process{
        name:"Fine Black Powder",
        time:16,
        building:Assembler {
            input:(Sulfur {amount:2},Some(CompactedCoal {amount:1})),
            output:(BlackPowder {amount:4},)
        },
        tier:HardDrive(MAMUnlock(SulfurTier(CompactedCoalResearch)))
    },
    Process{
        name:"Fine Concrete",
        time:24,
        building:Assembler {
            input:(Silica {amount:3},Some(Limestone {amount:12})),
            output:(Concrete {amount:10},)
        },
        tier:HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process{
        name:"Fused Quickwire",
        time:8,
        building:Assembler {
            input:(CateriumIngot {amount:1},Some(CuIngot {amount:5})),
            output:(Quickwire {amount:12},)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process{
        name:"Fused Wire",
        time:20,
        building:Assembler {
            input:(CuIngot {amount:4},Some(CateriumIngot {amount:1})),
            output:(CuWire {amount:30},)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumResearch)))
    },
    Process{
        name:"Heat Exchanger",
        time:6,
        building:Assembler {
            input:(AlCasing {amount:3},Some(Rubber {amount:3})),
            output:(Heatsink {amount:1},)
        },
        tier:HardDrive(MainUnlock(Tier8(AdvancedAluminumProduction)))
    },
    Process{
        name:"Insulated Cable",
        time:12,
        building:Assembler {
            input:(CuWire {amount:9},Some(Rubber {amount:6})),
            output:(Cable {amount:20},)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"OC Supercomputer",
        time:20,
        building:Assembler {
            input:(RadioControlUnit {amount:3},Some(CoolingSystem {amount:3})),
            output:(SuperComputer {amount:1},)
        },
        tier:HardDrive(MainUnlock(Tier7(AeronauticalEngineering)))
    },
    Process{
        name:"OC Supercomputer",
        time:20,
        building:Assembler {
            input:(RadioControlUnit {amount:3},Some(CoolingSystem {amount:3})),
            output:(SuperComputer {amount:1},)
        },
        tier:HardDrive(MainUnlock(Tier8(AdvancedAluminumProduction)))
    },
    Process{
        name:"Plutonium Fuel Unit",
        time:120,
        building:Assembler {
            input:(EncasedPuCell {amount:20},Some(PressureConversionCube {amount:1})),
            output:(PuRod {amount:1},)
        },
        tier:HardDrive(MainUnlock(Tier8(ParticleEnrichment)))
    },
    Process{
        name:"Quickwire Cable",
        time:24,
        building:Assembler {
            input:(Quickwire {amount:3},Some(Rubber{amount:2})),
            output:(Cable {amount:11},)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Quickwire Cable",
        time:24,
        building:Assembler {
            input:(Quickwire {amount:3},Some(Rubber{amount:2})),
            output:(Cable {amount:11},)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process{
        name:"Quickwire Stator",
        time:15,
        building:Assembler {
            input:(SteelPipe {amount:4},Some(Quickwire {amount:15})),
            output:(Stator{amount:2},)
        },
        tier:HardDrive(MainUnlock(Tier4(AdvancedSteel)))
    },
    Process{
        name:"Quickwire Stator",
        time:15,
        building:Assembler {
            input:(SteelPipe {amount:4},Some(Quickwire {amount:15})),
            output:(Stator{amount:2},)
        },
        tier:HardDrive(MAMUnlock(Caterium(CateriumIngotResearch)))
    },
    Process{
        name:"Rubber Concrete",
        time:12,
        building:Assembler {
            input:(Limestone {amount:10},Some(Rubber {amount:2})),
            output:(Concrete {amount:9},)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Silicon Circuit Board",
        time:24,
        building:Assembler {
            input:(CuSheet {amount:11},Some(Silica {amount:11})),
            output:(CircuitBoard {amount:5},)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Silicon Circuit Board",
        time:24,
        building:Assembler {
            input:(CuSheet {amount:11},Some(Silica {amount:11})),
            output:(CircuitBoard {amount:5},)
        },
        tier:HardDrive(MAMUnlock(Quartz(QuartzResearch)))
    },
    Process{
        name:"Steel Coated Plate",
        time:24,
        building:Assembler {
            input:(SteelIngot {amount:2},Some(Plastic{amount:2})),
            output:(FePlate {amount:18},)
        },
        tier:HardDrive(MainUnlock(Tier5(OilProcessing)))
    },
    Process{
        name:"Steel Rotor",
        time:12,
        building:Assembler {
            input:(SteelPipe {amount:2},Some(CuWire {amount:6})),
            output:(Rotor{amount:1},)
        },
        tier: HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"Steeled Frame",
        time:60,
        building:Assembler {
            input:(ReinforcedIronPlate {amount:2},Some(SteelPipe {amount:10})),
            output:(ModularFrame {amount:3},)
        },
        tier:HardDrive(MainUnlock(Tier3(BasicSteel)))
    },
    Process{
        name:"Stitched Iron Plate",
        time:32,
        building:Assembler {
            input:(FePlate {amount:10},Some(CuWire {amount:20})),
            output:(ReinforcedIronPlate {amount:3},)
        },
        tier:HardDrive(MainUnlock(Tier0(Onboarding)))
    },
    Process{
        name:"Adaptive Control Unit",
        time:120,
        building:Manufacturer {
            input:(AutomatedWiring {amount:15}, Some(CircuitBoard {amount:10}), Some(HeavyModularFrame {amount:2}), Some(Computer {amount:2})),
            output:(AdaptiveControlUnit {amount:2},)
        },
        tier:MainProgression(Tier5(IndustrialManufacturing))
    },
];
