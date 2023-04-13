///
#[macro_export]
macro_rules! serde_exist {
    ($struct:ident) => {
        pub mod exist {
            use crate::TryExist;
            use serde::{de::Error, Deserialize, Deserializer, Serializer};

            pub fn serialize<S: Serializer>(
                path: &super::$struct,
                ser: S,
            ) -> Result<S::Ok, S::Error> {
                ser.serialize_str(&format!("{:?}", path.0))
            }

            pub fn deserialize<'de, D: Deserializer<'de>>(
                des: D,
            ) -> Result<super::$struct, D::Error> {
                let s = String::deserialize(des).map_err(Error::custom)?;
                super::$struct::try_exist(s).map_err(Error::custom)
            }
        }
    };
}

#[macro_export]
macro_rules! serde_expanded {
    ($struct:ident) => {
        pub mod expanded {
            use crate::TryExist;
            use serde::{de::Error, Deserialize, Deserializer, Serializer};

            pub fn serialize<S: Serializer>(
                path: &super::$struct,
                ser: S,
            ) -> Result<S::Ok, S::Error> {
                ser.serialize_str(&format!("{:#?}", path.0))
            }

            pub fn deserialize<'de, D: Deserializer<'de>>(
                des: D,
            ) -> Result<super::$struct, D::Error> {
                let s = String::deserialize(des).map_err(Error::custom)?;
                super::$struct::try_exist(s).map_err(Error::custom)
            }
        }
    };
}
