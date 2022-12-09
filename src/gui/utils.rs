#![allow(dead_code)]

use eframe::egui::TextBuffer;
use std::{ops::Range, str::FromStr};

pub type TextEditFromStrBind<T> = TextEditBind<T, <T as FromStr>::Err>;

pub struct TextEditBind<T, E> {
    buf: String,
    value: Option<Result<T, E>>,
    convert: Box<dyn Fn(&str) -> Result<T, E> + 'static>,
}

impl<T, E> TextEditBind<T, E> {
    pub fn new(convert: impl Fn(&str) -> Result<T, E> + 'static) -> Self {
        Self {
            buf: String::new(),
            value: None,
            convert: Box::new(convert),
        }
    }

    pub fn value(&self) -> Option<Result<&T, &E>> {
        self.value.as_ref().map(|v| v.as_ref())
    }

    pub fn new_with(
        buf: impl Into<String>,
        value: Option<T>,
        convert: impl Fn(&str) -> Result<T, E> + 'static,
    ) -> Self {
        Self {
            buf: buf.into(),
            value: value.map(|v| Ok(v)),
            convert: Box::new(convert),
        }
    }

    fn update(&mut self) {
        self.value = Some((self.convert)(&self.buf));
    }
}

impl<T: Clone, E: Clone> TextEditBind<T, E> {
    pub fn value_clone(&self) -> Option<Result<T, E>> {
        match self.value.clone() {
            Some(Ok(v)) => Some(Ok(v)),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}

impl<T: FromStr + 'static> TextEditBind<T, T::Err> {
    pub fn new_from_str() -> Self {
        Self {
            buf: String::new(),
            value: None,
            convert: Box::new(T::from_str),
        }
    }

    pub fn new_from_str_with(buf: impl Into<String>, value: Option<T>) -> Self {
        Self {
            buf: buf.into(),
            value: value.map(|v| Ok(v)),
            convert: Box::new(T::from_str),
        }
    }
}

impl<T, E> TextBuffer for TextEditBind<T, E> {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        &self.buf
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.buf = format!(
            "{}{text}{}",
            &self.buf[..char_index],
            &self.buf[char_index..]
        );
        self.update();

        text.len()
    }

    fn delete_char_range(&mut self, char_range: Range<usize>) {
        self.buf.drain(char_range);
        self.update();
    }
}
