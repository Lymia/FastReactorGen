use anyhow::*;
use bitstream_io::{BitRead, BitReader, BE as BitBE};
use byteorder::*;
use num_enum::TryFromPrimitive;
use serde_json::{value::Value, Map, Number};
use std::{
    convert::TryInto,
    io::{BufRead, Cursor},
};

#[derive(TryFromPrimitive, Copy, Clone, Debug)]
#[repr(u8)]
enum DataType {
    Object = 1,
    String = 2,
    I32 = 3,
    F32 = 4,
    Bool = 5,
    I64 = 6,
    F64 = 7,
    List = 9,
    I8 = 10,
    I16 = 11,
    NumberList = 12,
}

#[derive(TryFromPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u16)]
enum Version {
    Ver0 = 0,
    Ver1 = 1,
}

fn read_jstring(data: &mut Cursor<&[u8]>) -> Result<String> {
    let length = data.read_i16::<BE>()?;
    ensure!(length >= 0, "Negative String length found.");
    let str = cesu8::from_java_cesu8(&data.fill_buf()?[..length as usize])?.to_string();
    data.set_position(data.position() + length as u64);
    Ok(str)
}
fn decode_type_tag_opt(data: &mut Cursor<&[u8]>) -> Result<Option<DataType>> {
    Ok(match data.read_u8()? {
        0 => None,
        tp => Some(tp.try_into()?),
    })
}
fn decode_type_tag(data: &mut Cursor<&[u8]>) -> Result<DataType> {
    match decode_type_tag_opt(data)? {
        Some(tp) => Ok(tp),
        None => bail!("Type tags of `0` are only valid in objects."),
    }
}

fn decode_config(data: &mut Cursor<&[u8]>, ver: Version) -> Result<Value> {
    let mut obj = Map::new();
    dbg!(ver);
    match ver {
        Version::Ver0 => {
            while let Some(tag) = decode_type_tag_opt(data)? {
                let key = read_jstring(data)?;
                let value = decode_value(tag, data, ver)?;
                obj.insert(key, value);
            }
        }
        Version::Ver1 => {
            while let Some(tag) = decode_type_tag_opt(data)? {
                let value = decode_value(tag, data, ver)?;
                let key = read_jstring(data)?;
                obj.insert(key, value);
            }
        }
    }
    Ok(Value::Object(obj))
}
fn decode_list(data: &mut Cursor<&[u8]>, ver: Version) -> Result<Value> {
    let tag_type = if ver == Version::Ver0 { 2 } else { data.read_u8()? };
    if tag_type == 0 {
        Ok(Value::Array(Vec::new()))
    } else if tag_type == 1 {
        let count = data.read_i32::<BE>()?;
        ensure!(count >= 0, "Negative list length found.");
        let tag = decode_type_tag(data)?;

        let mut vec = Vec::new();
        for _ in 0..count {
            vec.push(decode_value(tag, data, ver)?);
        }
        Ok(Value::Array(vec))
    } else {
        let mut vec = Vec::new();
        while let Some(tag) = decode_type_tag_opt(data)? {
            vec.push(decode_value(tag, data, ver)?);
        }
        Ok(Value::Array(vec))
    }
}
fn decode_number_list(data: &mut Cursor<&[u8]>, ver: Version) -> Result<Value> {
    ensure!(ver != Version::Ver0, "Number lists are not supported on version 0.");

    let size_class = data.fill_buf()?[0] >> 6;
    let len = match size_class {
        0 => (data.read_u8()? & 0x3F) as usize,
        1 => (data.read_u16::<BE>()? & 0x3FFF) as usize,
        2 => (data.read_u32::<BE>()? & 0x3FFFFFFF) as usize,
        3 => {
            data.read_u8()?;
            data.read_u32::<BE>()? as usize
        }
        _ => unreachable!(),
    };

    let mut vec = Vec::new();
    let flags = data.read_u8()?;
    if flags & 0x80 != 0 {
        // i64 only format
        for _ in 0..len {
            vec.push(Value::Number(data.read_i64::<BE>()?.into()));
        }
    } else {
        let has_neg = flags & 0x40 != 0;
        let digits = (flags & 0x3F) as u32;
        if digits == 0 {
            for _ in 0..len {
                vec.push(Value::Number(0.into()));
            }
        } else {
            let mut bits_in = BitReader::<_, BitBE>::new(data);
            for _ in 0..len {
                let is_neg = if has_neg { bits_in.read_bit()? } else { false };
                let val: i64 = bits_in.read(digits)?;
                let val = if is_neg { -val } else { val };
                vec.push(Value::Number(val.into()));
            }
        }
    }
    Ok(Value::Array(vec))
}

fn f64_to_value(f: f64) -> Result<Value> {
    Ok(Value::Number(match Number::from_f64(f) {
        Some(v) => v,
        None => bail!("f64 cannot be encoded as JSON Number."),
    }))
}
fn decode_value(tp: DataType, data: &mut Cursor<&[u8]>, ver: Version) -> Result<Value> {
    match tp {
        DataType::Object => decode_config(data, ver),
        DataType::String => Ok(Value::String(read_jstring(data)?)),
        DataType::I32 => Ok(Value::Number(data.read_i32::<BE>()?.into())),
        DataType::F32 => f64_to_value(data.read_f32::<BE>()? as f64),
        DataType::Bool => Ok(Value::Bool(data.read_u8()? != 0)),
        DataType::I64 => Ok(Value::Number(data.read_i64::<BE>()?.into())),
        DataType::F64 => f64_to_value(data.read_f64::<BE>()?),
        DataType::List => decode_list(data, ver),
        DataType::I8 => Ok(Value::Number(data.read_i8()?.into())),
        DataType::I16 => Ok(Value::Number(data.read_i16::<BE>()?.into())),
        DataType::NumberList => decode_number_list(data, ver),
    }
}

pub fn decode(data: &[u8]) -> Result<(Value, &[u8])> {
    let mut data = Cursor::new(data);
    let ver = data.read_u16::<BE>()?;
    let config = decode_config(&mut data, ver.try_into()?)?;
    let pos = data.position() as usize;
    Ok((config, &data.into_inner()[pos..]))
}
