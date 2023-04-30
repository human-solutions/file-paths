#[macro_export]
macro_rules! with_file {
    ($struct:ident, $to_struct:ident) => {
        impl $struct {
            pub fn with_file(&self, file: $to_struct) -> $to_struct {
                let path = self.0.path.clone() + &file.0.path;
                let p = PathInner { path, t: self.0.t };
                $to_struct(p)
            }

            pub fn with_file_str(&self, file: &str) -> Result<$to_struct> {
                Ok(self.with_file(file.try_into()?))
            }
        }
    };
}
