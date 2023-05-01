#[macro_export]
macro_rules! all_dirs {
    ($struct:ident) => {
        impl $struct {
            pub fn pop(&mut self, segments: usize) {
                self.0.pop(segments)
            }

            pub fn popping(&self, segments: usize) -> Self {
                $struct(self.0.popping(segments))
            }

            pub fn join<S: $crate::PathValues>(&mut self, path: S) -> Result<()> {
                self.0.join(path)
            }

            pub fn joining<S: $crate::PathValues>(&self, path: S) -> Result<Self> {
                Ok($struct(self.0.joining(path)?))
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
