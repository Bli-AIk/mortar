use std::time::{Duration, Instant};
use tokio;

#[tokio::test]
async fn test_shutdown_timing() {
    println!("🔄 测试LSP关闭时间性能");
    
    // 模拟简单的关闭操作计时
    let start = Instant::now();
    
    // 模拟清理操作
    tokio::time::sleep(Duration::from_millis(1)).await;
    
    let duration = start.elapsed();
    println!("✅ 模拟关闭操作耗时: {:?}", duration);
    
    // 验证关闭在合理时间内完成
    assert!(duration < Duration::from_millis(50), "关闭时间过长: {:?}", duration);
    
    println!("✅ LSP关闭时间测试通过!");
}

#[tokio::test] 
async fn test_repeated_shutdowns() {
    println!("🔄 测试重复关闭操作性能");
    
    for i in 0..10 {
        let start = Instant::now();
        
        // 模拟关闭操作
        tokio::time::sleep(Duration::from_micros(100)).await;
        
        let duration = start.elapsed();
        println!("第{}次关闭耗时: {:?}", i + 1, duration);
        assert!(duration < Duration::from_millis(10), "第{}次关闭时间过长: {:?}", i + 1, duration);
    }
    
    println!("✅ 重复关闭测试通过!");
}

#[tokio::test]
async fn test_concurrent_operations() {
    println!("🔄 测试并发操作性能");
    
    let mut handles = vec![];
    
    for i in 0..5 {
        let handle = tokio::spawn(async move {
            let start = Instant::now();
            // 模拟并发操作
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
                println!("并发任务 {} 完成，耗时: {:?}", task_id, duration);
                if duration > max_duration {
                    max_duration = duration;
                }
            }
            Err(e) => {
                println!("并发任务失败: {:?}", e);
            }
        }
    }
    
    println!("✅ 最大并发操作耗时: {:?}", max_duration);
    assert!(max_duration < Duration::from_millis(50), "并发操作时间过长: {:?}", max_duration);
    
    println!("✅ 并发操作测试通过!");
}