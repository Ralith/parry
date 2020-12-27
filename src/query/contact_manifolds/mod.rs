pub use self::contact_manifold::{ContactManifold, KinematicsCategory, TrackedContact};
pub use self::contact_manifolds_ball_ball::{
    contact_manifold_ball_ball, contact_manifold_ball_ball_shapes,
};
pub use self::contact_manifolds_capsule_capsule::{
    contact_manifold_capsule_capsule, contact_manifold_capsule_capsule_shapes,
};
pub use self::contact_manifolds_convex_ball::{
    contact_manifold_convex_ball, contact_manifold_convex_ball_shapes,
};
// pub use self::contact_manifolds_cuboid_capsule::{
//     contact_manifold_cuboid_capsule, contact_manifold_cuboid_capsule_shapes,
// };
pub use self::contact_manifolds_cuboid_cuboid::{
    contact_manifold_cuboid_cuboid, contact_manifold_cuboid_cuboid_shapes,
};
pub use self::contact_manifolds_cuboid_triangle::{
    contact_manifold_cuboid_triangle, contact_manifold_cuboid_triangle_shapes,
};
pub use self::contact_manifolds_heightfield_shape::{
    contact_manifolds_heightfield_shape, contact_manifolds_heightfield_shape_shapes,
};
pub use self::contact_manifolds_pfm_pfm::{
    contact_manifold_pfm_pfm, contact_manifold_pfm_pfm_shapes,
};
pub use self::contact_manifolds_trimesh_shape::{
    contact_manifolds_trimesh_shape, contact_manifolds_trimesh_shape_shapes,
};
pub use self::contact_manifolds_workspace::ContactManifoldsWorkspace;

pub(self) use self::contact_manifolds_heightfield_shape::HeightFieldShapeContactManifoldsWorkspace;
pub(self) use self::contact_manifolds_trimesh_shape::TriMeshShapeContactManifoldsWorkspace;
pub(self) use self::contact_manifolds_workspace::WorkspaceSerializationTag;

mod contact_manifold;
mod contact_manifolds_ball_ball;
mod contact_manifolds_capsule_capsule;
mod contact_manifolds_convex_ball;
// mod contact_manifolds_cuboid_capsule;
mod contact_manifolds_cuboid_cuboid;
mod contact_manifolds_cuboid_triangle;
mod contact_manifolds_heightfield_shape;
mod contact_manifolds_pfm_pfm;
mod contact_manifolds_trimesh_shape;
mod contact_manifolds_workspace;