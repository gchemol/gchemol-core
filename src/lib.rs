// [[file:../gchemol-core.note::*header][header:1]]
//===============================================================================#
//   DESCRIPTION:  molecule object repsented in graph data structure
//
//       OPTIONS:  ---
//  REQUIREMENTS:  ---
//         NOTES:  based on petgraph
//        AUTHOR:  Wenping Guo <ybyygu@gmail.com>
//       LICENCE:  GPL version 3
//       CREATED:  <2018-04-12 Thu 15:48>
//       UPDATED:  <>
//===============================================================================#

#![deny(missing_docs)] // rustdoc will fail if there is missing docs
// header:1 ends here

// [[file:../gchemol-core.note::*header][header:3]]
//!# Core chemical objects for gchemol
// header:3 ends here

// [[file:../gchemol-core.note::92c9c7da][92c9c7da]]
mod atom;
mod bond;
mod data;
mod element;
mod formula;
mod lattice;
mod molecule;
mod property;

#[cfg(feature = "adhoc")]
mod clean;
#[cfg(feature = "adhoc")]
mod connect;
#[cfg(feature = "adhoc")]
mod freeze;
#[cfg(feature = "adhoc")]
mod geometry;
#[cfg(feature = "adhoc")]
mod inertia;
#[cfg(feature = "adhoc")]
mod order;
#[cfg(feature = "adhoc")]
mod ring;
#[cfg(feature = "adhoc")]
mod topology;

#[cfg(feature = "adhoc")]
pub use crate::freeze::Mask;
// 92c9c7da ends here

// [[file:../gchemol-core.note::*common][common:1]]
/// shared dependencies in crate
pub(crate) mod common {
    pub use gchemol_graph::{NodeIndex, NxGraph};
    pub use gut::prelude::*;
}
// common:1 ends here

// [[file:../gchemol-core.note::*exports][exports:1]]
pub use crate::atom::*;
pub use crate::bond::*;
pub use crate::element::*;
pub use crate::lattice::*;
pub use crate::molecule::*;

pub use crate::property::PropertyStore;

pub use neighbors;
// exports:1 ends here
