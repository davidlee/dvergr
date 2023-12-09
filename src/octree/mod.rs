// sources: https://github.com/ybyygu/rust-octree/blob/master/src/octant.rs
// https://carloshernandezbarbera.com/octree-implementation/
// http://nomis80.org/code/octree.html
// https://www.geeksforgeeks.org/octree-insertion-and-searching/
// https://www.gamedev.net/tutorials/programming/general-and-gameplay-programming/introduction-to-octrees-r3529/
//
// https://github.com/bfops/playform/blob/019595dd1000135a3e770af3b62ac52c59a6d125/server/src/voxel_tree.rs
// https://github.com/Nercury/octree-rs
//
// https://github.com/veloren/veloren
// https://github.com/veloren/veloren/blob/master/common/src/terrain/mod.rs
// https://github.com/veloren/veloren/blob/master/common/src/terrain/block.rs#L32
// https://github.com/veloren/veloren/blob/master/common/src/terrain/map.rs
// https://github.com/veloren/veloren/blob/master/common/src/terrain/chonk.rs
// https://github.com/veloren/veloren/blob/master/common/src/volumes/chunk.rs
// https://github.com/veloren/veloren/blob/master/common/src/util/spatial_grid.rs

// http://procworld.blogspot.com

// hmmm, this seems kinda hard.

// https://eisenwave.github.io/voxel-compression-docs/svo/svo.html
//
// https://research.nvidia.com/sites/default/files/pubs/2010-02_Efficient-Sparse-Voxel/laine2010tr1_paper.pdf
//

// NOTE use extent defaults from veloren

// TODO decide what we store in the leaves: ECS identity? Custom struct? binary for type of cell? etc
// TODO keep all the things in one structure, or separate (data structures) for eg type & material, items, etc?
