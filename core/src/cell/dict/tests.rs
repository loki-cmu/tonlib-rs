// tests cover parser & builder together, so make sense to keep them in the same module
use crate::cell::dict::predefined_readers::{
    key_reader_256bit, key_reader_u16, key_reader_u32, key_reader_u64, key_reader_u8,
    key_reader_uint, val_reader_ref_cell, val_reader_uint,
};
use crate::cell::dict::predefined_writers::{val_writer_ref_cell, val_writer_unsigned_min_size};
use crate::cell::{ArcCell, BagOfCells, Cell, CellBuilder};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::ops::Deref;
use tokio_test::assert_ok;

#[test]
fn test_blockchain_data() -> anyhow::Result<()> {
    let expected_data = HashMap::from([
        (0u8, BigUint::from(25965603044000000000u128)),
        (1, BigUint::from(5173255344000000000u64)),
        (2, BigUint::from(344883687000000000u64)),
    ]);
    let boc_b64 = "te6cckEBBgEAWgABGccNPKUADZm5MepOjMABAgHNAgMCASAEBQAnQAAAAAAAAAAAAAABMlF4tR2RgCAAJgAAAAAAAAAAAAABaFhaZZhr6AAAJgAAAAAAAAAAAAAAR8sYU4eC4AA1PIC5";
    let boc = BagOfCells::parse_base64(boc_b64)?;
    let dict_cell = boc.single_root()?.reference(0)?;
    let parsed_data = assert_ok!(dict_cell
        .parser()
        .load_dict(8, key_reader_u8, val_reader_uint));
    assert_eq!(expected_data, parsed_data);

    let writer = |builder: &mut CellBuilder, val: BigUint| {
        builder.store_uint(150, &val)?; // empirically found bit length
        Ok(())
    };
    let mut builder = CellBuilder::new();
    assert_ok!(builder.store_dict(8, writer, expected_data));
    let constructed_cell = builder.build()?;
    assert_eq!(dict_cell.deref(), &constructed_cell);
    Ok(())
}

#[test]
fn test_key_len_bigger_than_reader() -> anyhow::Result<()> {
    let data = HashMap::from([
        (0u16, BigUint::from(4u32)),
        (1, BigUint::from(5u32)),
        (2, BigUint::from(6u32)),
        (10u16, BigUint::from(7u32)),
        (127, BigUint::from(8u32)),
    ]);

    for key_len_bits in [8, 16, 32, 64, 111] {
        let mut builder = CellBuilder::new();
        builder.store_dict(key_len_bits, val_writer_unsigned_min_size, data.clone())?;
        let dict_cell = builder.build()?;
        let parsed = dict_cell
            .parser()
            .load_dict(key_len_bits, key_reader_u16, val_reader_uint)?;
        assert_eq!(data, parsed, "key_len_bits: {}", key_len_bits);
    }
    Ok(())
}

#[test]
fn test_reader_u8() -> anyhow::Result<()> {
    let data = HashMap::from([
        (0u8, BigUint::from(4u32)),
        (1, BigUint::from(5u32)),
        (2, BigUint::from(6u32)),
        (64, BigUint::from(7u32)),
    ]);
    let key_len_bits = 8;
    let mut builder = CellBuilder::new();
    builder.store_dict(key_len_bits, val_writer_unsigned_min_size, data.clone())?;
    let dict_cell = builder.build()?;
    let parsed = dict_cell
        .parser()
        .load_dict(key_len_bits, key_reader_u8, val_reader_uint)?;
    assert_eq!(data, parsed);
    Ok(())
}

#[test]
fn test_reader_u16() -> anyhow::Result<()> {
    let data = HashMap::from([
        (0u16, BigUint::from(4u32)),
        (1, BigUint::from(5u32)),
        (2, BigUint::from(6u32)),
        (64, BigUint::from(7u32)),
    ]);
    let key_len_bits = 8;
    let mut builder = CellBuilder::new();
    builder.store_dict(key_len_bits, val_writer_unsigned_min_size, data.clone())?;
    let dict_cell = builder.build()?;
    let parsed = dict_cell
        .parser()
        .load_dict(key_len_bits, key_reader_u16, val_reader_uint)?;
    assert_eq!(data, parsed);
    Ok(())
}

#[test]
fn test_reader_u32() -> anyhow::Result<()> {
    let data = HashMap::from([
        (0u32, BigUint::from(4u32)),
        (1, BigUint::from(5u32)),
        (2, BigUint::from(6u32)),
        (64, BigUint::from(7u32)),
    ]);
    let key_len_bits = 8;
    let mut builder = CellBuilder::new();
    builder.store_dict(key_len_bits, val_writer_unsigned_min_size, data.clone())?;
    let dict_cell = builder.build()?;
    let parsed = dict_cell
        .parser()
        .load_dict(key_len_bits, key_reader_u32, val_reader_uint)?;
    assert_eq!(data, parsed);
    Ok(())
}

#[test]
fn test_reader_u64() -> anyhow::Result<()> {
    let data = HashMap::from([
        (0u64, BigUint::from(4u32)),
        (1, BigUint::from(5u32)),
        (2, BigUint::from(6u32)),
        (64, BigUint::from(7u32)),
    ]);
    let key_len_bits = 8;
    let mut builder = CellBuilder::new();
    builder.store_dict(key_len_bits, val_writer_unsigned_min_size, data.clone())?;
    let dict_cell = builder.build()?;
    let parsed = dict_cell
        .parser()
        .load_dict(key_len_bits, key_reader_u64, val_reader_uint)?;
    assert_eq!(data, parsed);
    Ok(())
}

#[test]
fn test_reader_256bit() -> anyhow::Result<()> {
    let data = HashMap::from([
        (0u64, BigUint::from(4u32)),
        (1u64, BigUint::from(4u32)),
        (2u64, BigUint::from(4u32)),
    ]);
    let key_len_bits = 8;
    let mut builder = CellBuilder::new();
    builder.store_dict(key_len_bits, val_writer_unsigned_min_size, data.clone())?;
    let dict_cell = builder.build()?;
    let parsed = dict_cell
        .parser()
        .load_dict(key_len_bits, key_reader_256bit, val_reader_uint)?;
    assert_eq!(parsed.len(), 3);
    Ok(())
}

#[test]
fn test_reader_uint() -> anyhow::Result<()> {
    let data = HashMap::from([
        (BigUint::from(0u32), BigUint::from(4u32)),
        (BigUint::from(1u32), BigUint::from(5u32)),
        (BigUint::from(2u32), BigUint::from(6u32)),
        (BigUint::from(64u32), BigUint::from(7u32)),
    ]);
    let key_len_bits = 8;
    let mut builder = CellBuilder::new();
    builder.store_dict(key_len_bits, val_writer_unsigned_min_size, data.clone())?;
    let dict_cell = builder.build()?;
    let parsed = dict_cell
        .parser()
        .load_dict(key_len_bits, key_reader_uint, val_reader_uint)?;
    assert_eq!(data, parsed);
    Ok(())
}

#[test]
fn test_reader_cell() -> anyhow::Result<()> {
    let data = HashMap::from([
        (
            BigUint::from(0u32),
            ArcCell::new(Cell::new(vec![0], 20, vec![], false)?),
        ),
        (
            BigUint::from(1u32),
            ArcCell::new(Cell::new(vec![1], 20, vec![], false)?),
        ),
        (
            BigUint::from(2u32),
            ArcCell::new(Cell::new(vec![2], 20, vec![], false)?),
        ),
        (
            BigUint::from(6u32),
            ArcCell::new(Cell::new(vec![6], 20, vec![], false)?),
        ),
    ]);
    let key_len_bits = 8;
    let mut builder = CellBuilder::new();
    builder.store_dict(key_len_bits, val_writer_ref_cell, data.clone())?;
    let dict_cell = builder.build()?;
    let mut parser = dict_cell.parser();
    let parsed = parser.load_dict(key_len_bits, key_reader_uint, val_reader_ref_cell)?;
    assert_eq!(data, parsed);
    Ok(())
}
