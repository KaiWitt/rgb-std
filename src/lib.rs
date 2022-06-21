// RGB Standard Library: high-level API to RGB smart contracts.
// Written in 2019-2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// To the extent possible under law, the author(s) have dedicated all copyright
// and related and neighboring rights to this software to the public domain
// worldwide. This software is distributed without any warranty.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;

mod consignments;
mod disclosure;
mod stash;
pub mod fungible;
mod state;

pub mod prelude {
    pub use consignments::{
        AnchoredBundles, ChainIter, ConsignmentEndpoints, ConsignmentId, ConsignmentType, Contract,
        ContractConsignment, ExtensionList, InmemConsignment, MeshIter, StateTransfer,
        RGB_INMEM_CONSIGNMENT_VERSION,
    };
    pub use disclosure::{Disclosure, DisclosureId, RGB_DISCLOSURE_VERSION};
    pub use rgb_core::prelude::*;
    pub use rgb_core::{field, secp256k1zkp, type_map};
    pub use stash::Stash;
    pub use state::{AssignedState, ContractState, StateAtom};

    use super::*;
}

pub use prelude::*;
