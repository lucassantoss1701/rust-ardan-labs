use std::fmt::Debug;

fn test(){
    println!("test");
}

fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.join(test, test);


    /*pool.scope(|scope|{
        scope.spawn_broadcast(|_scope, broadcast_context|{
            println!("Hello from broadcast_context {}", broadcast_context.index());
        })
    });*/


    // pool.scope(| scope |{
    //    for n in 0..20{
    //        scope.spawn(move |_| {
    //            println!("Hello from scoped thread {n}");
    //        })
    //    }
    // });

    println!("Hello from the main thread");
}
