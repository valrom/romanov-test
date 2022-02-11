// Round 2

const COMP_LIMIT: usize = 100;

/// Implement basic function to split some generic computational work between threads.
/// Split should occur only on some threshold - if computational work (input length) is shorter than this
/// threshold, no splitting should occur and no threads should be created.
/// You get as input: 
/// 1. Vec<T>
/// 2. Function f(t: T) -> R
/// Threshold can be just constant. 
/// You should return:
///  1. Up to you, but probably some Vec of the same length as input(1)
/// Code should be published on github.
pub fn split_compitation<T ,R, F>(data: &Vec<T>, f: &F) -> Vec<R>
    where T : Send + Sync + Copy,
    R : Send,
    F : Send + Fn(T) -> R + Sync,
{
    let mut out_vec = Vec::with_capacity(data.len());

    if data.len() < COMP_LIMIT {
        for i in data {
            out_vec.push(f(*i));
        }
    } else {
        // Don't sure how much threads to create
        // Let it be 4
        const CHUNKS: usize = 4;
        let step = data.len() / CHUNKS;
        
        crossbeam::scope(|s| {
            let mut joins = Vec::with_capacity(CHUNKS + 1);

            for chunk in data.chunks(step) {
                joins.push(
                    s.spawn(move |_| {
                        let mut out = Vec::with_capacity(step);

                        for &i in chunk {
                            out.push(f(i));
                        }

                        out
                    })
                );
                
            }

            for i in joins {
                out_vec.append(i.join().unwrap().as_mut());
            }

        }).unwrap();
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

        let out = split_compitation(&vs, &from_char_to_i32);

        assert_eq!(
            out,
            vec![11, 11, 11, 11, 11, 11, 33, 11, 22, 22, 22, 22, 11]
        );
    }

    #[test]
    fn square_test() {
        let vs = vec![2; 1000];

        let out = split_compitation(&vs, &|i| {
            i*i*i
        });

        assert_eq!(out, vec![8;1000]);
    }
}
