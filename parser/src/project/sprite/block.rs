use super::op_codes::OpCode;
use enum_as_inner::EnumAsInner;
use indexmap::IndexMap;
use serde::{
    de::{self, IntoDeserializer},
    Deserialize,
};
use serde_repr::Deserialize_repr;
use std::{fmt::Debug, str::FromStr};

#[derive(Debug, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum ShadowState {
    Shadow = 1,
    NoShadow = 2,
    Obscured = 3,
}

#[derive(Debug, Clone, EnumAsInner)]
pub enum Shadow {
    Number(f32),
    PositiveNumber(f32),
    PositiveInteger(u32),
    Integer(i32),
    Angle(f32),
    Color(u32),
    String(String),
}

fn value_to<T>(value: &serde_json::Value) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    value.as_str().unwrap().parse::<T>().unwrap()
}

// TODO: clean up this massive mess of not handling errors
impl<'de> serde::Deserialize<'de> for Shadow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        Ok(
            match value
                .get(0)
                .and_then(serde_json::Value::as_u64)
                .ok_or_else(|| de::Error::custom("test"))?
            {
                4 => Shadow::Number(value_to::<f32>(value.get(1).unwrap())),
                5 => Shadow::PositiveNumber(value_to::<f32>(value.get(1).unwrap())),
                6 => Shadow::PositiveInteger(value_to::<u32>(value.get(1).unwrap())),
                7 => Shadow::Integer(value_to::<i32>(value.get(1).unwrap())),
                8 => Shadow::Angle(value_to::<f32>(value.get(1).unwrap())),
                10 => Shadow::String(value.get(1).unwrap().as_str().unwrap().to_string()),
                n => panic!("unknown type: {}", n),
            },
        )
    }
}

#[derive(Debug, Clone, EnumAsInner)]
pub enum InputData {
    BlockRef(String),
    Shadow(Shadow),
}

impl<'de> serde::Deserialize<'de> for InputData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        match value {
            serde_json::Value::String(block_ref) => Ok(InputData::BlockRef(block_ref)),
            serde_json::Value::Array(array) => Ok(Shadow::deserialize(array.into_deserializer())
                .map(InputData::Shadow)
                .unwrap()),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub shadow_state: ShadowState,
    pub data: Vec<InputData>,
}

impl<'de> serde::Deserialize<'de> for Input {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        let arr = match value {
            serde_json::Value::Array(arr) => arr,
            _ => unimplemented!(),
        };
        let mut iter = arr.into_iter();
        let shadow_state =
            ShadowState::deserialize(iter.next().unwrap().into_deserializer()).unwrap();
        let data = Vec::deserialize(iter.collect::<Vec<_>>().into_deserializer()).unwrap();

        Ok(Input { shadow_state, data })
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub opcode: OpCode,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub inputs: IndexMap<String, Input>,
    pub shadow: bool,
    pub top_level: bool,
    pub x: Option<i32>,
    pub y: Option<i32>,
}
