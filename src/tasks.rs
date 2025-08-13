
pub async fn run_tasks() {

    let task1 = tokio::spawn(async {
        // Simulate some asynchronous work

        println!("Task 1 is running");

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        println!("Task 1 is complete");
    });

    let task2 = tokio::spawn(async {
        // Simulate some asynchronous work
        println!("Task 2 is running");
    });

    // Wait for both tasks to complete
    let _ = tokio::join!(task1, task2);
}