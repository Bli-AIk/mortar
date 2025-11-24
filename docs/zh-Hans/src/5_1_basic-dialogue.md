# 写一段简单对话

让我们从最简单的场景开始：一个 NPC 和玩家的短暂对话。

## 场景设定

想象你在做一个 RPG 游戏，有个村民 NPC 会跟玩家打招呼，然后问玩家要不要帮忙。

## 第一版：纯文本对话

最简单的版本，先把对话写出来：

```mortar
// 村民的问候
node VillagerGreeting {
    text: "你好呀，冒险者！"
    text: "欢迎来到我们的小村庄。"
    text: "需要我帮忙吗？"
    
    choice: [
        "需要帮助" -> OfferHelp,
        "不用了，谢谢" -> PoliteFarewell
    ]
}

node OfferHelp {
    text: "太好了！让我看看能帮你什么..."
    text: "这是一份地图，希望对你有用！"
}

node PoliteFarewell {
    text: "好的，祝你旅途愉快！"
}
```

**运行效果**：
1. 显示三段文字
2. 玩家选择
3. 根据选择跳转到不同节点

## 第二版：添加音效

现在让对话更生动，加入音效：

```mortar
node VillagerGreeting {
    text: "你好呀，冒险者！"
    with events: [
        // 在"你"字出现时播放问候音效
        0, play_sound("greeting.wav")
    ]
    
    text: "欢迎来到我们的小村庄。"
    with events: [
        // 在"小村庄"这几个字时播放温馨音乐
        7, play_music("village_theme.ogg")
    ]
    
    text: "需要我帮忙吗？"
    
    choice: [
        "需要帮助" -> OfferHelp,
        "不用了，谢谢" -> PoliteFarewell
    ]
}

node OfferHelp {
    text: "太好了！让我看看能帮你什么..."
    
    text: "这是一份地图，希望对你有用！"
    with events: [
        // 获得道具时的音效
        0, play_sound("item_get.wav"),
        // 同时显示道具图标
        0, show_item_icon("map")
    ]
}

node PoliteFarewell {
    text: "好的，祝你旅途愉快！"
    with events: [
        0, play_sound("farewell.wav")
    ]
}

// 函数声明
fn play_sound(file_name: String)
fn play_music(filename: String)
fn show_item_icon(item_name: String)
```

**新增内容**：
- 每段对话都配上了合适的音效
- 获得道具时有特殊效果
- 所有用到的函数都声明了

## 第三版：动态内容

让对话更个性化，根据玩家名字来问候：

```mortar
node VillagerGreeting {
    // 使用字符串插值，动态插入玩家名字
    text: $"你好呀，{get_player_name()}！"
    with events: [
        0, play_sound("greeting.wav")
    ]
    
    text: "欢迎来到我们的小村庄。"
    with events: [
        7, play_music("village_theme.ogg")
    ]
    
    text: "需要我帮忙吗？"
    
    choice: [
        "需要帮助" -> OfferHelp,
        "不用了，谢谢" -> PoliteFarewell
    ]
}

node OfferHelp {
    text: "太好了！让我看看能帮你什么..."
    
    text: "这是一份地图，希望对你有用！"
    with events: [
        0, play_sound("item_get.wav"),
        0, show_item_icon("map")
    ]
    
    text: $"祝你好运，{get_player_name()}！"
}

node PoliteFarewell {
    text: "好的，祝你旅途愉快！"
    with events: [
        0, play_sound("farewell.wav")
    ]
}

// 函数声明
fn play_sound(file_name: String)
fn play_music(filename: String)
fn show_item_icon(item_name: String)
fn get_player_name() -> String  // 返回玩家名字
```

**新增内容**：
- 使用 `$"..."` 语法的字符串插值
- `{get_player_name()}` 会被替换成实际的玩家名字
- 更有亲切感

## 第四版：条件选项

有些玩家可能已经有地图了，我们加个条件判断：

```mortar
node VillagerGreeting {
    text: $"你好呀，{get_player_name()}！"
    with events: [
        0, play_sound("greeting.wav")
    ]
    
    text: "欢迎来到我们的小村庄。"
    with events: [
        7, play_music("village_theme.ogg")
    ]
    
    text: "需要我帮忙吗？"
    
    choice: [
        // 只有没有地图时才显示这个选项
        "需要帮助" when need_map() -> OfferHelp,
        
        // 已有地图的玩家看到这个
        "我已经有地图了" when has_map() -> AlreadyHasMap,
        
        // 这个选项总是显示
        "不用了，谢谢" -> PoliteFarewell
    ]
}

node OfferHelp {
    text: "太好了！让我看看能帮你什么..."
    text: "这是一份地图，希望对你有用！"
    with events: [
        0, play_sound("item_get.wav"),
        0, show_item_icon("map")
    ]
    text: $"祝你好运，{get_player_name()}！"
}

node AlreadyHasMap {
    text: "哦，看来你准备得很充分！"
    text: "那就祝你一路平安吧！"
}

node PoliteFarewell {
    text: "好的，祝你旅途愉快！"
    with events: [
        0, play_sound("farewell.wav")
    ]
}

// 函数声明
fn play_sound(file_name: String)
fn play_music(filename: String)
fn show_item_icon(item_name: String)
fn get_player_name() -> String
fn need_map() -> Bool      // 判断是否需要地图
fn has_map() -> Bool       // 判断是否已有地图
```

**新增内容**：
- 选项带上了 `when` 条件
- 根据玩家状态显示不同选项
- 更符合真实游戏逻辑

## 编译和使用

保存为 `village_npc.mortar`，然后编译：

```bash
# 编译成JSON
mortar village_npc.mortar

# 如果想看格式化的JSON
mortar village_npc.mortar --pretty

# 指定输出文件名
mortar village_npc.mortar -o npc_dialogue.json
```

## 在游戏中实现

你的游戏需要：

1. **读取 JSON**：解析编译后的 JSON 文件
2. **实现函数**：实现所有声明的函数
   ```csharp
   // 例如在 Unity C# 中
   void play_sound(string filename) {
       AudioSource.PlayOneShot(Resources.Load<AudioClip>(filename));
   }
   
   string get_player_name() {
       return PlayerData.name;
   }
   
   bool has_map() {
       return PlayerInventory.HasItem("map");
   }
   ```

3. **执行对话**：按照 JSON 的指示显示文本、触发事件、处理选择

## 小结

这个例子展示了：
- ✅ 基本的节点和文本
- ✅ 事件绑定
- ✅ 选项跳转
- ✅ 字符串插值
- ✅ 条件判断
- ✅ 函数声明

从这个简单的例子出发，你可以创建更复杂的对话系统！

## 接下来

- 想学更复杂的分支剧情？看 [制作互动故事](./5_2_interactive-story.md)
- 想了解具体集成方法？看 [接入你的游戏](./5_3_game-integration.md)
- 想深入理解语法？看 [核心概念](../4_0_core-concepts)
