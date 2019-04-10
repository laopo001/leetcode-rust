use std::collections::HashSet;
#[allow(dead_code, unused_assignments)]
pub fn num_unique_emails(emails: &mut Vec<String>) -> i32 {
    let mut set: HashSet<String> = HashSet::new();
    for item in emails.into_iter() {
        let arr: Vec<_> = item.split("@").collect();
        let mut nemail = String::new();
        let left: &str = arr[0];
        let right: &str = arr[1];
        let mut t: String = left.replace(".", "");
        let arr2: Vec<_> = t.match_indices("+").collect();
        if arr2.len() > 0 {
            t = t[0..arr2[0].0].to_string();
        }
        nemail = t + "@" + right;
        set.insert(nemail);
    }
    return set.len() as i32;
}
