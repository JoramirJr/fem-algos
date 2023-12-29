pub fn bubble_sort(mut container: [usize; 7], mut idx: usize) -> [usize; 7] {
    let container_len = container.len();
    let mut loop_idx = 0;
    if idx >= container_len - 1 {
        return container;
    }

    loop {
        if loop_idx < container_len - idx && loop_idx + 1 < container_len {
            match container[loop_idx] > container[loop_idx + 1] {
                true => {
                    let tmp = container[loop_idx];
                    container[loop_idx] = container[loop_idx + 1];
                    container[loop_idx + 1] = tmp
                }
                false => {}
            }
        } else {
            break;
        }
        loop_idx += 1;
    }
    idx += 1;
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
