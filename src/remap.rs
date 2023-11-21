use crate::{ffi, VertexStream};
use std::mem;

/// Generates a vertex remap table from the vertex buffer and an optional index buffer and returns number of unique vertices.
///
/// As a result, all vertices that are binary equivalent map to the same (new) location, with no gaps in the resulting sequence.
/// Resulting remap table maps old vertices to new vertices and can be used in `remap_vertex_buffer`/`remap_index_buffer`.
///
/// The `indices` can be `None` if the input is unindexed.
pub fn generate_vertex_remap<T>(vertices: &[T], indices: Option<&[u32]>) -> (usize, Vec<u32>) {
    generate_vertex_sized_remap(vertices, mem::size_of::<T>(), indices)
}

pub fn generate_vertex_sized_remap<T>(
    vertices: &[T],
    vertex_size: usize,
    indices: Option<&[u32]>,
) -> (usize, Vec<u32>) {
    let vertex_count = vertices.len() / (vertex_size / mem::size_of::<T>());
    let mut remap: Vec<u32> = vec![0; vertex_count];
    let vertex_count = unsafe {
        match indices {
            Some(indices) => ffi::meshopt_generateVertexRemap(
                remap.as_mut_ptr().cast(),
                indices.as_ptr().cast(),
                indices.len(),
                vertices.as_ptr().cast(),
                vertex_count,
                vertex_size,
            ),
            None => ffi::meshopt_generateVertexRemap(
                remap.as_mut_ptr(),
                std::ptr::null(),
                vertex_count,
                vertices.as_ptr().cast(),
                vertex_count,
                vertex_size,
            ),
        }
    };
    (vertex_count, remap)
}

/// Generates a vertex remap table from multiple vertex streams and an optional index buffer and returns number of unique vertices.
///
/// As a result, all vertices that are binary equivalent map to the same (new) location, with no gaps in the resulting sequence.
/// Resulting remap table maps old vertices to new vertices and can be used in `remap_vertex_buffer`/`remap_index_buffer`.
///
/// To remap vertex buffers, you will need to call `remap_vertex_buffer` for each vertex stream.
///
/// The `indices` can be `None` if the input is unindexed.
pub fn generate_vertex_remap_multi(
    vertex_count: usize,
    streams: &[VertexStream<'_>],
    indices: Option<&[u32]>,
) -> (usize, Vec<u32>) {
    let streams: Vec<ffi::meshopt_Stream> = streams
        .iter()
        .map(|stream| ffi::meshopt_Stream {
            data: stream.data.cast(),
            size: stream.size,
            stride: stream.stride,
        })
        .collect();
    let mut remap: Vec<u32> = vec![0; vertex_count];
    let vertex_count = unsafe {
        match indices {
            Some(indices) => ffi::meshopt_generateVertexRemapMulti(
                remap.as_mut_ptr(),
                indices.as_ptr(),
                indices.len(),
                vertex_count,
                streams.as_ptr(),
                streams.len(),
            ),
            None => ffi::meshopt_generateVertexRemapMulti(
                remap.as_mut_ptr(),
                std::ptr::null(),
                vertex_count,
                vertex_count,
                streams.as_ptr(),
                streams.len(),
            ),
        }
    };
    (vertex_count, remap)
}

/// Generate index buffer from the source index buffer and remap table generated by `generate_vertex_remap`.
///
/// `indices` can be `None` if the input is unindexed.
pub fn remap_index_buffer(indices: Option<&[u32]>, vertex_count: usize, remap: &[u32]) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    if let Some(indices) = indices {
        result.resize(indices.len(), 0u32);
        unsafe {
            ffi::meshopt_remapIndexBuffer(
                result.as_mut_ptr(),
                indices.as_ptr(),
                indices.len(),
                remap.as_ptr(),
            );
        }
    } else {
        result.resize(vertex_count, 0u32);
        unsafe {
            ffi::meshopt_remapIndexBuffer(
                result.as_mut_ptr(),
                std::ptr::null(),
                vertex_count,
                remap.as_ptr(),
            );
        }
    }

    result
}

/// Generate index buffer from the source index buffer and remap table generated by `generate_vertex_remap`.
///
/// `indices` can be `None` if the input is unindexed.
pub fn remap_index_buffer_in_place(indices: &mut [u32], remap: &[u32]) {
    unsafe {
        ffi::meshopt_remapIndexBuffer(
            indices.as_mut_ptr(),
            indices.as_ptr(),
            indices.len(),
            remap.as_ptr(),
        );
    }
}

/// Generates vertex buffer from the source vertex buffer and remap table generated by `generate_vertex_remap`.
pub fn remap_vertex_buffer<T: Clone + Default>(
    vertices: &[T],
    vertex_count: usize,
    remap: &[u32],
) -> Vec<T> {
    remap_vertex_buffer_sized(vertices, vertex_count, mem::size_of::<T>(), remap)
}

/// Generates vertex buffer from the source vertex buffer and remap table generated by `generate_vertex_remap`.
pub fn remap_vertex_buffer_in_place<T: Clone + Default>(
    vertices: &mut [T],
    vertex_count: usize,
    remap: &[u32],
) {
    remap_vertex_buffer_sized_in_place(vertices, vertex_count, mem::size_of::<T>(), remap)
}

pub fn remap_vertex_buffer_sized<T: Clone + Default>(
    vertices: &[T],
    vertex_count: usize,
    vertex_size: usize,
    remap: &[u32],
) -> Vec<T> {
    let mut result: Vec<T> = vec![T::default(); vertex_count * (vertex_size / mem::size_of::<T>())];
    unsafe {
        ffi::meshopt_remapVertexBuffer(
            result.as_mut_ptr().cast(),
            vertices.as_ptr().cast(),
            vertex_count,
            vertex_size,
            remap.as_ptr(),
        );
    }
    result
}

pub fn remap_vertex_buffer_sized_in_place<T: Clone + Default>(
    vertices: &mut [T],
    vertex_count: usize,
    vertex_size: usize,
    remap: &[u32],
) {
    unsafe {
        ffi::meshopt_remapVertexBuffer(
            vertices.as_mut_ptr().cast(),
            vertices.as_ptr().cast(),
            vertex_count,
            vertex_size,
            remap.as_ptr(),
        );
    }
}
