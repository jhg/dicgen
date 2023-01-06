use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum DictionaryGeneratorError {
    AlphabetEmpty,
}

impl Display for DictionaryGeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DictionaryGeneratorError::AlphabetEmpty => write!(f, "Alphabet is empty, then combinations can't be generated"),
        }
    }
}

impl Error for DictionaryGeneratorError {}
