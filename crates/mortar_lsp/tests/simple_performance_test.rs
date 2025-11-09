use std::time::{Duration, Instant};

// Core performance test focused on parsing, not dependent on complex LSP components
#[tokio::test]
async fn test_parsing_performance() {
    println!("📊 Testing Mortar language parsing performance");

    // Prepare test content - using correct syntax
    let simple_content = r#"
node start {
    text: "hello world"
}
"#;

    let complex_content = r#"
node start {
    text: "欢迎来到游戏！"
    text: "这是一个复杂的测试节点"
    
    choice: [
        "开始游戏" -> game,
        "查看设置" -> settings,
        "退出游戏" -> exit
    ]
} -> game

node game {
    text: "这里是游戏主界面"
    text: "你的分数很高"
    
    choice: [
        "进入战斗" -> battle,
        "查看背包" -> inventory,
        "返回主界面" -> start
    ]
}

node battle {
    text: "战斗开始！"
    
    choice: [
        "攻击" -> battle_result,
        "防御" -> battle_result,
        "逃跑" -> game
    ]
}

node battle_result {
    text: "战斗结束"
    
    choice: [
        "继续游戏" -> game,
        "返回主界面" -> start
    ]
}

node inventory {
    text: "这是你的背包"
    
    choice: [
        "使用物品" -> game,
        "返回" -> game
    ]
}

node settings {
    text: "游戏设置"
    
    choice: [
        "音量设置" -> volume_settings,
        "图像设置" -> graphics_settings,
        "返回" -> start
    ]
}

node volume_settings {
    text: "调整音量"
    
    choice: [
        "返回设置" -> settings
    ]
}

node graphics_settings {
    text: "调整图像质量"
    
    choice: [
        "返回设置" -> settings
    ]
}

node exit {
    text: "谢谢游戏！再见！"
}

fn play_sound(file_name: String)
fn set_volume(level: Number)
fn get_score() -> Number
"#;

    // Test simple content parsing
    println!("🟡 Testing simple content parsing performance...");
    let start = Instant::now();
    let mut success_count = 0;
    for i in 0..100 {
        match mortar_compiler::ParseHandler::parse_source_code(simple_content, false) {
            Ok(_) => success_count += 1,
            Err(e) => println!("Simple parsing failed {}: {}", i, e),
        }
    }
    let simple_duration = start.elapsed();
    println!(
        "✅ 100 simple content parses: {} successes, time: {:?}",
        success_count, simple_duration
    );

    // Test complex content parsing
    println!("🟠 Testing complex content parsing performance...");
    let start = Instant::now();
    let mut success_count = 0;
    for i in 0..50 {
        match mortar_compiler::ParseHandler::parse_source_code(complex_content, false) {
            Ok(_) => success_count += 1,
            Err(e) => println!("Complex parsing failed {}: {}", i, e),
        }
    }
    let complex_duration = start.elapsed();
    println!(
        "✅ 50 complex content parses: {} successes, time: {:?}",
        success_count, complex_duration
    );

    // Performance statistics
    if success_count > 0 {
        let simple_avg = simple_duration.as_micros() / 100;
        let complex_avg = complex_duration.as_micros() / 50;

        println!("\n📈 Performance Statistics:");
        println!("  Simple content average parse time: {}μs", simple_avg);
        println!("  Complex content average parse time: {}μs", complex_avg);
    }

    // Performance assertions
    assert!(
        simple_duration < Duration::from_millis(500),
        "Simple content parsing time too long: {:?}",
        simple_duration
    );
    assert!(
        complex_duration < Duration::from_secs(2),
        "Complex content parsing time too long: {:?}",
        complex_duration
    );

    println!("✅ Parsing performance test passed!");
}

#[tokio::test]
async fn test_memory_usage_simulation() {
    println!("🧠 Testing memory usage simulation");

    let start = Instant::now();

    // Simulate large document processing
    let mut parse_results = Vec::new();

    let sample_content = r#"
node node_{} {
    text: "这是节点 {}"
    
    choice: [
        "选项1" -> node_{},
        "选项2" -> node_{}
    ]
}

fn event_{}() -> String
"#;

    for i in 0..200 {
        let content = sample_content
            .replace("{}", &i.to_string())
            .replace("node_{}", &format!("node_{}", (i + 1) % 50)); // Circular reference to avoid infinite growth

        match mortar_compiler::ParseHandler::parse_source_code(&content, false) {
            Ok(program) => {
                parse_results.push(program);
            }
            Err(e) => {
                println!("Parse error (document {}): {}", i, e);
            }
        }

        if i % 50 == 0 && i > 0 {
            println!("Processed {} documents", i);
        }
    }

    let processing_duration = start.elapsed();
    println!("Processing 200 documents took: {:?}", processing_duration);
    println!(
        "Successfully parsed document count: {}",
        parse_results.len()
    );

    // Test cleanup
    let cleanup_start = Instant::now();
    drop(parse_results);
    let cleanup_duration = cleanup_start.elapsed();
    println!("Memory cleanup took: {:?}", cleanup_duration);

    // Performance assertions
    assert!(
        processing_duration < Duration::from_secs(5),
        "Document processing time too long: {:?}",
        processing_duration
    );

    println!("✅ Memory usage test passed!");
}

#[tokio::test]
async fn test_concurrent_parsing() {
    println!("🔄 Testing concurrent parsing performance");

    let content = r#"
node concurrent_node {
    text: "这是并发测试节点"
    
    choice: [
        "继续" -> next_node
    ]
}

node next_node {
    text: "下一个节点"
}

fn concurrent_test(id: Number) -> String
"#;

    let start = Instant::now();

    // Create concurrent tasks
    let mut handles = vec![];

    for i in 0..10 {
        let content_copy = content.to_string();
        let handle = tokio::spawn(async move {
            let task_start = Instant::now();

            // Each task parses multiple times
            let mut results = Vec::new();
            for _ in 0..20 {
                if let Ok(program) = mortar_compiler::ParseHandler::parse_source_code(&content_copy, false)
                {
                    results.push(program);
                }
            }

            let task_duration = task_start.elapsed();
            println!(
                "Task {} completed, parsed {} times, took: {:?}",
                i,
                results.len(),
                task_duration
            );
            (i, results.len(), task_duration)
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    let mut total_parses = 0;
    for handle in handles {
        match handle.await {
            Ok((_, count, _)) => total_parses += count,
            Err(e) => println!("Task failed: {:?}", e),
        }
    }

    let total_duration = start.elapsed();
    println!("Total concurrent test time: {:?}", total_duration);
    println!("Total completed parses: {}", total_parses);

    // Performance assertions
    assert!(
        total_duration < Duration::from_secs(3),
        "Concurrent parsing time too long: {:?}",
        total_duration
    );
    assert_eq!(total_parses, 200, "Incorrect parse count");

    println!("✅ Concurrent parsing test passed!");
}
