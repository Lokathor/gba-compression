#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BitUnPackUElemSize {
  _1 = 1,
  _2 = 2,
  _4 = 4,
  _8 = 8,
  _16 = 16,
  _32 = 32,
}
impl BitUnPackUElemSize {
  pub const fn elements_per_u32(self) -> usize {
    32 >> (self as u32).trailing_zeros()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BitUnPackPElemSize {
  _1 = 1,
  _2 = 2,
  _4 = 4,
  _8 = 8,
}
impl BitUnPackPElemSize {
  pub const fn elements_per_u8(self) -> usize {
    8 >> (self as u32).trailing_zeros()
  }
}

pub const fn u32_count_to_u8_count(
  u32_count: usize, u_elem_size: BitUnPackUElemSize,
  p_elem_size: BitUnPackPElemSize,
) -> BitUnpackResult<usize> {
  let elements_per_u32 = u_elem_size.elements_per_u32();
  let elements_per_u8 = p_elem_size.elements_per_u8();
  let element_count = u32_count * elements_per_u32;
  if element_count % elements_per_u8 != 0 {
    Err(BitUnPackError::PackedElemCountSlop)
  } else {
    Ok(element_count / elements_per_u8)
  }
}

pub const fn u8_count_to_u32_count(
  u8_count: usize, p_elem_size: BitUnPackPElemSize,
  u_elem_size: BitUnPackUElemSize,
) -> BitUnpackResult<usize> {
  let elements_per_u32 = u_elem_size.elements_per_u32();
  let elements_per_u8 = p_elem_size.elements_per_u8();
  let element_count = u8_count * elements_per_u8;
  if element_count % elements_per_u32 != 0 {
    Err(BitUnPackError::UnpackedElemCountSlop)
  } else {
    Ok(element_count / elements_per_u32)
  }
}

/// Errors to do with `BitUnPack` related functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BitUnPackError {
  PackedElemCountSlop,
  UnpackedElemCountSlop,
}
pub type BitUnpackResult<T> = Result<T, BitUnPackError>;

/// Assumes that any offsets have been pre-applied to the `u32` data.
pub fn pack_u32_to_u8(
  u32_data: &[u32], unpack_z: BitUnPackUElemSize, pack_z: BitUnPackPElemSize,
) -> BitUnpackResult<Vec<u8>> {
  let req_capacity = u32_count_to_u8_count(u32_data.len(), unpack_z, pack_z)?;
  let mut out_buffer = Vec::with_capacity(req_capacity);
  //
  let u_mask: u32 = ((1_u64 << (unpack_z as u32)) - 1) as u32;
  let mut p_buffer: u8 = 0;
  let mut p_bits: u32 = 0;
  for mut u in u32_data.iter().copied() {
    let mut u_bits = 32;
    while u_bits > 0 {
      let temp: u8 = (u & u_mask) as u8;
      u_bits -= (unpack_z as u32);
      u = u.wrapping_shl(unpack_z as u32);
      //
      p_buffer |= (temp << p_bits);
      p_bits += pack_z as u32;
      if p_bits == 8 {
        out_buffer.push(p_buffer);
        p_buffer = 0;
        p_bits = 0;
      }
    }
  }
  debug_assert_eq!(p_buffer, 0);
  debug_assert_eq!(p_bits, 0);
  //
  Ok(out_buffer)
}
