pub(crate) trait CharExt {
    fn is_slash(&self) -> bool;
    fn is_forbidden_in_path(&self) -> bool;
    fn is_allowed_in_environment_var(&self) -> bool;
}

impl CharExt for char {
    fn is_slash(&self) -> bool {
        *self == '\\' || *self == '/'
    }

    /// https://en.wikipedia.org/wiki/Filename#Reserved_characters_and_words
    fn is_forbidden_in_path(&self) -> bool {
        #[cfg(not(feature = "strict"))]
        {
            self.is_ascii_control() || *self == ':'
        }
        #[cfg(feature = "strict")]
        {
            self.is_ascii_control()
                || [':', '"', '*', '/', '<', '>', '?', '\\', '|'].contains(&self)
        }
    }

    fn is_allowed_in_environment_var(&self) -> bool {
        self.is_ascii_uppercase() || self.is_ascii_digit() || *self == '_'
    }
}
