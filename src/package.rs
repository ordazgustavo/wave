use std::collections::BTreeMap;

use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer, Serialize};

use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{self, MapAccess, Visitor};
use void::Void;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub main: Option<String>,
    #[serde(
        deserialize_with = "string_or_struct",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub author: Option<Person>,
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<BTreeMap<String, String>>,
}

impl Package {
    pub fn from_json(pkg: &str) -> Result<Self> {
        serde_json::from_str(pkg).context("Reading package.json")
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Generating package.json")
    }
}

impl Default for Package {
    fn default() -> Self {
        Self {
            name: Some(String::from("")),
            version: Some(String::from("1.0.0")),
            main: Some(String::from("index.js")),
            author: None,
            license: Some(String::from("MIT")),
            dependencies: None,
            dev_dependencies: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Person {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl FromStr for Person {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Person {
            name: s.to_string(),
            email: None,
            url: None,
        })
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Option<T>, E>
        where
            E: de::Error,
        {
            Ok(Some(FromStr::from_str(value).unwrap()))
        }

        fn visit_map<M>(self, map: M) -> Result<Option<T>, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Ok(Some(
                Deserialize::deserialize(de::value::MapAccessDeserializer::new(map)).unwrap(),
            ))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
