// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(phase)]

//! An efficient, low-level, bindless graphics API for Rust. See [the
//! blog](http://gfx-rs.github.io/) for explanations and annotated examples.

#[phase(plugin, link)] extern crate log;
extern crate libc;

extern crate device;
extern crate render;

// public re-exports
pub use render::{DeviceHelper, Renderer};
pub use render::batch;
pub use render::mesh::{Attribute, Mesh, VertexFormat};
pub use render::mesh::{Slice, ToSlice};
pub use render::mesh::SliceKind;
pub use render::state::{BlendPreset, DrawState};
pub use render::shade;
pub use render::target::{Frame, Plane};
pub use device::Device;
pub use device::{attrib, state, tex};
pub use device::as_byte_slice;
pub use device::{BufferHandle, BufferInfo, RawBufferHandle, ShaderHandle};
pub use device::{ProgramHandle, SurfaceHandle, TextureHandle};
pub use device::BufferUsage;
pub use device::{VertexCount, InstanceCount};
pub use device::PrimitiveType;
pub use device::draw::CommandBuffer;
pub use device::shade::UniformValue;
pub use device::shade::{ShaderSource, ProgramInfo};
pub use device::target::{ColorValue, ClearData, Mask, Layer, Level, Rect, Target};
pub use device::target::{COLOR, DEPTH, STENCIL};

// TODO: Remove this re-export once `gl_device` becomes a separate crate.
pub use device::gl_device::{GlDevice, GlCommandBuffer};

/// A convenient wrapper suitable for single-threaded operation.
pub struct Graphics<D, C: device::draw::CommandBuffer> {
    /// Graphics device.
    pub device: D,
    /// Renderer front-end.
    pub renderer: Renderer<C>,
    /// Hidden batch context.
    context: batch::Context,
}

impl<D: device::Device<C>, C: device::draw::CommandBuffer> Graphics<D, C> {
    /// Create a new graphics wrapper.
    pub fn new(mut device: D) -> Graphics<D, C> {
        let rend = device.create_renderer();
        Graphics {
            device: device,
            renderer: rend,
            context: batch::Context::new(),
        }
    }

    /// Create a new ref batch.
    pub fn make_batch<L, T: shade::ShaderParam<L>>(&mut self,
                      program: &ProgramHandle,
                      mesh: &Mesh,
                      slice: Slice,
                      state: &DrawState)
                      -> Result<batch::RefBatch<L, T>, batch::BatchError> {
        self.context.make_batch(program, mesh, slice, state)
    }

    /// Clear the `Frame` as the `ClearData` specifies.
    pub fn clear(&mut self, data: ClearData, mask: Mask, frame: &Frame) {
        self.renderer.clear(data, mask, frame)
    }

    /// Draw a ref batch.
    pub fn draw<'a, L, T: shade::ShaderParam<L>>(&'a mut self,
        batch: &'a batch::RefBatch<L, T>, data: &'a T, frame: &Frame) {
        self.renderer.draw(&(batch, data, &self.context), frame)
    }

    /// Submit the internal command buffer and reset for the next frame.
    pub fn end_frame(&mut self) {
        self.device.submit(self.renderer.as_buffer());
        self.renderer.reset();
    }
}
