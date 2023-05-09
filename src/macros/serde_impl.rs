#[macro_export]
macro_rules! serde_exist {
    ($struct:ident) => {
        pub mod exist {
            use serde::{de::Error, Deserialize, Deserializer, Serializer};
            use $crate::TryExist;

            pub fn serialize<S: Serializer>(
                path: &super::$struct,
                ser: S,
            ) -> Result<S::Ok, S::Error> {
                ser.serialize_str(&path.0.debug_string())
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
            use serde::{de::Error, Deserialize, Deserializer, Serializer};
            use $crate::TryExist;

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

#[macro_export]
macro_rules! serde_impl {
    ($struct:ident) => {
        impl serde::Serialize for $struct {
            fn serialize<S: serde::Serializer>(
                &self,
                ser: S,
            ) -> std::result::Result<S::Ok, S::Error> {
                ser.serialize_str(&self.0.debug_string())
            }
        }

        impl<'de> serde::Deserialize<'de> for $struct {
            fn deserialize<D: serde::Deserializer<'de>>(
                des: D,
            ) -> std::result::Result<Self, D::Error> {
                let path = String::deserialize(des)?;
                $struct::try_from(path).map_err(serde::de::Error::custom)
            }
        }
    };
}

#[test]
fn test_serde() {}
