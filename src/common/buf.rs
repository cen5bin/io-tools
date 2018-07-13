#![allow(dead_code)]

use std::mem;
use error::*;
use std::mem::ManuallyDrop;
use std::marker::PhantomData;

pub struct ByteBuffer<'a> {
    ptr: *const u8,
    pos: usize,
    limit: usize,
    capacity: usize,
    _phantom_data: PhantomData<&'a ()>,
}

macro_rules! read_at_func {
    ($obj:ident, $ty:ty, $of:ident) => {
        if $obj.limit < ::std::mem::size_of::<$ty>() + $of {
            Err(IOError::create_buffer_overflow_err())
        } else {
            let val = unsafe { *($obj.ptr.offset($of as isize) as *const $ty) };
            Ok(val)
        }
    };
}

macro_rules! read_func {
    ($obj:ident, $ty:ty) => {
        if $obj.remaining() < ::std::mem::size_of::<$ty>() {
            Err(IOError::create_buffer_overflow_err())
        } else {
            let val = unsafe { *($obj.ptr.offset($obj.pos as isize) as *const $ty) };
            $obj.pos += ::std::mem::size_of::<$ty>();
            Ok(val)
        }
    };
}

macro_rules! write_func {
    ($obj:ident, $ty:ty, $da:ident) => {
        if $obj.remaining() < ::std::mem::size_of::<$ty>() {
            Err(IOError::create_buffer_overflow_err())
        } else {
            unsafe {
                let src = &$da as *const $ty as *const u8;
                let dst = $obj.ptr.offset($obj.pos as isize) as *mut u8;
                ::std::intrinsics::copy_nonoverlapping(src, dst, ::std::mem::size_of::<$ty>());
                $obj.pos += ::std::mem::size_of::<$ty>();
            }
            Ok(())
        }
    };
}

macro_rules! write_at_func {
    ($obj:ident, $ty:ty, $of:ident, $da:ident) => {
        if $obj.limit < ::std::mem::size_of::<$ty>() + $of {
            Err(IOError::create_buffer_overflow_err())
        } else {
            unsafe {
                let src = &$da as *const $ty as *const u8;
                let dst = $obj.ptr.offset($of as isize) as *mut u8;
                ::std::intrinsics::copy_nonoverlapping(src, dst, ::std::mem::size_of::<$ty>());
            }
            Ok(())
        }
    };
}

impl<'a> ByteBuffer<'a> {
    pub fn with_capacity(capacity: usize) -> Self {
        let buf = Vec::with_capacity(capacity);
        let ptr = buf.as_ptr();
        mem::forget(buf);
        ByteBuffer {
            ptr,
            pos: 0,
            limit: capacity,
            capacity,
            _phantom_data: PhantomData,
        }
    }
    pub fn wrap(buf: &'a Vec<u8>) -> ManuallyDrop<Self> {
        ManuallyDrop::new(ByteBuffer {
            ptr: buf.as_ptr(),
            pos: 0,
            limit: buf.len(),
            capacity: buf.len(),
            _phantom_data: PhantomData,
        })
    }
    pub fn into_vec(self) -> Vec<u8> {
        unsafe { Vec::from_raw_parts(self.ptr as *mut u8, self.limit, self.limit) }
    }
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts(self.ptr, self.limit)
        }
    }
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            ::std::slice::from_raw_parts_mut(self.ptr as *mut u8, self.limit)
        }
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }
    pub fn as_ptr_mut(&self) -> *mut u8 {
        self.ptr as *mut u8
    }

    pub fn read_u8(&mut self) -> IOResult<u8> { read_func!(self, u8) }
    pub fn read_u16(&mut self) -> IOResult<u16> { read_func!(self, u16) }
    pub fn read_u32(&mut self) -> IOResult<u32> { read_func!(self, u32) }
    pub fn read_u64(&mut self) -> IOResult<u64> { read_func!(self, u64) }
    pub fn read_i8(&mut self) -> IOResult<i8> { read_func!(self, i8) }
    pub fn read_i16(&mut self) -> IOResult<i16> { read_func!(self, i16) }
    pub fn read_i32(&mut self) -> IOResult<i32> { read_func!(self, i32) }
    pub fn read_i64(&mut self) -> IOResult<i64> { read_func!(self, i64) }
    pub fn read_f32(&mut self) -> IOResult<f32> { read_func!(self, f32) }
    pub fn read_f64(&mut self) -> IOResult<f64> { read_func!(self, f64) }
    pub fn read_u8_at(&self, offset: usize) -> IOResult<u8> { read_at_func!(self, u8, offset) }
    pub fn read_u16_at(&self, offset: usize) -> IOResult<u16> { read_at_func!(self, u16, offset) }
    pub fn read_u32_at(&self, offset: usize) -> IOResult<u32> { read_at_func!(self, u32, offset) }
    pub fn read_u64_at(&self, offset: usize) -> IOResult<u64> { read_at_func!(self, u64, offset) }
    pub fn read_i8_at(&self, offset: usize) -> IOResult<i8> { read_at_func!(self, i8, offset) }
    pub fn read_i16_at(&self, offset: usize) -> IOResult<i16> { read_at_func!(self, i16, offset) }
    pub fn read_i32_at(&self, offset: usize) -> IOResult<i32> { read_at_func!(self, i32, offset) }
    pub fn read_i64_at(&self, offset: usize) -> IOResult<i64> { read_at_func!(self, i64, offset) }
    pub fn read_f32_at(&self, offset: usize) -> IOResult<f32> { read_at_func!(self, f32, offset) }
    pub fn read_f64_at(&self, offset: usize) -> IOResult<f64> { read_at_func!(self, f64, offset) }
    pub fn write_u8(&mut self, data: u8) -> IOResult<()> { write_func!(self, u8, data) }
    pub fn write_u16(&mut self, data: u16) -> IOResult<()> { write_func!(self, u16, data) }
    pub fn write_u32(&mut self, data: u32) -> IOResult<()> { write_func!(self, u32, data) }
    pub fn write_u64(&mut self, data: u64) -> IOResult<()> { write_func!(self, u64, data) }
    pub fn write_i8(&mut self, data: i8) -> IOResult<()> { write_func!(self, i8, data) }
    pub fn write_i16(&mut self, data: i16) -> IOResult<()> { write_func!(self, i16, data) }
    pub fn write_i32(&mut self, data: i32) -> IOResult<()> { write_func!(self, i32, data) }
    pub fn write_i64(&mut self, data: i64) -> IOResult<()> { write_func!(self, i64, data) }
    pub fn write_f32(&mut self, data: f32) -> IOResult<()> { write_func!(self, f32, data) }
    pub fn write_f64(&mut self, data: f64) -> IOResult<()> { write_func!(self, f64, data) }
    pub fn write_u8_at(&mut self, offset: usize, data: u8) -> IOResult<()> { write_at_func!(self, u8, offset, data) }
    pub fn write_u16_at(&mut self, offset: usize, data: u16) -> IOResult<()> { write_at_func!(self, u16, offset, data) }
    pub fn write_u32_at(&mut self, offset: usize, data: u32) -> IOResult<()> { write_at_func!(self, u32, offset, data) }
    pub fn write_u64_at(&mut self, offset: usize, data: u64) -> IOResult<()> { write_at_func!(self, u64, offset, data) }
    pub fn write_i8_at(&mut self, offset: usize, data: i8) -> IOResult<()> { write_at_func!(self, i8, offset, data) }
    pub fn write_i16_at(&mut self, offset: usize, data: i16) -> IOResult<()> { write_at_func!(self, i16, offset, data) }
    pub fn write_i32_at(&mut self, offset: usize, data: i32) -> IOResult<()> { write_at_func!(self, i32, offset, data) }
    pub fn write_i64_at(&mut self, offset: usize, data: i64) -> IOResult<()> { write_at_func!(self, i64, offset, data) }
    pub fn write_f32_at(&mut self, offset: usize, data: f32) -> IOResult<()> { write_at_func!(self, f32, offset, data) }
    pub fn write_f64_at(&mut self, offset: usize, data: f64) -> IOResult<()> { write_at_func!(self, f64, offset, data) }
    pub fn read_varint(&mut self) -> IOResult<i64> {
        let mut ret = 0;
        let mut bits = 0;
        let mut p = self.pos;
        loop {
            let x = self.read_u8_at(p)?;
            p += 1;
            ret = ((x & 127) as i64) << bits | ret;
            bits += 7;
            if (x & 128) == 0 {
                self.pos = p;
                return Ok(ret);
            }
        }
    }
    pub fn read_varint_at(&self, offset: usize) -> IOResult<i64> {
        let mut ret = 0;
        let mut p = offset;
        let mut bits = 0;
        loop {
            let x = self.read_u8_at(p)?;
            p += 1;
            ret = ((x & 127) as i64) << bits | ret;
            bits += 7;
            if (x & 128) == 0 {
                return Ok(ret);
            }
        }
    }
    pub fn write_varint(&mut self, data: i64) -> IOResult<()> {
        if self.remaining() < varint_len(data) {
            Err(IOError::create_buffer_overflow_err())
        } else {
            let mut x = data;
            while x > 127 {
                self.write_u8((x & 127 | 128) as u8)?;
                x >>= 7;
            }
            let ret = self.write_u8(x as u8);
            ret
        }
    }
    pub fn write_varint_at(&mut self, offset: usize, data: i64) -> IOResult<()> {
        if self.limit < varint_len(data) + offset {
            Err(IOError::create_buffer_overflow_err())
        } else {
            let mut p = offset;
            let mut x = data;
            while x > 127 {
                self.write_u8_at(p, (x & 127 | 128) as u8)?;
                p += 1;
                x >>= 7;
            }
            self.write_u8_at(p, x as u8)
        }
    }
    pub fn read_all(&mut self) -> IOResult<&[u8]> {
        unsafe {
            Ok(::std::slice::from_raw_parts(self.ptr, self.remaining()))
        }
    }
    pub fn read_all_at(&self, offset: usize) -> IOResult<&[u8]> {
        if offset > self.limit {
            Err(IOError::create_buffer_overflow_err())
        } else {
            unsafe {
                Ok(::std::slice::from_raw_parts(self.ptr.offset(offset as isize), self.limit - offset))
            }
        }
    }
    pub fn read_bytes(&mut self, len: usize) -> IOResult<&[u8]> {
        if self.remaining() < len {
            Err(IOError::create_buffer_overflow_err())
        } else {
            unsafe {
                let val = ::std::slice::from_raw_parts(self.ptr.offset(self.pos as isize), len);
                self.pos += len;
                Ok(val)
            }
        }
    }
    pub fn read_bytes_at(&self, offset: usize, len: usize) -> IOResult<&[u8]> {
        if self.limit < len + offset {
            Err(IOError::create_buffer_overflow_err())
        } else {
            unsafe {
                let val = ::std::slice::from_raw_parts(self.ptr.offset(offset as isize), len);
                Ok(val)
            }
        }
    }
    pub fn write_bytes(&mut self, bytes: &[u8]) -> IOResult<()> {
        if self.remaining() < bytes.len() {
            Err(IOError::create_buffer_overflow_err())
        } else {
            unsafe {
                let src = bytes.as_ptr();
                let dst = self.ptr.offset(self.pos as isize) as *mut u8;
                ::std::intrinsics::copy_nonoverlapping(src, dst, bytes.len());
                self.pos += bytes.len();
            }
            Ok(())
        }
    }
    pub fn write_bytes_at(&mut self, offset: usize, bytes: &[u8]) -> IOResult<()> {
        if self.limit < bytes.len() + offset {
            Err(IOError::create_buffer_overflow_err())
        } else {
            unsafe {
                let src = bytes.as_ptr();
                let dst = self.ptr.offset(offset as isize) as *mut u8;
                ::std::intrinsics::copy_nonoverlapping(src, dst, bytes.len());
            }
            Ok(())
        }
    }
    pub fn read_str(&mut self) -> IOResult<&str> {
        let len = self.read_varint_at(self.pos)?;
        let var_len = varint_len(len);
        let data = self.read_bytes_at(self.pos + var_len, len as usize)?;
        let ret = ::std::str::from_utf8(data)?;
        self.unsafe_set_position(self.pos + var_len + len as usize);
        Ok(ret)
    }
    pub fn read_str_at(&self, offset: usize) -> IOResult<&str> {
        let len = self.read_varint_at(offset)?;
        let var_len = varint_len(len);
        let data = self.read_bytes_at(offset + var_len, len as usize)?;
        let ret = ::std::str::from_utf8(data)?;
        Ok(ret)
    }
    pub fn write_str(&mut self, data: &str) -> IOResult<()> {
        let len = data.len() as i64;
        let var_len = varint_len(len);
        if self.remaining() < var_len + len as usize {
            Err(IOError::create_buffer_overflow_err())
        } else {
            self.write_varint(len)?;
            self.write_bytes(data.as_ref())?;
            Ok(())
        }
    }
    pub fn write_str_at(&mut self, offset: usize, data: &str) -> IOResult<()> {
        let len = data.len() as i64;
        let var_len = varint_len(len);
        if self.limit < offset + var_len + len as usize {
            Err(IOError::create_buffer_overflow_err())
        } else {
            self.write_varint_at(offset, len)?;
            self.write_bytes_at(offset + var_len, data.as_ref())?;
            Ok(())
        }
    }

    pub fn remaining(&self) -> usize { self.limit - self.pos }
    pub fn has_remaining(&self) -> bool { self.remaining() > 0 }
    pub fn limit(&self) -> usize { self.limit }
    pub fn capacity(&self) -> usize { self.capacity }
    pub fn position(&self) -> usize { self.pos }

    pub fn set_limit(&mut self, limit: usize) {
        if limit > self.capacity {
            panic!("limit {} exceed capacity {}", limit, self.capacity);
        }
        self.limit = limit;
    }
    pub fn set_position(&mut self, pos: usize) {
        if pos > self.limit {
            panic!("position {} exceed limit {}", pos, self.limit);
        }
        self.pos = pos;
    }

    pub fn clear(&mut self) {
        self.pos = 0;
        self.limit = self.capacity;
    }
    pub fn flip(&mut self) {
        self.limit = self.pos;
        self.pos = 0;
    }
    pub fn compact(&mut self) {
        unsafe {
            let src = self.ptr.offset(self.pos as isize);
            let dst = self.ptr as *mut u8;
            ::std::intrinsics::copy(src, dst, self.remaining());
            self.limit = self.remaining();
            self.pos = 0;
        }
    }

    fn unsafe_set_position(&self, pos: usize) {
        unsafe {
            *(&self.pos as *const usize as *mut usize) = pos;
        }
    }
}


impl<'a> From<Vec<u8>> for ByteBuffer<'a> {
    fn from(buf: Vec<u8>) -> Self {
        let ptr = buf.as_ptr();
        let capacity = buf.capacity();
        mem::forget(buf);
        ByteBuffer {
            ptr,
            pos: 0,
            limit: capacity,
            capacity,
            _phantom_data: PhantomData,
        }
    }
}

impl<'a> Drop for ByteBuffer<'a> {
    fn drop(&mut self) {
        unsafe { Vec::from_raw_parts(self.ptr as *mut u8, self.capacity, self.capacity); }
    }
}

fn varint_len(data: i64) -> usize {
    let mut ret = 0;
    let mut x = data;
    while x > 0 {
        x >>= 7;
        ret += 1;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_rw {
        ($ty:ty, $r_func:ident, $w_func:ident, $da:expr) => {
            let mut buf = ByteBuffer::with_capacity(100);
            let data = $da as $ty;
            buf.$w_func(data).unwrap();
            buf.flip();
            let ret = buf.$r_func().unwrap();
            assert_eq!(data, ret);
        };
    }

    macro_rules! test_rw_of {
        ($ty:ty, $r_func:ident, $w_func:ident, $da:expr) => {
            let mut buf = ByteBuffer::with_capacity(100);
            let data = $da as $ty;
            buf.$w_func(20, data).unwrap();
            let ret = buf.$r_func(20).unwrap();
            assert_eq!(data, ret);
        };
    }

    #[test]
    fn test_base_rw() {
        test_rw!(i8, read_i8, write_i8, 123);
        test_rw!(i16, read_i16, write_i16, 123);
        test_rw!(i32, read_i32, write_i32, 123);
        test_rw!(i64, read_i64, write_i64, 123);
        test_rw!(u8, read_u8, write_u8, 123);
        test_rw!(u16, read_u16, write_u16, 123);
        test_rw!(u32, read_u32, write_u32, 123);
        test_rw!(u64, read_u64, write_u64, 123);
        test_rw!(f32, read_f32, write_f32, 123);
        test_rw!(f64, read_f64, write_f64, 123);

        test_rw_of!(i8, read_i8_at, write_i8_at, 123);
        test_rw_of!(i16, read_i16_at, write_i16_at, 123);
        test_rw_of!(i32, read_i32_at, write_i32_at, 123);
        test_rw_of!(i64, read_i64_at, write_i64_at, 123);
        test_rw_of!(u8, read_u8_at, write_u8_at, 123);
        test_rw_of!(u16, read_u16_at, write_u16_at, 123);
        test_rw_of!(u32, read_u32_at, write_u32_at, 123);
        test_rw_of!(u64, read_u64_at, write_u64_at, 123);
        test_rw_of!(f32, read_f32_at, write_f32_at, 123);
        test_rw_of!(f64, read_f64_at, write_f64_at, 123);
    }

    #[test]
    fn test_combination_op() {
        let mut buf = ByteBuffer::with_capacity(100);
        buf.write_i8(1).unwrap();
        buf.write_f32(1.0).unwrap();
        buf.write_u16(12).unwrap();
        assert_eq!(buf.position(), 7);
        buf.flip();
        assert_eq!(buf.position(), 0);
        assert_eq!(buf.limit(), 7);
        assert_eq!(buf.read_f32_at(1).unwrap(), 1.0);
        assert_eq!(buf.read_u16_at(5).unwrap(), 12);
        assert_eq!(buf.read_i8().unwrap(), 1);
        assert_eq!(buf.position(), 1);
        assert_eq!(buf.remaining(), 6);
        buf.compact();
        assert_eq!(buf.position(), 0);
        assert_eq!(buf.limit(), 6);
        assert_eq!(buf.remaining(), 6);
        assert_eq!(buf.read_f32_at(0).unwrap(), 1.0);
        assert_eq!(buf.read_u16_at(4).unwrap(), 12);
        buf.clear();
        assert_eq!(buf.position(), 0);
        assert_eq!(buf.limit(), 100);
        let mut buf = ByteBuffer::from(buf.into_vec());
        buf.set_limit(6);
        assert_eq!(buf.position(), 0);
        assert_eq!(buf.limit(), 6);
        assert_eq!(buf.remaining(), 6);
        assert_eq!(buf.read_f32_at(0).unwrap(), 1.0);
        assert_eq!(buf.read_u16_at(4).unwrap(), 12);
        buf.clear();
        assert_eq!(buf.position(), 0);
        assert_eq!(buf.limit(), 100);
    }

    #[test]
    fn test_varint() {
        let mut buf = ByteBuffer::with_capacity(100);
        let data = 1213145563;
        buf.write_varint(data).unwrap();
        buf.flip();
        assert_eq!(buf.read_varint().unwrap(), data);
        buf.clear();
        buf.write_varint_at(56, data).unwrap();
        assert_eq!(buf.read_varint_at(56).unwrap(), data);
    }

    #[test]
    fn test_str() {
        let mut buf = ByteBuffer::with_capacity(100);
        let data = "asdadfasaf";
        buf.write_str(data).unwrap();
        buf.flip();
        assert_eq!(buf.read_str().unwrap(), data);
        buf.clear();
        buf.write_str_at(12, data).unwrap();
        assert_eq!(buf.read_str_at(12).unwrap(), data);
    }

    #[test]
    fn test_bytes() {
        let mut buf = ByteBuffer::with_capacity(100);
        let bytes = vec![1, 3, 4];
        buf.write_bytes(bytes.as_ref()).unwrap();
        buf.flip();
        assert_eq!(Vec::from(buf.read_bytes(bytes.len()).unwrap()), bytes);
    }

    #[test]
    fn test_error() {
        let mut buf = ByteBuffer::with_capacity(100);
        assert!(buf.write_i8_at(100, 1).is_err());
        assert!(buf.write_i16_at(100, 1).is_err());
        assert!(buf.write_i32_at(100, 1).is_err());
        assert!(buf.write_i64_at(100, 1).is_err());
        assert!(buf.write_u8_at(100, 1).is_err());
        assert!(buf.write_u16_at(100, 1).is_err());
        assert!(buf.write_u32_at(100, 1).is_err());
        assert!(buf.write_u64_at(100, 1).is_err());
        assert!(buf.write_f32_at(100, 1.0).is_err());
        assert!(buf.write_f64_at(100, 1.0).is_err());
        assert!(buf.write_str_at(100, "aaaa").is_err());
    }

}
