use {
    future_test::operations::{AndThenFut, Join},
    future_test::{new_executor_and_spawner, TimerFuture},
    std::time::Duration,
};
extern crate chrono;
use chrono::{DateTime, Utc};

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(
        async {
            log(1, "Mixed iniciado dentro do future");
            TimerFuture::new(Duration::new(1, 0)).await;
            log(1, "Mixed finaliado dentro do future");
        },
        || {
            log(1, "Mixed finalzado - 1s");
        },
    );

    spawner.spawn(TimerFuture::new(Duration::new(3, 0)), || {
        log(2, "TimerFuture finalizado - 3s");
    });

    let join_test = Join::new(
        async {
            TimerFuture::new(Duration::new(8, 0)).await;
        },
        async {
            TimerFuture::new(Duration::new(5, 0)).await;
        },
    );
    spawner.spawn(join_test, || {
        log(3, "Join finalizado - 8s");
    });

    let and_then_test = AndThenFut::new(
        async {
            TimerFuture::new(Duration::new(5, 0)).await;
        },
        async {
            TimerFuture::new(Duration::new(8, 0)).await;
        },
    );
    spawner.spawn(and_then_test, || {
        log(4, "AndThenFut finalizado - 13s");
    });

    log(0, "Tudo está async mesmo...");

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}

fn log(id: usize, msg: &str) {
    let now: DateTime<Utc> = Utc::now();
    println!("{} [EXP-{}] {}", now.format("%H:%M:%S.%6f"), id, msg);
}
