use std::collections::BTreeSet;

pub struct DictionaryGenerator {
    alphabet: Vec<char>,
    last_value: Vec<char>,
    current_value: Option<Vec<char>>,
}

impl DictionaryGenerator {
    /// # Examples
    /// 
    /// ```
    /// # use dicgen::DictionaryGenerator;
    /// let mut generator = DictionaryGenerator::new("abc", "b", "ab");
    /// 
    /// assert_eq!(generator.next(), Some("b".to_string()));
    /// assert_eq!(generator.next(), Some("c".to_string()));
    /// assert_eq!(generator.next(), Some("aa".to_string()));
    /// assert_eq!(generator.next(), Some("ab".to_string()));
    /// assert_eq!(generator.next(), None);
    /// ```
    pub fn new<A: AsRef<str>, I: AsRef<str>, E: AsRef<str>>(alphabet: A, init: I, end: E) -> Self {
        let mut alphabet: Vec<char> = BTreeSet::from_iter(alphabet.as_ref().chars()).into_iter().collect();
        let mut last_value: Vec<char> = end.as_ref().chars().rev().collect();
        let mut current_value: Vec<char> = init.as_ref().chars().rev().collect();

        alphabet.shrink_to_fit();
        last_value.shrink_to_fit();
        current_value.reserve_exact(last_value.len() - current_value.len());

        DictionaryGenerator {
            alphabet,
            last_value,
            current_value: Some(current_value),
        }
    }

    /// Use first `char` of alphabet as init for [`DictionaryGenerator::new`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use dicgen::DictionaryGenerator;
    /// let mut generator = DictionaryGenerator::new_from_start("abc", "ab");
    /// 
    /// assert_eq!(generator.next(), Some("a".to_string()));
    /// assert_eq!(generator.next(), Some("b".to_string()));
    /// assert_eq!(generator.next(), Some("c".to_string()));
    /// assert_eq!(generator.next(), Some("aa".to_string()));
    /// assert_eq!(generator.next(), Some("ab".to_string()));
    /// assert_eq!(generator.next(), None);
    /// ```
    pub fn new_from_start<A: AsRef<str>, E: AsRef<str>>(alphabet: A, end: E) -> Self {
        let init = alphabet.as_ref().chars().next().unwrap().to_string();

        DictionaryGenerator::new(alphabet, init, end)
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
        self.current_value.as_ref().map(|value| value.iter().rev().collect())
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
