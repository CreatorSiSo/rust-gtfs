use serde::{de::Visitor, Deserialize};

#[derive(Debug)]
pub struct Color {
	r: u8,
	g: u8,
	b: u8,
}

impl<'de> Deserialize<'de> for Color {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_str(ColorVisitor)
	}
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
	type Value = Color;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("hex color string")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		if v.len() != 6 {
			Err(serde::de::Error::invalid_length(6, &self))
		} else {
			let r = u8::from_str_radix(&v[0..2], 16)
				.map_err(|err| serde::de::Error::custom(err.to_string()))?;
			let g = u8::from_str_radix(&v[2..4], 16)
				.map_err(|err| serde::de::Error::custom(err.to_string()))?;
			let b = u8::from_str_radix(&v[4..6], 16)
				.map_err(|err| serde::de::Error::custom(err.to_string()))?;
			Ok(Color { r, g, b })
		}
	}
}
