use std::thread;


// Round 2

const COMP_LIMIT: usize = 10;

pub fn split_compitation<T ,R, F>(mut data: Vec<T>, f: &F) -> Vec<R>
    where T : Send + Sync + Copy + std::fmt::Debug + 'static,
    R : Send + Copy + Default + 'static,
    F : Send + Fn(T) -> R + Sync + 'static,
{
    let mut out_vec = vec![R::default(); data.len()];

    if data.len() < COMP_LIMIT {
        for i in 0..data.len() {
            out_vec[i] = f(data[i]);
        }
    } else {
        // Don't sure how much threads to create
        // Let it be 4
        const CHUNKS: usize = 4;
        let step = CHUNKS / data.len();
        
        // split data to parts
        let mut parts = Vec::with_capacity(CHUNKS);
        for i in data.chunks(step) {
            parts.push(Vec::from(i));
        }
        dbg!(&parts);
    }

    out_vec
}

mod test {
    use crate::*;

    fn from_char_to_i32(in_char: char) -> i32 {
        if in_char.is_alphabetic() {
            11
        } else if in_char.is_numeric() {
            22
        } else {
            33
        }
    }

    #[test]
    fn basic_test_under_limit_count() {
        let vs = vec![
            'f', 'e', 'n', 'd', 'e', 'r', ' ', 'F', '1', '0', '2', '0', 'S',
        ];

        let out = split_compitation(vs, &from_char_to_i32);

        assert_eq!(
            out,
            vec![11, 11, 11, 11, 11, 11, 33, 11, 22, 22, 22, 22, 11]
        );
    }
}
