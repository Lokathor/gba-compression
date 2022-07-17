#![allow(bad_style)]

use gba_compression::*;

#[test]
fn test_BitUnPackUElemSize_elements_per_u32() {
  assert_eq!(BitUnPackUElemSize::_1.elements_per_u32(), 32);
  assert_eq!(BitUnPackUElemSize::_2.elements_per_u32(), 16);
  assert_eq!(BitUnPackUElemSize::_4.elements_per_u32(), 8);
  assert_eq!(BitUnPackUElemSize::_8.elements_per_u32(), 4);
  assert_eq!(BitUnPackUElemSize::_16.elements_per_u32(), 2);
  assert_eq!(BitUnPackUElemSize::_32.elements_per_u32(), 1);
}

#[test]
fn test_BitUnPackPElemSize_elements_per_u8() {
  assert_eq!(BitUnPackPElemSize::_1.elements_per_u8(), 8);
  assert_eq!(BitUnPackPElemSize::_2.elements_per_u8(), 4);
  assert_eq!(BitUnPackPElemSize::_4.elements_per_u8(), 2);
  assert_eq!(BitUnPackPElemSize::_8.elements_per_u8(), 1);
}

#[test]
fn test_u32_count_to_u8_count() {
  let u32_2_u8 = u32_count_to_u8_count;
  type U = BitUnPackUElemSize;
  type P = BitUnPackPElemSize;
  use BitUnPackError::*;
  const SLOP: BitUnpackResult<usize> = Err(PackedElemCountSlop);
  //
  assert_eq!(u32_2_u8(0, U::_32, P::_8), Ok(0));
  assert_eq!(u32_2_u8(0, U::_32, P::_4), Ok(0));
  assert_eq!(u32_2_u8(0, U::_32, P::_2), Ok(0));
  assert_eq!(u32_2_u8(0, U::_32, P::_1), Ok(0));
  //
  assert_eq!(u32_2_u8(1, U::_32, P::_8), Ok(1));
  assert_eq!(u32_2_u8(1, U::_32, P::_4), SLOP);
  assert_eq!(u32_2_u8(1, U::_32, P::_2), SLOP);
  assert_eq!(u32_2_u8(1, U::_32, P::_1), SLOP);
  //
  assert_eq!(u32_2_u8(2, U::_32, P::_8), Ok(2));
  assert_eq!(u32_2_u8(2, U::_32, P::_4), Ok(1));
  assert_eq!(u32_2_u8(2, U::_32, P::_2), SLOP);
  assert_eq!(u32_2_u8(2, U::_32, P::_1), SLOP);
  //
  assert_eq!(u32_2_u8(3, U::_32, P::_8), Ok(3));
  assert_eq!(u32_2_u8(3, U::_32, P::_4), SLOP);
  assert_eq!(u32_2_u8(3, U::_32, P::_2), SLOP);
  assert_eq!(u32_2_u8(3, U::_32, P::_1), SLOP);
  //
  assert_eq!(u32_2_u8(4, U::_32, P::_8), Ok(4));
  assert_eq!(u32_2_u8(4, U::_32, P::_4), Ok(2));
  assert_eq!(u32_2_u8(4, U::_32, P::_2), Ok(1));
  assert_eq!(u32_2_u8(4, U::_32, P::_1), SLOP);
  //
  assert_eq!(u32_2_u8(5, U::_32, P::_8), Ok(5));
  assert_eq!(u32_2_u8(5, U::_32, P::_4), SLOP);
  assert_eq!(u32_2_u8(5, U::_32, P::_2), SLOP);
  assert_eq!(u32_2_u8(5, U::_32, P::_1), SLOP);
  //
  assert_eq!(u32_2_u8(6, U::_32, P::_8), Ok(6));
  assert_eq!(u32_2_u8(6, U::_32, P::_4), Ok(3));
  assert_eq!(u32_2_u8(6, U::_32, P::_2), SLOP);
  assert_eq!(u32_2_u8(6, U::_32, P::_1), SLOP);
  //
  assert_eq!(u32_2_u8(7, U::_32, P::_8), Ok(7));
  assert_eq!(u32_2_u8(7, U::_32, P::_4), SLOP);
  assert_eq!(u32_2_u8(7, U::_32, P::_2), SLOP);
  assert_eq!(u32_2_u8(7, U::_32, P::_1), SLOP);
  //
  assert_eq!(u32_2_u8(8, U::_32, P::_8), Ok(8));
  assert_eq!(u32_2_u8(8, U::_32, P::_4), Ok(4));
  assert_eq!(u32_2_u8(8, U::_32, P::_2), Ok(2));
  assert_eq!(u32_2_u8(8, U::_32, P::_1), Ok(1));
  //
  assert_eq!(u32_2_u8(9, U::_32, P::_8), Ok(9));
  assert_eq!(u32_2_u8(9, U::_32, P::_4), SLOP);
  assert_eq!(u32_2_u8(9, U::_32, P::_2), SLOP);
  assert_eq!(u32_2_u8(9, U::_32, P::_1), SLOP);
}

#[test]
fn test_u8_count_to_u32_count() {
  let u8_2_u32 = u8_count_to_u32_count;
  type U = BitUnPackUElemSize;
  type P = BitUnPackPElemSize;
  use BitUnPackError::*;
  const SLOP: BitUnpackResult<usize> = Err(UnpackedElemCountSlop);
  //
  assert_eq!(u8_2_u32(0, P::_8, U::_32), Ok(0));
  assert_eq!(u8_2_u32(0, P::_8, U::_16), Ok(0));
  assert_eq!(u8_2_u32(0, P::_8, U::_8), Ok(0));
  //
  assert_eq!(u8_2_u32(1, P::_8, U::_32), Ok(1));
  assert_eq!(u8_2_u32(1, P::_8, U::_16), SLOP);
  assert_eq!(u8_2_u32(1, P::_8, U::_8), SLOP);
  //
  assert_eq!(u8_2_u32(2, P::_8, U::_32), Ok(2));
  assert_eq!(u8_2_u32(2, P::_8, U::_16), Ok(1));
  assert_eq!(u8_2_u32(2, P::_8, U::_8), SLOP);
  //
  assert_eq!(u8_2_u32(3, P::_8, U::_32), Ok(3));
  assert_eq!(u8_2_u32(3, P::_8, U::_16), SLOP);
  assert_eq!(u8_2_u32(3, P::_8, U::_8), SLOP);
  //
  assert_eq!(u8_2_u32(4, P::_8, U::_32), Ok(4));
  assert_eq!(u8_2_u32(4, P::_8, U::_16), Ok(2));
  assert_eq!(u8_2_u32(4, P::_8, U::_8), Ok(1));
}

#[test]
fn test_pack_u32_to_u8() {
  type U = BitUnPackUElemSize;
  type P = BitUnPackPElemSize;
  //
  let r = pack_u32_to_u8(&[], U::_32, P::_8);
  assert!(r.unwrap().is_empty());
  //
  let r = pack_u32_to_u8(&[1], U::_32, P::_8);
  assert_eq!(r.unwrap(), vec![1]);
  //
  let r = pack_u32_to_u8(&[1, 2], U::_32, P::_4);
  assert_eq!(r.unwrap(), vec![(2 << 4) | 1]);
  //
  let r = pack_u32_to_u8(&[1, 2, 3, 4], U::_32, P::_2);
  assert_eq!(r.unwrap(), vec![(4 << 6) | (3 << 4) | (2 << 2) | 1]);
  //
  let r = pack_u32_to_u8(&[1, 1, 0, 0, 1, 1, 1, 1], U::_32, P::_1);
  assert_eq!(r.unwrap(), vec![0b1111_0011]);
}
