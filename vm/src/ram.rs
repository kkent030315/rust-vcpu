//! This module implements a simple emulation of Dynamic Random-Access Memory
//! (DRAM).
//!
//! The DRAM in this module only stores VM bytecode or raw data and that virtual
//! DRAM does _not_ include a page table or virtual memory management, unlike
//! modern CPUs, which typically use page tables to map virtual addresses to
//! physical memory locations.
//!
//! Instead, this DRAM emulation directly maps
//! offsets to the underlying memory (backed by a `Vec<u8>`), making it a
//! simplified model for memory storage. The memory operations (reading and
//! writing 8-bit, 16-bit, 32-bit, and 64-bit values) occur directly at specific
//! offsets without any translation between virtual and physical addresses. This
//! lack of page table management means that any access beyond the allocated
//! memory space will result in an exception, such as an `AccessViolation`.
//!
//! This DRAM is not thread-safe, meaning that concurrent accesses to the memory
//! may lead to data races and undefined behavior unless proper synchronization
//! is applied.

use crate::exception::Exception;

/// The default DRAM size is set to 1Mib.
pub const DEFAULT_SIZE: usize = 1024 * 1024;

/// The DRAM storage class
#[derive(Debug, Clone)]
pub struct Dram(pub Vec<u8>);

impl Default for Dram {
    fn default() -> Self {
        Self {
            0: Vec::with_capacity(DEFAULT_SIZE),
        }
    }
}

impl Dram {
    /// Make an new instance of [`Dram`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Make an new instance of [`Dram`] with data
    #[must_use]
    pub fn with_data<S: Into<Vec<u8>>>(data: S) -> Self {
        Self { 0: data.into() }
    }

    /// Reads an 8-bit unsigned integer at the specified offset.
    ///
    /// # Arguments
    /// - `offset`: The memory offset from which to read the 8-bit value.
    ///
    /// # Returns
    /// - `Ok(u8)`: The 8-bit value read from the specified offset.
    /// - `Err(Exception::AccessViolation)`: If the offset is out of bounds.
    pub fn read_u8(&self, offset: usize) -> Result<u8, Exception> {
        const SIZE: usize = core::mem::size_of::<u8>();
        if offset + SIZE > self.0.len() {
            return Err(Exception::AccessViolation);
        }
        Ok(self.0[offset])
    }

    /// Reads a 16-bit unsigned integer at the specified offset.
    ///
    /// # Arguments
    /// - `offset`: The memory offset from which to read the 16-bit value.
    ///
    /// # Returns
    /// - `Ok(u16)`: The 8-bit value read from the specified offset.
    /// - `Err(Exception::AccessViolation)`: If the offset is out of bounds.
    pub fn read_u16le(&self, offset: usize) -> Result<u16, Exception> {
        const SIZE: usize = core::mem::size_of::<u16>();
        if offset + SIZE > self.0.len() {
            return Err(Exception::AccessViolation);
        }
        let bytes: [u8; SIZE] = self.0[offset..offset + SIZE].try_into().unwrap();
        Ok(u16::from_le_bytes(bytes).into())
    }

    /// Reads a 32-bit unsigned integer at the specified offset.
    ///
    /// # Arguments
    /// - `offset`: The memory offset from which to read the 32-bit value.
    ///
    /// # Returns
    /// - `Ok(u32)`: The 8-bit value read from the specified offset.
    /// - `Err(Exception::AccessViolation)`: If the offset is out of bounds.
    pub fn read_u32le(&self, offset: usize) -> Result<u32, Exception> {
        const SIZE: usize = core::mem::size_of::<u32>();
        if offset + SIZE > self.0.len() {
            return Err(Exception::AccessViolation);
        }
        let bytes: [u8; SIZE] = self.0[offset..offset + SIZE].try_into().unwrap();
        Ok(u32::from_le_bytes(bytes).into())
    }

    /// Reads a 64-bit unsigned integer at the specified offset.
    ///
    /// # Arguments
    /// - `offset`: The memory offset from which to read the 64-bit value.
    ///
    /// # Returns
    /// - `Ok(u64)`: The 8-bit value read from the specified offset.
    /// - `Err(Exception::AccessViolation)`: If the offset is out of bounds.
    pub fn read_u64le(&self, offset: usize) -> Result<u64, Exception> {
        const SIZE: usize = core::mem::size_of::<u64>();
        if offset + SIZE > self.0.len() {
            return Err(Exception::AccessViolation);
        }
        let bytes: [u8; SIZE] = self.0[offset..offset + SIZE].try_into().unwrap();
        Ok(u64::from_le_bytes(bytes).into())
    }

    /// Writes an 8-bit unsigned integer to the specified offset.
    ///
    /// # Arguments
    /// - `offset`: The memory offset where the value should be written.
    /// - `value`: The value to write to memory.
    ///
    /// # Returns
    /// - `Ok(())`: If the value is successfully written to the specified
    ///   offset.
    /// - `Err(Exception::AccessViolation)`: If the offset is out of bounds.
    pub fn write_u8(&mut self, offset: usize, value: u8) -> Result<(), Exception> {
        if offset >= self.0.len() {
            return Err(Exception::AccessViolation);
        }
        self.0[offset] = value;
        Ok(())
    }

    /// Writes a 16-bit unsigned integer to the specified offset.
    ///
    /// # Arguments
    /// - `offset`: The memory offset where the value should be written.
    /// - `value`: The value to write to memory.
    ///
    /// # Returns
    /// - `Ok(())`: If the value is successfully written to the specified
    ///   offset.
    /// - `Err(Exception::AccessViolation)`: If the offset is out of bounds.
    pub fn write_u16le(&mut self, offset: usize, value: u16) -> Result<(), Exception> {
        const SIZE: usize = core::mem::size_of::<u16>();
        if offset + SIZE > self.0.len() {
            return Err(Exception::AccessViolation);
        }
        self.0[offset..offset + SIZE].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    /// Writes a 32-bit unsigned integer to the specified offset.
    ///
    /// # Arguments
    /// - `offset`: The memory offset where the value should be written.
    /// - `value`: The value to write to memory.
    ///
    /// # Returns
    /// - `Ok(())`: If the value is successfully written to the specified
    ///   offset.
    /// - `Err(Exception::AccessViolation)`: If the offset is out of bounds.
    pub fn write_u32le(&mut self, offset: usize, value: u32) -> Result<(), Exception> {
        const SIZE: usize = core::mem::size_of::<u32>();
        if offset + SIZE > self.0.len() {
            return Err(Exception::AccessViolation);
        }
        self.0[offset..offset + SIZE].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    /// Writes a 64-bit unsigned integer to the specified offset.
    ///
    /// # Arguments
    /// - `offset`: The memory offset where the value should be written.
    /// - `value`: The value to write to memory.
    ///
    /// # Returns
    /// - `Ok(())`: If the value is successfully written to the specified
    ///   offset.
    /// - `Err(Exception::AccessViolation)`: If the offset is out of bounds.
    pub fn write_u64le(&mut self, offset: usize, value: u64) -> Result<(), Exception> {
        const SIZE: usize = core::mem::size_of::<u64>();
        if offset + SIZE > self.0.len() {
            return Err(Exception::AccessViolation);
        }
        self.0[offset..offset + SIZE].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }
}
