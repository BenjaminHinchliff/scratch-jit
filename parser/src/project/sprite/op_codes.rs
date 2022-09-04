use serde::{
    de::{IntoDeserializer, Visitor},
    Deserialize,
};

#[derive(Debug)]
pub enum OpCode {
    Event(Event),
    Motion(Motion),
    Argument,
    Unknown,
}

struct OpCodeVisitor;

impl<'de> Visitor<'de> for OpCodeVisitor {
    type Value = OpCode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a scratch block op code")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let (category, block) = v
            .split_once('_')
            .ok_or_else(|| E::custom(format!("invalid op code format: {}", v)))?;
        match category {
            "event" => Event::deserialize(block.into_deserializer()).map(OpCode::Event),
            "motion" => Motion::deserialize(block.into_deserializer()).map(OpCode::Motion),
            _ => Ok(OpCode::Unknown),
        }
    }
}

impl<'de> Deserialize<'de> for OpCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(OpCodeVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Event {
    WhenFlagClicked,
    #[serde(other)]
    Unknown
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Motion {
    MoveSteps,
    TurnLeft,
    GoToXY,
    PointInDirection,
    XPosition,
    YPosition,
    #[serde(other)]
    Unknown,
}
