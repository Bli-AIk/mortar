//! 演示如何使用 Mortar 反序列化器加载和使用 .mortared 文件
//!
//! 运行方式:
//! ```bash
//! cargo run --example deserialize_example
//! ```

use mortar_compiler::{Deserializer, MortaredData};

fn main() -> Result<(), String> {
    println!("=== Mortar 反序列化示例 ===\n");

    // 创建一个临时的测试 JSON 数据
    let test_json = r#"{
        "metadata": {
            "version": "0.4.0",
            "generated_at": "2025-11-20T00:00:00Z"
        },
        "variables": [
            {
                "name": "player_name",
                "type": "String"
            },
            {
                "name": "score",
                "type": "Number",
                "value": 0
            }
        ],
        "constants": [
            {
                "name": "MAX_LEVEL",
                "type": "Number",
                "value": 99,
                "public": true
            }
        ],
        "enums": [
            {
                "name": "GameState",
                "variants": ["menu", "playing", "paused"]
            }
        ],
        "nodes": [
            {
                "name": "Start",
                "texts": [
                    {
                        "text": "欢迎来到游戏世界！",
                        "events": [
                            {
                                "index": 0.0,
                                "actions": [
                                    {
                                        "type": "play_music",
                                        "args": ["intro.mp3"]
                                    }
                                ]
                            }
                        ]
                    },
                    {
                        "text": "你准备好开始冒险了吗？"
                    }
                ],
                "choice": [
                    {
                        "text": "是的，开始！",
                        "next": "GameStart"
                    },
                    {
                        "text": "让我再想想",
                        "action": "break"
                    }
                ],
                "next": "MainMenu"
            },
            {
                "name": "GameStart",
                "texts": [
                    {
                        "text": "游戏开始！祝你好运。"
                    }
                ]
            },
            {
                "name": "MainMenu",
                "texts": [
                    {
                        "text": "主菜单"
                    }
                ]
            }
        ],
        "functions": [
            {
                "name": "play_music",
                "params": [
                    {
                        "name": "file",
                        "type": "String"
                    }
                ]
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
    // 获取所有节点名称
    let node_names = data.node_names();
    println!("  所有节点: {:?}", node_names);

    // 访问特定节点
    if let Some(node) = data.get_node("Start") {
        println!("\n  节点 '{}' 详情:", node.name);

        // 显示文本
        for (i, text) in node.texts.iter().enumerate() {
            println!("    文本 {}: {}", i + 1, text.text);

            // 显示事件
            if let Some(events) = &text.events {
                for event in events {
                    println!("      事件 @ {}", event.index);
                    for action in &event.actions {
                        println!(
                            "        - {}({:?})",
                            action.action_type, action.args
                        );
                    }
                }
            }
        }

        // 显示选项
        if let Some(choices) = &node.choice {
            println!("    选项:");
            for (i, choice) in choices.iter().enumerate() {
                print!("      {}. {}", i + 1, choice.text);
                if let Some(next) = &choice.next {
                    print!(" -> {}", next);
                }
                if let Some(action) = &choice.action {
                    print!(" [{}]", action);
                }
                println!();
            }
        }

        // 显示下一个节点
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
    // 变量
    println!("  变量:");
    for var in &data.variables {
        print!("    let {}: {}", var.name, var.var_type);
        if let Some(value) = &var.value {
            print!(" = {:?}", value);
        }
        println!();
    }

    // 常量
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

    let mut current_node = "Start";

    loop {
        if let Some(node) = data.get_node(current_node) {
            println!("  === {} ===", node.name);

            // 显示所有文本
            for text in &node.texts {
                println!("  {}", text.text);

                // 模拟事件触发
                if let Some(events) = &text.events {
                    for event in events {
                        for action in &event.actions {
                            println!(
                                "  [触发事件: {}({:?})]",
                                action.action_type, action.args
                            );
                        }
                    }
                }
            }

            // 如果有选项，显示但不实际选择
            if let Some(choices) = &node.choice {
                println!("\n  可用选项:");
                for (i, choice) in choices.iter().enumerate() {
                    println!("    {}. {}", i + 1, choice.text);
                }
                println!("  (在实际游戏中，玩家会在这里做出选择)\n");
                break; // 示例中到此结束
            }

            // 如果有下一个节点，继续
            if let Some(next) = &node.next {
                current_node = next;
                println!();
            } else {
                break;
            }
        } else {
            println!("  节点 '{}' 不存在！", current_node);
            break;
        }
    }
}
