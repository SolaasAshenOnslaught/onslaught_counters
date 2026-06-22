//
// use bevy::prelude::*;
// use crate::types::Chronolog;
//
// /// Will loop through queried chronologs to initiate their ticking.  Will only trigger the ticking
// /// of tickers that are actually present in a chronolog, not empty slots.  Look at the .tick method
// /// inside the Chronolog type for more information.
// pub fn chronolog_ticking(
//     time: Res<Time>,
//     mut logs: Query<&mut Chronolog>,
// ) {
//
//     let delta = time.delta();
//
//     for mut log in &mut logs {
//         log.tick(delta);
//     }
// }
