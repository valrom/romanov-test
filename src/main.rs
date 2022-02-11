

fn main() {
    let vs = vec![10;100];
    let mut outs = Vec::with_capacity(100);

    let step = 100 / 4;

    crossbeam::scope(|s| {
        let mut joins = Vec::with_capacity(4);

        for chunk in vs.chunks(step) {
            joins.push(
                s.spawn( move |_| {
                    let mut out = Vec::with_capacity(step);

                    for &i in chunk {
                        out.push(i*i);
                    }

                    out
                })
            );
        }

        for i in joins {
            outs.append(i.join().unwrap().as_mut());
        }


    }).unwrap();

    println!("{:?}\n{:?}", vs, outs);
}
