#[macro_export]
macro_rules! all_files {
    ($struct:ident) => {
        impl $struct {
            pub fn extensions(&self) -> $crate::Extensions {
                self.0.extensions()
            }

            pub fn with_extension<E: $crate::FileExtensions>(&self, extension: E) -> Self {
                let mut me = self.clone();
                me.0.set_extensions(extension);
                me
            }

            pub fn set_extensions<E: $crate::FileExtensions>(&mut self, extensions: E) {
                self.0.set_extensions(extensions);
            }

            pub fn add_extension(&mut self, extension: &str) {
                self.0.add_extension(extension)
            }
        }
    };
}
