use num::integer::Roots;

pub fn crystal_balls_search(container: [usize; 20], search: usize) {
    let leng_sqrt = container.len();
    leng_sqrt.sqrt();
    let mut while_index = leng_sqrt;

    let first_loop_rst = loop {
        if container[while_index] == search {
            break container[while_index];
        }

    }
    if first_loop_rst {
        return fir
    }

}
