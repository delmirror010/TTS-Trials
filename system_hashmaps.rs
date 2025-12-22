use std::collections::HashMap;

let mut role_sys = HashMap::new();
did_sys.insert(String::from(1415600828076785705), a_sys);

let mut a_sys = HashMap::new();
a_sys.insert(String::from("-y"), "Yuuen");

// gets member name from message author role ID and suffix
pub fn get_member_nickname(shorthand: &str, msg_author_role: i32) {
  
  return (did_sys.get(msg_author_role)).get(shorthand);  
  
}

