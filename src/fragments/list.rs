use fragments;
use utils;

pub fn generate(fragment: &fragments::Fragment, indent:u16, max_width:u16) -> String {
  fragments::generate_block(&fragment, utils::next_indent(indent), max_width)
}

#[cfg(test)]
mod test {
  use super::*;
  use fragments;

  #[test]
  fn list_generates() {
    let fragment = fragments::Fragment {
      element_type: fragments::ElementType::List,
      children: Some(vec![
        Box::new(fragments::Fragment {
          content: "item 1".to_string(),
          ..Default::default()
        }),
        Box::new(fragments::Fragment {
          content: "item 2".to_string(),
          ..Default::default()
        })]),
      ..Default::default()
    };

    assert_eq!(generate(&fragment, 1, 100), "   -item 1\n   -item 2\n");
  }

  /* #[test] */
  /* fn nested_list_generates() { */
  /*   let fragment = fragments::Fragment { */
  /*     element_type: fragments::ElementType::List, */
  /*     children: vec![ */
  /*       fragments::Fragment { */
  /*         content: "item 1".to_string(), */
  /*         ..Default::default() */
  /*       }, */
  /*       fragments::Fragment { */
  /*         element_type: fragments::ElementType::List, */
  /*         children: vec![ */
  /*           fragments::Fragment { */
  /*             content: "nested item 1".to_string(), */
  /*             ..Default::default() */
  /*           }, */
  /*           fragments::Fragment { */
  /*             content: "nested item 2".to_string(), */
  /*             ..Default::default() */
  /*           }, */
  /*         ], */
  /*         ..Default::default() */
  /*       }, */
  /*       fragments::Fragment { */
  /*         content: "item 2".to_string(), */
  /*         ..Default::default() */
  /*       }], */
  /*     ..Default::default() */
  /*   }; */

  /*   assert_eq!(generate(&fragment, 1, 100), " -item 1\n   -nested item 1\n   -nested item 2\n -item 2\n"); */
  /* } */
}
