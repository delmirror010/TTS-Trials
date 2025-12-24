use std::collections::HashMap;
use serenity::model::id::RoleId;
use std::sync::LazyLock;

pub const ROLE_CHESHIRE: RoleId = RoleId::new(1411815911614189578);
pub const ROLE_LOST_SOULS: RoleId = RoleId::new(1411813765913120810);
pub const ROLE_HEARTS_QUEEN: RoleId = RoleId::new(1411960237673414707);
pub const ROLE_DREAM_END: RoleId = RoleId::new(1415600828076785705);

// A public global list of all "Systems" roles
pub static SPEC_ROLES: &[RoleId] = &[
    ROLE_CHESHIRE,
    ROLE_LOST_SOULS,
    ROLE_HEARTS_QUEEN,
    ROLE_DREAM_END,
];

let mut role_sys = HashMap::new();
role_sys.insert(RoleId::new(1415600828076785705), a_sys);
role_sys.insert(RoleId::new(1411815911614189578), angel_sys);
role_sys.insert(RoleId::new(1411813765913120810), lost_souls_sys);
role_sys.insert(RoleId::new(1411960237673414707), bunni_sys);

let mut a_sys = HashMap::new();
a_sys.insert(String::from("-y"), "Yuuen");

let mut angel_sys = HashMap::new();

let mut lost_souls_sys = HashMap::new();

let mut bunni_sys = HashMap::new();

// gets member name from message author role ID and suffix
pub fn get_member_nickname(shorthand: &str, msg_author_role: &RoleId) -> Option<&str> {
    // Check if the specific role exists in the system
    let spec_sys = role_sys.get(role_id)?; 

    // Check if the shorthand exists in that system
    // The .get() here returns an Option to match the return type
    spec_sys.get(shorthand).copied()
}

static SUFFIX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\s-([a-zA-Z]+)$").expect("Invalid regex")
});

pub fn shorthand_search(msg: &str) -> Option<&str> {
    // Finds the suffix match
    let caps = SUFFIX_RE.captures(msg)?;
    // Returns reference to suffix e.g. " -y"
    Some(caps[0]) 
}
    
pub fn exact_role_check(member: member) {
     if member.roles.contains(&ROLE_CHESHIRE) {
                    has_special_role = true;
                    get_member_nickname(shorthand_search(&message.content), ROLE_CHESHIRE)
                } else if member.roles.contains(&ROLE_LOST_SOULS) {
                    has_special_role = true;
                    get_member_nickname(shorthand_search(&message.content), ROLE_LOST_SOULS)
                } else if member.roles.contains(&ROLE_HEARTS_QUEEN) {
                    has_special_role = true;
                    get_member_nickname(shorthand_search(&message.content), ROLE_HEARTS_QUEEN)
                } else if member.role.contains(&ROLE_DREAM_END) {
                    // toggles the check
                    has_special_role = true;
                    // returns Option<&str> 
                    get_member_nickname(shorthand_search(&message.content), ROLE_DREAM_END)
}







