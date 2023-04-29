#[macro_export]
macro_rules! all_dirs {
    ($struct:ident) => {
        impl $struct {
            pub fn push<S: $crate::StrValues>(&mut self, segments: S) -> Result<()> {
                self.0.push_segments(segments)
            }

            pub fn pushing<S: $crate::StrValues>(&self, segments: S) -> Result<Self> {
                Ok($struct(self.0.pushing_segments(segments)?))
            }

            pub fn pop(&mut self) {
                self.0.pop_last_segment()
            }

            pub fn popping(&self) -> Self {
                $struct(self.0.popping_last_segment())
            }

            pub fn join(&mut self, folder: &$crate::RelativeFolderPath) {
                self.0.join(&folder);
            }

            pub fn joining(&self, folder: &$crate::RelativeFolderPath) -> Self {
                $struct(self.0.joining(&folder))
            }

            pub fn parent(&self) -> Option<$struct> {
                if let Some(parent) = self.0.parent() {
                    Some($struct(parent))
                } else {
                    None
                }
            }
        }
    };
}
