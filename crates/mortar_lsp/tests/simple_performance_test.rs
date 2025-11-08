use std::time::{Duration, Instant};

// 专注于核心性能测试，不依赖复杂的LSP组件
#[tokio::test]
async fn test_parsing_performance() {
    println!("📊 测试Mortar语言解析性能");

    // 准备测试内容 - 使用正确的语法
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

    // 测试简单内容解析
    println!("🟡 测试简单内容解析性能...");
    let start = Instant::now();
    let mut success_count = 0;
    for i in 0..100 {
        match mortar_compiler::ParseHandler::parse_source_code(simple_content) {
            Ok(_) => success_count += 1,
            Err(e) => println!("简单解析失败 {}: {}", i, e),
        }
    }
    let simple_duration = start.elapsed();
    println!(
        "✅ 100次简单内容解析: 成功{}次, 耗时: {:?}",
        success_count, simple_duration
    );

    // 测试复杂内容解析
    println!("🟠 测试复杂内容解析性能...");
    let start = Instant::now();
    let mut success_count = 0;
    for i in 0..50 {
        match mortar_compiler::ParseHandler::parse_source_code(complex_content) {
            Ok(_) => success_count += 1,
            Err(e) => println!("复杂解析失败 {}: {}", i, e),
        }
    }
    let complex_duration = start.elapsed();
    println!(
        "✅ 50次复杂内容解析: 成功{}次, 耗时: {:?}",
        success_count, complex_duration
    );

    // 性能统计
    if success_count > 0 {
        let simple_avg = simple_duration.as_micros() / 100;
        let complex_avg = complex_duration.as_micros() / 50;

        println!("\n📈 性能统计:");
        println!("  简单内容平均解析时间: {}μs", simple_avg);
        println!("  复杂内容平均解析时间: {}μs", complex_avg);
    }

    // 性能断言
    assert!(
        simple_duration < Duration::from_millis(500),
        "简单内容解析时间过长: {:?}",
        simple_duration
    );
    assert!(
        complex_duration < Duration::from_secs(2),
        "复杂内容解析时间过长: {:?}",
        complex_duration
    );

    println!("✅ 解析性能测试通过!");
}

#[tokio::test]
async fn test_memory_usage_simulation() {
    println!("🧠 测试内存使用模拟");

    let start = Instant::now();

    // 模拟大量文档处理
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
            .replace("node_{}", &format!("node_{}", (i + 1) % 50)); // 循环引用以避免无限增长

        match mortar_compiler::ParseHandler::parse_source_code(&content) {
            Ok(program) => {
                parse_results.push(program);
            }
            Err(e) => {
                println!("解析错误 (文档 {}): {}", i, e);
            }
        }

        if i % 50 == 0 && i > 0 {
            println!("已处理 {} 个文档", i);
        }
    }

    let processing_duration = start.elapsed();
    println!("处理200个文档耗时: {:?}", processing_duration);
    println!("成功解析的文档数量: {}", parse_results.len());

    // 清理测试
    let cleanup_start = Instant::now();
    drop(parse_results);
    let cleanup_duration = cleanup_start.elapsed();
    println!("内存清理耗时: {:?}", cleanup_duration);

    // 性能断言
    assert!(
        processing_duration < Duration::from_secs(5),
        "文档处理时间过长: {:?}",
        processing_duration
    );

    println!("✅ 内存使用测试通过!");
}

#[tokio::test]
async fn test_concurrent_parsing() {
    println!("🔄 测试并发解析性能");

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

    // 创建并发任务
    let mut handles = vec![];

    for i in 0..10 {
        let content_copy = content.to_string();
        let handle = tokio::spawn(async move {
            let task_start = Instant::now();

            // 每个任务解析多次
            let mut results = Vec::new();
            for _ in 0..20 {
                if let Ok(program) = mortar_compiler::ParseHandler::parse_source_code(&content_copy)
                {
                    results.push(program);
                }
            }

            let task_duration = task_start.elapsed();
            println!(
                "任务 {} 完成，解析了 {} 次，耗时: {:?}",
                i,
                results.len(),
                task_duration
            );
            (i, results.len(), task_duration)
        });

        handles.push(handle);
    }

    // 等待所有任务完成
    let mut total_parses = 0;
    for handle in handles {
        match handle.await {
            Ok((_, count, _)) => total_parses += count,
            Err(e) => println!("任务失败: {:?}", e),
        }
    }

    let total_duration = start.elapsed();
    println!("并发测试总耗时: {:?}", total_duration);
    println!("总共完成 {} 次解析", total_parses);

    // 性能断言
    assert!(
        total_duration < Duration::from_secs(3),
        "并发解析时间过长: {:?}",
        total_duration
    );
    assert_eq!(total_parses, 200, "解析次数不正确");

    println!("✅ 并发解析测试通过!");
}
