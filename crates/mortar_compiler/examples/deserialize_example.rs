//! # deserialize_example.rs
//!
//! # deserialize_example.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Example demonstrating how to use the Mortar deserializer to load and use `.mortared` files.
//!
//! 演示如何使用 Mortar 反序列化器加载和使用 `.mortared` 文件的示例。
//!
//! ## Source File Overview
//!
//! ## 源文件概述
//!
//! Contains a complete example of loading JSON data, navigating the node structure, and simulating a dialogue runtime.
//!
//! 包含加载 JSON 数据、遍历节点结构以及模拟对话运行时的完整示例。

//! 演示如何使用 Mortar 反序列化器加载和使用 .mortared 文件
//!
//! 运行方式:
//! ```bash
//! cargo run --example deserialize_example
//! ```

use mortar_compiler::{Deserializer, MortaredData};
use serde_json::Value;

fn main() -> Result<(), String> {
    println!("=== Mortar 反序列化示例 ===\n");

    // 创建一个临时的测试 JSON 数据 (使用新的 content 结构)
    let test_json = r#"{
        "metadata": {
            "version": "0.5.0",
            "generated_at": "2025-11-23T00:00:00Z"
        },
        "variables": [
            { "name": "player_name", "type": "String" },
            { "name": "score", "type": "Number", "value": 0 }
        ],
        "constants": [
            { "name": "MAX_LEVEL", "type": "Number", "value": 99, "public": true }
        ],
        "enums": [
            { "name": "GameState", "variants": ["menu", "playing", "paused"] }
        ],
        "nodes": [
            {
                "name": "Start",
                "content": [
                    {
                        "type": "text",
                        "value": "欢迎来到游戏世界！",
                        "events": [
                            {
                                "index": 0.0,
                                "actions": [
                                    { "type": "play_music", "args": ["intro.mp3"] }
                                ]
                            }
                        ]
                    },
                    {
                        "type": "text",
                        "value": "你准备好开始冒险了吗？"
                    },
                    {
                        "type": "choice",
                        "options": [
                            { "text": "是的，开始！", "next": "GameStart" },
                            { "text": "让我再想想", "action": "break" }
                        ]
                    }
                ],
                "next": "MainMenu"
            },
            {
                "name": "GameStart",
                "content": [
                    { "type": "text", "value": "游戏开始！祝你好运。" }
                ]
            },
            {
                "name": "MainMenu",
                "content": [
                    { "type": "text", "value": "主菜单" }
                ]
            }
        ],
        "functions": [
            {
                "name": "play_music",
                "params": [ { "name": "file", "type": "String" } ]
            },
            {
                "name": "get_player_name",
                "params": [],
                "return": "String"
            }
        ]
    }"#;

    // 1. 从 JSON 字符串反序列化
    println!("1. 从 JSON 字符串加载数据...");
    let data = Deserializer::from_json(test_json)?;
    print_basic_info(&data);

    // 2. 访问节点
    println!("\n2. 访问节点信息:");
    access_nodes(&data);

    // 3. 访问函数
    println!("\n3. 访问函数声明:");
    access_functions(&data);

    // 4. 访问变量和常量
    println!("\n4. 访问变量和常量:");
    access_variables_constants(&data);

    // 5. 访问枚举
    println!("\n5. 访问枚举:");
    access_enums(&data);

    // 6. 模拟对话系统
    println!("\n6. 模拟简单的对话系统:");
    simulate_dialogue(&data);

    Ok(())
}

fn print_basic_info(data: &MortaredData) {
    println!("  编译器版本: {}", data.metadata.version);
    println!("  生成时间: {}", data.metadata.generated_at);
    println!("  节点数量: {}", data.nodes.len());
    println!("  函数数量: {}", data.functions.len());
    println!("  变量数量: {}", data.variables.len());
    println!("  常量数量: {}", data.constants.len());
    println!("  枚举数量: {}", data.enums.len());
}

fn access_nodes(data: &MortaredData) {
    let node_names = data.node_names();
    println!("  所有节点: {:?}", node_names);

    if let Some(node) = data.get_node("Start") {
        println!("\n  节点 '{}' 详情:", node.name);

        for (i, item_value) in node.content.iter().enumerate() {
            let item_type = item_value.get("type").and_then(Value::as_str).unwrap_or("");

            match item_type {
                "text" => {
                    let text = item_value
                        .get("value")
                        .and_then(Value::as_str)
                        .unwrap_or("");
                    println!("    内容 {}: [文本] {}", i + 1, text);
                    if let Some(events) = item_value.get("events").and_then(Value::as_array) {
                        for event in events {
                            let index = event.get("index").and_then(Value::as_f64).unwrap_or(0.0);
                            println!("      事件 @ {}", index);
                            if let Some(actions) = event.get("actions").and_then(Value::as_array) {
                                for action in actions {
                                    let action_type =
                                        action.get("type").and_then(Value::as_str).unwrap_or("");
                                    let args = action
                                        .get("args")
                                        .and_then(Value::as_array)
                                        .cloned()
                                        .unwrap_or_default();
                                    println!("        - {}({:?})", action_type, args);
                                }
                            }
                        }
                    }
                }
                "choice" => {
                    println!("    内容 {}: [选项]", i + 1);
                    if let Some(options) = item_value.get("options").and_then(Value::as_array) {
                        for (opt_idx, choice) in options.iter().enumerate() {
                            let text = choice.get("text").and_then(Value::as_str).unwrap_or("");
                            print!("      {}. {}", opt_idx + 1, text);
                            if let Some(next) = choice.get("next").and_then(Value::as_str) {
                                print!(" -> {}", next);
                            }
                            if let Some(action) = choice.get("action").and_then(Value::as_str) {
                                print!(" [{}]", action);
                            }
                            println!();
                        }
                    }
                }
                "run_event" => {
                    let name = item_value.get("name").and_then(Value::as_str).unwrap_or("");
                    println!("    内容 {}: [运行事件] {}", i + 1, name);
                }
                _ => {
                    println!("    内容 {}: [未知类型]", i + 1);
                }
            }
        }

        if let Some(next) = &node.next {
            println!("    默认跳转: {}", next);
        }
    }
}

fn access_functions(data: &MortaredData) {
    for func in &data.functions {
        print!("  函数: {}", func.name);

        if !func.params.is_empty() {
            print!("(");
            for (i, param) in func.params.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}: {}", param.name, param.param_type);
            }
            print!(")");
        } else {
            print!("()");
        }

        if let Some(ret) = &func.return_type {
            print!(" -> {}", ret);
        }
        println!();
    }
}

fn access_variables_constants(data: &MortaredData) {
    println!("  变量:");
    for var in &data.variables {
        print!("    let {}: {}", var.name, var.var_type);
        if let Some(value) = &var.value {
            print!(" = {:?}", value);
        }
        println!();
    }

    println!("  常量:");
    for constant in &data.constants {
        print!("    ");
        if constant.public {
            print!("pub ");
        }
        println!(
            "const {}: {} = {:?}",
            constant.name, constant.const_type, constant.value
        );
    }
}

fn access_enums(data: &MortaredData) {
    for enum_def in &data.enums {
        println!("  枚举 {} {{", enum_def.name);
        for variant in &enum_def.variants {
            println!("    {},", variant);
        }
        println!("  }}");
    }
}

fn simulate_dialogue(data: &MortaredData) {
    println!("  开始模拟对话...\n");

    let mut current_node_name = "Start";

    loop {
        if let Some(node) = data.get_node(current_node_name) {
            println!("  === {} ===", node.name);

            for item_value in &node.content {
                let item_type = item_value.get("type").and_then(Value::as_str).unwrap_or("");
                match item_type {
                    "text" => {
                        let text = item_value
                            .get("value")
                            .and_then(Value::as_str)
                            .unwrap_or("");
                        println!("  {}", text);
                        if let Some(events) = item_value.get("events").and_then(Value::as_array) {
                            for event in events {
                                if let Some(actions) =
                                    event.get("actions").and_then(Value::as_array)
                                {
                                    for action in actions {
                                        let action_type = action
                                            .get("type")
                                            .and_then(Value::as_str)
                                            .unwrap_or("");
                                        let args = action
                                            .get("args")
                                            .and_then(Value::as_array)
                                            .cloned()
                                            .unwrap_or_default();
                                        println!("  [触发事件: {}({:?})]", action_type, args);
                                    }
                                }
                            }
                        }
                    }
                    "choice" => {
                        if let Some(options) = item_value.get("options").and_then(Value::as_array) {
                            println!("\n  可用选项:");
                            for (i, choice) in options.iter().enumerate() {
                                let text = choice.get("text").and_then(Value::as_str).unwrap_or("");
                                println!("    {}. {}", i + 1, text);
                            }
                            println!("  (在实际游戏中，玩家会在这里做出选择)\n");
                        }
                        // In a real game, we'd wait for input. Here, we stop.
                        current_node_name = ""; // End simulation
                        break;
                    }
                    "run_event" => {
                        let name = item_value.get("name").and_then(Value::as_str).unwrap_or("");
                        println!("  [运行事件: {}]", name);
                    }
                    _ => {}
                }
            }

            if current_node_name.is_empty() {
                break;
            }

            if let Some(next) = &node.next {
                current_node_name = next;
                println!();
            } else {
                break;
            }
        } else {
            println!("  节点 '{}' 不存在！", current_node_name);
            break;
        }
    }
}
