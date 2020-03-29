use {
    // The timer we wrote in the previous section:
    future_test::TimerFuture,
    future_test::new_executor_and_spawner,
    std::time::Duration,
};

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    // spawner.spawn(async {
    //     println!("howdy!");
    //     // Wait for our timer future to complete after two seconds.
    //     TimerFuture::new(Duration::new(2, 0)).await;
    //     println!("done!");
    // });

    println!("Iniciado thread async!");
    spawner.spawn(TimerFuture::new(Duration::new(2, 0)), || {
        println!("Agora acabou mesmo!");
    });
    println!("Ela está async mesmo...");

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}
