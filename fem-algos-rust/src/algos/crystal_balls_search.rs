use num::integer::Roots;

pub fn crystal_balls_search(container: &Vec<bool>) -> Option<usize> {
    let len = container.len();
    let len_sqrt = len.sqrt();
    let mut sqrt_loop_idx = len_sqrt;

    let breakarea_search = loop {
        if sqrt_loop_idx < len {
            if container[sqrt_loop_idx] {
                break Some(sqrt_loop_idx);
            }
            sqrt_loop_idx += len_sqrt;
        } else {
            break None;
        }
    };

    if breakarea_search.is_none() {
        return None;
    }

    let mut linear_sqrt_idx = sqrt_loop_idx - len_sqrt;

    loop {
        if linear_sqrt_idx <= sqrt_loop_idx {
            if container[linear_sqrt_idx] {
                break Some(linear_sqrt_idx);
            }
            linear_sqrt_idx += 1;
        } else {
            break None;
        }
    }
}

#[cfg(test)]
mod test {
    use super::crystal_balls_search;
    use rand::Rng;
    use std::iter::repeat;

    #[test]
    fn basics() {
        let idx_rand: usize = rand::thread_rng().gen_range(0..1_000);

        let thousand_falses = repeat(false).take(1_000);

        let mut vec = Vec::from_iter(thousand_falses.clone());

        let mut for_idx = 0;

        for _ in vec.clone() {
            if for_idx >= idx_rand {
                vec[for_idx] = true;
            }
            for_idx += 1;
        }

        assert_eq!(crystal_balls_search(&vec), Some(idx_rand));

        assert_eq!(
            crystal_balls_search(&Vec::from_iter(thousand_falses.clone())),
            None
        );
    }
}
