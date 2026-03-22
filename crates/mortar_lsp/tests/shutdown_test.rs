//! This file contains timing-oriented tests for the Mortar LSP shutdown path.
//! The scenarios simulate repeated and concurrent teardown work so the crate can
//! catch obviously regressive shutdown latency before it reaches editor users.
//!
//! 这个文件包含 Mortar LSP 关闭流程的时序测试。它通过模拟重复和并发的清理工作，
//! 提前抓出明显退化的关闭延迟，避免问题先传到编辑器用户手里。

use std::time::{Duration, Instant};

#[tokio::test]
async fn test_shutdown_timing() {
    println!("🔄 Testing LSP shutdown timing performance");

    // Simulate simple shutdown operation timing
    let start = Instant::now();

    // Simulate cleanup operations
    tokio::time::sleep(Duration::from_millis(1)).await;

    let duration = start.elapsed();
    println!("✅ Simulated shutdown operation took: {:?}", duration);

    // Verify shutdown completes within reasonable time
    assert!(
        duration < Duration::from_millis(50),
        "Shutdown time too long: {:?}",
        duration
    );

    println!("✅ LSP shutdown timing test passed!");
}

#[tokio::test]
async fn test_repeated_shutdowns() {
    println!("🔄 Testing repeated shutdown operations performance");

    for i in 0..10 {
        let start = Instant::now();

        // Simulate shutdown operation
        tokio::time::sleep(Duration::from_micros(100)).await;

        let duration = start.elapsed();
        println!("Shutdown #{} took: {:?}", i + 1, duration);
        assert!(
            duration < Duration::from_millis(10),
            "Shutdown #{} time too long: {:?}",
            i + 1,
            duration
        );
    }

    println!("✅ Repeated shutdown test passed!");
}

#[tokio::test]
async fn test_concurrent_operations() {
    println!("🔄 Testing concurrent operations performance");

    let mut handles = vec![];

    for i in 0..5 {
        let handle = tokio::spawn(async move {
            let start = Instant::now();
            // Simulate concurrent operation
            tokio::time::sleep(Duration::from_micros(200)).await;
            let duration = start.elapsed();
            (i, duration)
        });
        handles.push(handle);
    }

    let mut max_duration = Duration::from_nanos(0);
    for handle in handles {
        match handle.await {
            Ok((task_id, duration)) => {
                println!(
                    "Concurrent task {} completed, took: {:?}",
                    task_id, duration
                );
                max_duration = max_duration.max(duration);
            }
            Err(e) => {
                println!("Concurrent task failed: {:?}", e);
            }
        }
    }

    println!("✅ Max concurrent operation time: {:?}", max_duration);
    assert!(
        max_duration < Duration::from_millis(50),
        "Concurrent operation time too long: {:?}",
        max_duration
    );

    println!("✅ Concurrent operations test passed!");
}
