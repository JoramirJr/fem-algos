pub fn binary_search(container: [usize; 20], search: usize) -> bool {
    let container_len: usize = container.len();

    let mut low: usize = 0;
    let mut high: usize = container_len;
    let mut result: Option<usize> = None;

    while low < high {
        let mid: usize = low + ((high - low) / 2);
        let value: usize = container[mid];
        if value == search {
            println!("value: {:?}", value);
            result = Some(value);
            break;
        } else if value <= search {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    result.is_some()
}

#[cfg(test)]
mod test {
    use super::binary_search;

    #[test]
    fn basics() {
        let container: [usize; 20] = [
            77, 58, 95, 29, 98, 7, 40, 8, 66, 14, 11, 21, 18, 38, 20, 99, 25, 52, 87, 1,
        ];
        assert_eq!(binary_search(container, 77), true);
        assert_eq!(binary_search(container, 98), true);
        assert_eq!(binary_search(container, 11), true);
        assert_eq!(binary_search(container, 7), true);
        assert_eq!(binary_search(container, 14), true);
        assert_eq!(binary_search(container, 1400), false);
        assert_eq!(binary_search(container, 57), false);
        assert_eq!(binary_search(container, 0), false);
    }
}
