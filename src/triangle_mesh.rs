use cgmath::{Vector3, Vector4};

use sys::*;
use device::Device;
use buffer::Buffer;
use geometry::Geometry;
use ::{Format, GeometryType, BufferType};

pub struct TriangleMesh<'a> {
    device: &'a Device,
    pub(crate) handle: RTCGeometry,
    pub vertex_buffer: Buffer<'a, Vector4<f32>>,
    pub index_buffer: Buffer<'a, Vector3<u32>>,
}
impl<'a> TriangleMesh<'a> {
    pub fn unanimated(device: &'a Device, num_tris: usize, num_verts: usize) -> TriangleMesh<'a> {
        let h = unsafe {
            rtcNewGeometry(device.handle, GeometryType::TRIANGLE)
        };
        let vertex_buffer = Buffer::new(device, num_verts);
        let index_buffer = Buffer::new(device, num_tris);
        unsafe {
            rtcSetGeometryBuffer(h, BufferType::VERTEX, 0, Format::FLOAT3,
                                 vertex_buffer.handle, 0, 16, num_verts);

            rtcSetGeometryBuffer(h, BufferType::INDEX, 0, Format::UINT3,
                                 index_buffer.handle, 0, 12, num_tris);
        }
        TriangleMesh {
            device: device,
            handle: h,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        }
    }
}

impl<'a> Drop for TriangleMesh<'a> {
    fn drop(&mut self) {
        unsafe {
            rtcReleaseGeometry(self.handle);
        }
    }
}

impl<'a> Geometry for TriangleMesh<'a> {
    fn handle(&self) -> RTCGeometry { self.handle }
}

