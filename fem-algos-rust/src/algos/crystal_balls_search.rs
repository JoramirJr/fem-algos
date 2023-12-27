use num::integer::Roots;

pub fn crystal_balls_search(container: [usize; 20], needle: usize) -> Option<usize> {
    let leng_sqr = container.len();
    leng_sqr.sqrt();
    let mut sqr_loop_idx = leng_sqr;

    let breakarea_search = loop {
        if sqr_loop_idx < leng_sqr {
            sqr_loop_idx += leng_sqr;
            if container[sqr_loop_idx] == needle {
                break Some(container[sqr_loop_idx]);
            }
        } else {
            sqr_loop_idx += leng_sqr;
            break None;
        }
    };

    if breakarea_search.is_some() {
        return Some(container[sqr_loop_idx]);
    }

    sqr_loop_idx -= leng_sqr;
    let mut linear_sqr_idx = sqr_loop_idx;

    loop {
        if linear_sqr_idx > sqr_loop_idx {
            break None;
        } else {
            if container[linear_sqr_idx] == needle {
                break Some(container[linear_sqr_idx]);
            }
        }
        linear_sqr_idx += 1;
    }
}

#[cfg(test)]
mod test {
    use super::crystal_balls_search;
    use rand::{rngs::ThreadRng, Rng};
    use std::iter::repeat;

    #[test]
    fn basics() {
        let mut rng: ThreadRng = rand::thread_rng();
        let idx = rng.gen_range(0..10_000);

        let thousand_falses = repeat(false).take(1000);

        let mut data = Vec::from_iter(thousand_falses);

        for i in idx {
            data[i] = true;
        }

        // for (let i = idx; i < 10000; ++i) {
        //     data[i] = true;
        // }

        // expect(two_crystal_balls(data)).toEqual(idx);
        // expect(two_crystal_balls(new Array(821).fill(false))).toEqual(-1);
    }
}
