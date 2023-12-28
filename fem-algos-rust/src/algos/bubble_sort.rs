pub fn bubble_sort(mut container: [usize; 7], mut idx: usize) -> [usize; 7] {
    if idx >= container.len() {
        return container;
    }
    // if idx < container.len() - idx {
    for (local_idx, n) in container.iter().enumerate() {
        if local_idx < container.len() - idx {
            let n_copy = n.clone();
            match n_copy > container[local_idx + 1] {
                true => {
                    container[local_idx] = container[local_idx + 1];
                    container[local_idx + 1] = n_copy;
                    return container;
                }
                false => {}
            }
        }
    }
    // }
    idx += 1;
    println!("Container at index {}: {:?}", idx, container);

    return bubble_sort(container, idx);
}

#[cfg(test)]
mod test {
    use super::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let arr: [usize; 7] = [9, 3, 7, 4, 69, 420, 42];

        assert_eq!(bubble_sort(arr, 0), [3, 4, 7, 9, 42, 69, 420]);
    }
}
