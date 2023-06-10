use std::borrow::Cow;
use url::{form_urlencoded::Parse, Url};

pub trait UrlExt {
  fn param<'a>(&'a self, key: &'a str) -> Option<Cow<'a, str>> {
    self.param_iter(key).next()
  }
  fn param_iter<'a>(&'a self, key: &'a str) -> ParamIter<'a>;
}

impl UrlExt for Url {
  fn param_iter<'a>(&'a self, key: &'a str) -> ParamIter<'a> {
    ParamIter {
      inner: self.query_pairs(),
      key,
    }
  }
}

pub struct ParamIter<'a> {
  inner: Parse<'a>,
  key: &'a str,
}

impl<'a> Iterator for ParamIter<'a> {
  type Item = Cow<'a, str>;

  fn next(&mut self) -> Option<Self::Item> {
    let key = self.key;
    Some(self.inner.find(|(k, _)| k == key)?.1)
  }
}
