use super::Segments;

pub struct Extensions<'a> {
    parts: Segments<'a>,
}

impl<'a> Extensions<'a> {
    pub(crate) fn new(file: &'a str) -> Self {
        let parts = if let Some(idx) = index_after_first_dot(file) {
            let mut lengths = Vec::new();
            file[idx..]
                .split_inclusive('.')
                .filter(|s| !s.is_empty())
                .map(|s| s.len())
                .fold(0, |acc, val| {
                    lengths.push(acc + val);
                    acc + val
                });

            Segments::new_with_lengths(&file[idx..], lengths)
        } else {
            Segments::new_with_lengths(file, vec![])
        };
        Self { parts }
    }
}

impl<'a> Iterator for Extensions<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.parts.next()
    }
}

fn index_after_first_dot(s: &str) -> Option<usize> {
    if let Some(idx) = s.find('.') {
        let idx = idx + 1;
        if idx >= s.len() {
            None
        } else {
            Some(idx)
        }
    } else {
        None
    }
}
