#[macro_export]
macro_rules! all_files {
    ($struct:ident) => {
        impl $struct {
            pub fn extensions(&self) -> $crate::Extensions {
                self.0.extensions()
            }

            pub fn with_extension<E: $crate::StrValues>(&self, extension: E) -> Self {
                let mut me = self.clone();
                me.0.set_extensions(extension);
                me
            }

            pub fn set_extensions<E: $crate::StrValues>(&mut self, extensions: E) {
                self.0.set_extensions(extensions);
            }

            pub fn file_name(&self) -> &str {
                self.0.file_name()
            }

            pub fn set_file_name(&mut self, file_name: &str) -> anyhow::Result<()> {
                self.0.set_file_name(file_name)
            }

            pub fn with_file_name(&self, file_name: &str) -> anyhow::Result<Self> {
                Ok(Self(self.0.with_file_name(file_name)?))
            }

            pub fn file_stem(&self) -> &str {
                self.0.file_stem()
            }

            pub fn set_file_stem(&mut self, file_stem: &str) -> anyhow::Result<()> {
                self.0.set_file_stem(file_stem)
            }

            pub fn with_file_stem(&mut self, file_stem: &str) -> anyhow::Result<Self> {
                Ok(Self(self.0.with_file_stem(file_stem)?))
            }
        }
    };
}
