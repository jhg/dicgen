#![deny(clippy::perf)]

mod error;

use std::{collections::BTreeSet, io::Read};

pub use error::DictionaryGeneratorError;

pub struct DictionaryGenerator {
    alphabet: Vec<char>,
    last_value: Vec<char>,
    prefix: Option<String>,
    suffix: Option<String>,
    current_value: Option<Vec<char>>,
}

impl DictionaryGenerator {
    /// # Examples
    /// 
    /// ```
    /// # use dicgen::DictionaryGenerator;
    /// let mut generator = DictionaryGenerator::new("abc", "b", "ab").unwrap();
    /// 
    /// assert_eq!(generator.next(), Some("b".to_string()));
    /// assert_eq!(generator.next(), Some("c".to_string()));
    /// assert_eq!(generator.next(), Some("aa".to_string()));
    /// assert_eq!(generator.next(), Some("ab".to_string()));
    /// assert_eq!(generator.next(), None);
    /// ```
    pub fn new<A: AsRef<str>, I: AsRef<str>, E: AsRef<str>>(alphabet: A, init: I, end: E) -> Result<Self, DictionaryGeneratorError> {
        let mut alphabet: Vec<char> = BTreeSet::from_iter(alphabet.as_ref().chars()).into_iter().collect();
        let mut last_value: Vec<char> = end.as_ref().chars().rev().collect();
        let mut current_value: Vec<char> = init.as_ref().chars().rev().collect();

        alphabet.shrink_to_fit();
        last_value.shrink_to_fit();
        current_value.reserve_exact(last_value.len() - current_value.len());

        Ok(DictionaryGenerator {
            alphabet,
            last_value,
            prefix: None,
            suffix: None,
            current_value: Some(current_value),
        })
    }

    /// Use first `char` of alphabet as init for [`DictionaryGenerator::new`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use dicgen::DictionaryGenerator;
    /// let mut generator = DictionaryGenerator::new_from_start("abc", "ab").unwrap();
    /// 
    /// assert_eq!(generator.next(), Some("a".to_string()));
    /// assert_eq!(generator.next(), Some("b".to_string()));
    /// assert_eq!(generator.next(), Some("c".to_string()));
    /// assert_eq!(generator.next(), Some("aa".to_string()));
    /// assert_eq!(generator.next(), Some("ab".to_string()));
    /// assert_eq!(generator.next(), None);
    /// ```
    pub fn new_from_start<A: AsRef<str>, E: AsRef<str>>(alphabet: A, end: E) -> Result<Self, DictionaryGeneratorError> {
        let Some(init) = alphabet.as_ref().chars().next() else {
            return Err(DictionaryGeneratorError::AlphabetEmpty);
        };

        DictionaryGenerator::new(alphabet, init.to_string(), end)
    }

    pub fn with_prefix(self, prefix: &str) -> Self {
        if prefix.is_empty() {
            return self;
        }
        Self {
            prefix: Some(prefix.to_string()),
            ..self
        }
    }

    pub fn with_suffix(self, suffix: &str) -> Self {
        if suffix.is_empty() {
            return self;
        }
        Self {
            suffix: Some(suffix.to_string()),
            ..self
        }
    }

    #[inline]
    fn is_last(&self) -> bool {
        self.current_value.as_ref()
        .map(|current_value| current_value == &self.last_value)
        .unwrap_or(false)
    }

    #[inline]
    fn update(&mut self) {
        if self.is_last() {
            self.current_value = None;
        }
        let Some(current_value) = self.current_value.as_mut() else {
            return;
        };
        let mut current_offset = 0;
        loop {
            let offset_value = current_value[current_offset];
            if let Some(next_value) = self.alphabet.iter().skip_while(|&value| value != &offset_value).nth(1) {
                current_value[current_offset] = *next_value;
                break;
            }
            // Carriage.
            let first_letter = self.alphabet[0];
            current_value[current_offset] = first_letter;
            if current_offset == current_value.len() - 1 {
                current_value.push(first_letter);
                break;
            }
            current_offset += 1;
        }
    }

    #[inline]
    fn current(&self) -> Option<String> {
        let mut current = String::with_capacity(self.last_value.len());
        self.current_in(&mut current)?;
        Some(current)
    }

    #[inline]
    fn current_in(&self, current: &mut String) -> Option<()> {
        current.clear();

        if let Some(prefix) = &self.prefix {
            current.push_str(prefix.as_str());
        }
        for &char in self.current_value.as_ref()?.iter().rev() {
            current.push(char);
        }
        if let Some(suffix) = &self.suffix {
            current.push_str(suffix.as_str());
        }

        Some(())
    }

    #[inline]
    pub fn next_in(&mut self, current: &mut String) -> Option<()> {
        self.current_in(current)?;
        self.update();
        Some(())
    }

    pub fn reset_starting_in(&mut self, init: &str) {
        let mut current_value = self.current_value.take().unwrap();
        current_value.clear();
        current_value.extend(init.chars().rev());
        self.current_value = Some(current_value);
    }
}

impl Iterator for DictionaryGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current();
        self.update();
        
        current
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let current_value_len = self.current_value.as_ref().map(|value| value.len()).unwrap_or(self.last_value.len());

        let mut max_possible_values = 0;
        for i in current_value_len..=self.last_value.len() {
            max_possible_values += self.alphabet.len().pow(i as u32);
        }

        let min_possible_values = if self.current_value.is_none() {
            0
        } else {
            1
        };

        (min_possible_values, Some(max_possible_values))
    }
}

impl Read for DictionaryGenerator {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let Some(current_chars) = self.current_value.as_ref() else {
            return Ok(0);
        };

        let endline_len = '\n'.len_utf8();
        if buf.len() < current_chars.iter().map(|char| char.len_utf8()).sum::<usize>() + endline_len {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Buffer is too small"));
        }

        let mut wrote_bytes = 0;
        if let Some(prefix) = &self.prefix {
            let prefix_len = prefix.as_bytes().len();
            buf[..prefix_len].copy_from_slice(prefix.as_bytes());
            wrote_bytes += prefix_len;
        }
        for char in current_chars.iter().rev() {
            char.encode_utf8(&mut buf[wrote_bytes..]);
            wrote_bytes += char.len_utf8();
        }
        if let Some(suffix) = &self.suffix {
            let suffix_len = suffix.as_bytes().len();
            buf[wrote_bytes..wrote_bytes+suffix_len].copy_from_slice(suffix.as_bytes());
            wrote_bytes += suffix.as_bytes().len();
        }

        '\n'.encode_utf8(&mut buf[wrote_bytes..]);
        wrote_bytes += endline_len;

        self.update();

        Ok(wrote_bytes)
    }
}

#[cfg(test)]
mod test {

}
