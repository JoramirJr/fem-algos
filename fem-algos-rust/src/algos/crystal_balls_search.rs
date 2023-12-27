use num::integer::Roots;

pub fn crystal_balls_search(container: &Vec<bool>) -> Option<usize> {
    let len = container.len();
    let len_sqrt = len.sqrt();
    let mut sqrt_loop_idx = len_sqrt;

    let breakarea_search = loop {
        if sqrt_loop_idx < len {
            sqrt_loop_idx += len_sqrt;
            if container[sqrt_loop_idx] {
                break Some(sqrt_loop_idx);
            }
        } else {
            sqrt_loop_idx += leng_sqr;
        }
    };

    if breakarea_search.is_some() {
        return Some(sqrt_loop_idx);
    }

    sqrt_loop_idx -= leng_sqr;
    let mut linear_sqr_idx = sqrt_loop_idx;

    loop {
        if linear_sqr_idx <= sqrt_loop_idx {
            break None;
        } else {
            if container[linear_sqr_idx] {
                break Some(linear_sqr_idx);
            }
        }
        linear_sqr_idx += 1;
    }
}

#[cfg(test)]
mod test {
    use super::crystal_balls_search;
    use rand::Rng;
    use std::iter::repeat;

    #[test]
    fn basics() {
        let idx_rand: usize = rand::thread_rng().gen_range(0..10_000);

        let thousand_falses = repeat(false).take(1000);

        let mut vec = Vec::from_iter(thousand_falses.clone());

        let mut for_idx = 0;

        for _ in vec.clone() {
            if for_idx > idx_rand {
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
