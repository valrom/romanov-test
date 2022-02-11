use std::thread;

const COMP_LIMIT: usize = 20;

pub fn split_compitation<T ,R, F>(mut data: Vec<T>, f: &F) -> Vec<R>
    where T : Sync + 'static,
    R : Send + 'static,
    F : Send + Fn(T) -> R + Sync + 'static
{
    let mut out_vec = Vec::new();

    if data.len() < COMP_LIMIT {
        for i in data {
            out_vec.push(f(i));
        }
    } else {
        // Don't sure how much threads to create
        // Let it be 4
        const CHUNKS: usize = 4;

        let mut joins = Vec::new();
        let mut results: [Vec<R>; CHUNKS] = [Vec::new(); CHUNKS];

        for (id, chunk) in data.chunks(CHUNKS).enumerate() {
            let thread = thread::spawn(move || {
                for &i in chunk {
                    results.get_mut(id).unwrap().push(f(i));
                }
            });
            joins.push(thread);
        }

        for i in joins {
            i.join().unwrap();
        }
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
