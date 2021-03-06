#![allow(dead_code)]

use std::marker::PhantomData;

use CreationError;
use Format;
use FormatsEnumerationError;

#[derive(Default)]
pub struct EndpointsIterator;

impl Iterator for EndpointsIterator {
    type Item = Endpoint;

    #[inline]
    fn next(&mut self) -> Option<Endpoint> {
        None
    }
}

#[inline]
pub fn get_default_endpoint() -> Option<Endpoint> {
    None
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Endpoint;

impl Endpoint {
    #[inline]
    pub fn get_supported_formats_list(&self)
            -> Result<SupportedFormatsIterator, FormatsEnumerationError>
    {
        unreachable!()
    }

    #[inline]
    pub fn get_name(&self) -> String {
        "null".to_owned()
    }
}

pub struct SupportedFormatsIterator;

impl Iterator for SupportedFormatsIterator {
    type Item = Format;

    #[inline]
    fn next(&mut self) -> Option<Format> {
        None
    }
}

pub struct Voice;

impl Voice {
    #[inline]
    pub fn new(_: &Endpoint, _: &Format) -> Result<Voice, CreationError> {
        Err(CreationError::DeviceNotAvailable)
    }

    #[inline]
    pub fn append_data<'a, T>(&'a mut self, _: usize) -> Buffer<'a, T> {
        unreachable!()
    }

    #[inline]
    pub fn play(&mut self) {
    }

    #[inline]
    pub fn pause(&mut self) {
    }

    #[inline]
    pub fn get_period(&self) -> usize {
        0
    }

    #[inline]
    pub fn get_pending_samples(&self) -> usize {
        unreachable!()
    }

    #[inline]
    pub fn underflowed(&self) -> bool {
        false
    }
}

pub struct Buffer<'a, T: 'a> {
    marker: PhantomData<&'a T>,
}

impl<'a, T> Buffer<'a, T> {
    #[inline]
    pub fn get_buffer<'b>(&'b mut self) -> &'b mut [T] {
        unreachable!()
    }

    #[inline]
    pub fn len(&self) -> usize {
        0
    }

    #[inline]
    pub fn finish(self) {
    }
}
