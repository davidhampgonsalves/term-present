
pub fn spaces(count:u16) -> String {
  (0..count).into_iter().fold("".to_string(), |spaces, _i| spaces + " ")
}

pub fn wrap(content:String, indent:u16, max_width:u16) -> String {
  content
}

pub fn next_indent(indent:u16) -> u16 { indent + 2 }
