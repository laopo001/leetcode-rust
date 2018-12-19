fn num_unique_emails(emails: &mut Vec<String>) -> i32 {
    for item in emails.into_iter() {
        let arr: Vec<_> = item.split('@').collect();
        let left = arr[0];
        let right = arr[1];
        
    }
    return 123;
}

fn main() {

    let mut emails = vec!["123".to_string()];
    num_unique_emails(&mut emails);
    println!("Hello, world!");
}
