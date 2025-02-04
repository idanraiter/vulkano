// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! Low-level builders that allow submitting an operation to a queue.
//!
//! In order to submit an operation to the GPU, you must use one of the builder structs of this
//! module. These structs are low-level and unsafe, and are mostly used to implement other parts
//! of vulkano, so you are encouraged to not use them directly.

pub use self::{
    bind_sparse::{
        SubmitBindSparseBatchBuilder, SubmitBindSparseBufferBindBuilder, SubmitBindSparseBuilder,
        SubmitBindSparseError, SubmitBindSparseImageBindBuilder,
        SubmitBindSparseImageOpaqueBindBuilder,
    },
    queue_present::{SubmitPresentBuilder, SubmitPresentError},
    queue_submit::{SubmitCommandBufferBuilder, SubmitCommandBufferError},
    semaphores_wait::SubmitSemaphoresWaitBuilder,
};

mod bind_sparse;
mod queue_present;
mod queue_submit;
mod semaphores_wait;

/// Contains all the possible submission builders.
#[derive(Debug)]
pub enum SubmitAnyBuilder<'a> {
    Empty,
    SemaphoresWait(SubmitSemaphoresWaitBuilder<'a>),
    CommandBuffer(SubmitCommandBufferBuilder<'a>),
    QueuePresent(SubmitPresentBuilder<'a>),
    BindSparse(SubmitBindSparseBuilder<'a>),
}

impl<'a> SubmitAnyBuilder<'a> {
    /// Returns true if equal to `SubmitAnyBuilder::Empty`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        matches!(self, SubmitAnyBuilder::Empty)
    }
}
