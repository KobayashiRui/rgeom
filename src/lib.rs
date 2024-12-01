extern crate nalgebra as na;

pub mod plane;
pub use plane::*;

//pub mod half_edge_ds;
//pub use half_edge_ds::*;

//pub mod half_edge_ds_v2;
//pub use half_edge_ds_v2::*;
pub mod half_edge_ds_v3;
pub use half_edge_ds_v3::*;

//pub mod mesh_slice;
//pub use mesh_slice::*;

pub mod mesh_slice_v2;
pub use mesh_slice_v2::*;