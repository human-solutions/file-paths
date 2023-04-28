#[macro_export]
macro_rules! all_dirs {
    ($struct:ident) => {
        impl $struct {
            pub fn push<S: $crate::StringValues>(&mut self, segments: S) -> Result<()> {
                self.0.push_segments(segments)
            }

            pub fn pushing<S: $crate::StringValues>(&self, segments: S) -> Result<Self> {
                Ok($struct(self.0.pushing_segments(segments)?))
            }

            pub fn pop(&mut self) {
                self.0.pop_last_segment()
            }

            pub fn popping(&self) -> Self {
                $struct(self.0.popping_last_segment())
            }

            pub fn join(&mut self, folder: $crate::RelativeFolderPath) {
                self.0.join(&folder);
            }
        }
    };
}
