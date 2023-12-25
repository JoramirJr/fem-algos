pub fn linear_search(container: [i32; 20], search: i32) -> bool {
    container
        .iter()
        .find(|number: &&i32| number.to_owned() == &search)
        .is_some()
}
