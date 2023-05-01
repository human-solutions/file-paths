#[macro_export]
macro_rules! all_dirs {
    ($struct:ident) => {
        impl $struct {
            pub fn pop(&mut self) {
                self.0.pop_last_segment()
            }

            pub fn popping(&self) -> Self {
                $struct(self.0.popping_last_segment())
            }

            pub fn join<S: $crate::SegmentValues>(&mut self, folder: S) -> Result<()> {
                self.0.join(folder)
            }

            pub fn joining<S: $crate::SegmentValues>(&self, folder: S) -> Result<Self> {
                Ok($struct(self.0.joining(folder)?))
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
