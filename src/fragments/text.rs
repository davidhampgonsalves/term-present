use fragments;

pub fn generate(fragment: &fragments::Fragment, indent:u16, max_width:u16) -> String {
  fragment.content.to_string()
}

#[cfg(test)]
mod test {
  use super::*;
  use fragments;

  #[test]
  fn text_generates() {
    let fragment = fragments::Fragment {
      content: "sample text".to_string(),
      ..Default::default()
    };
    assert_eq!(generate(&fragment, 1, 100), "sample text");
  }
}
