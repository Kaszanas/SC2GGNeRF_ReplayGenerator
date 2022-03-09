use num_traits::FromPrimitive;
use rust_sc2::ids::UnitTypeId;

pub fn get_units_and_structures() -> Vec<UnitTypeId> {
    let mut result_vec = vec![];

    let continue_vec = vec![
        UnitTypeId::NydusCanal,
        UnitTypeId::ObserverSiegeMode,
        UnitTypeId::OverseerSiegeMode,
        UnitTypeId::OverlordCocoon,
        UnitTypeId::LiberatorAG,
        UnitTypeId::RavagerBurrowed,
        UnitTypeId::RavagerCocoon,
        UnitTypeId::LurkerMPBurrowed,
        UnitTypeId::LurkerMPEgg,
        UnitTypeId::WidowMineBurrowed,
        UnitTypeId::SwarmHostBurrowedMP,
        UnitTypeId::Larva,
        UnitTypeId::WarpPrismPhasing,
        UnitTypeId::UltraliskBurrowed,
        UnitTypeId::OverlordCocoon,
        UnitTypeId::QueenBurrowed,
        UnitTypeId::InfestorTerranBurrowed,
        UnitTypeId::ZerglingBurrowed,
        UnitTypeId::RoachBurrowed,
        UnitTypeId::HydraliskBurrowed,
        UnitTypeId::DroneBurrowed,
        UnitTypeId::BanelingBurrowed,
        UnitTypeId::BroodLordCocoon,
    ];

    for index in 0..2004 {
        if let Some(unit_type) = UnitTypeId::from_usize(index) {
            if unit_type.is_unit() {
                if continue_vec.contains(&unit_type) {
                    continue;
                }
                // if unit_type.is_unit() || unit_type.is_structure() {
                //     if unit_type == rust_sc2::ids::UnitTypeId::CreepTumorMissile
                //         || unit_type == rust_sc2::ids::UnitTypeId::CreepTumor
                //         || unit_type == rust_sc2::ids::UnitTypeId::CreepTumorBurrowed
                //     {
                //         continue;
                //     }
                result_vec.push(unit_type);
            }
        };
    }
    result_vec
}
