use std::iter::Skip;
use std::str::CharIndices;

pub trait CharWindows<'a> {
  fn char_windows(&'a self, size: usize) -> CharWindowsIter<'a>;
}

impl<'a> CharWindows<'a> for &str {
  fn char_windows(&'a self, size: usize) -> CharWindowsIter<'a> {
    assert!(size > 0);
    let lo = self.char_indices();
    let hi = self.char_indices().skip(size - 1);
    CharWindowsIter { input: self, lo, hi }
  }
}

pub struct CharWindowsIter<'a> {
  input: &'a str,
  lo: CharIndices<'a>,
  hi: Skip<CharIndices<'a>>,
}

impl<'a> Iterator for CharWindowsIter<'a> {
  type Item = &'a str;

  fn next(&mut self) -> Option<Self::Item> {
    let (lo, _) = self.lo.next()?;
    let (hi, _) = self.hi.next()?;
    self.input.get(lo..=hi)
  }
}
