# 制作互动故事

现在让我们创建一个完整的互动小故事，包含多个分支和结局。

## 故事大纲

**《神秘的森林》** - 一个探险者的故事：
- 玩家在森林里遇到神秘的魔法泉水
- 根据选择，会有不同的结局
- 有条件判断和多层嵌套选择

## 完整代码

```mortar
// ========== 开场 ==========
node OpeningScene {
    text: "夜幕降临，你独自走在幽暗的森林中。"
    with events: [
        0, play_ambient("forest_night.ogg"),
        3, fade_in_music()
    ]
    
    text: "突然，前方闪烁着奇异的蓝光。"
    with events: [
        3, flash_effect("#0088FF")
    ]
    
    text: "你走近一看，是一池闪闪发光的泉水..."
    
    choice: [
        "谨慎地观察" -> ObserveSpring,
        "直接喝一口" -> DirectDrink,
        "离开这里" -> ChooseLeave
    ]
}

// ========== 观察分支 ==========
node ObserveSpring {
    text: "你蹲下身，仔细观察这池泉水。"
    text: "水面上浮现出古老的文字..."
    with events: [
        0, show_text_effect("ancient_runes")
    ]
    
    text: "文字说：'饮此圣泉者，将获得真知与力量。'"
    
    choice: [
        "那我就喝吧" -> CautiousDrink,
        "感觉有点可怕，还是走吧" -> ChooseLeave,
        
        // 带装备的玩家有特殊选项
        "用魔法瓶收集泉水" when has_magic_bottle() -> CollectWater
    ]
}

node CautiousDrink {
    text: "你小心翼翼地捧起一点泉水，轻轻啜了一口。"
    with events: [
        7, play_sound("drink_water.wav")
    ]
    
    text: "一股清凉的能量涌入体内！"
    with events: [
        0, screen_flash("#00FFFF"),
        0, play_sound("power_up.wav")
    ]
    
    text: $"你感觉到力量在增长... 力量值提升了 {get_power_bonus()} 点！"
    
} -> GoodEndingPower

node CollectWater {
    text: "你拿出珍贵的魔法瓶，小心地收集了泉水。"
    with events: [
        0, play_sound("bottle_fill.wav"),
        10, show_item_obtained("holy_water")
    ]
    
    text: "这可是无价之宝，关键时刻能救命！"
    
} -> GoodEndingWisdom

// ========== 直接饮用分支 ==========
node DirectDrink {
    text: "不管三七二十一，你直接痛饮了一大口！"
    with events: [
        12, play_sound("gulp.wav")
    ]
    
    text: "咕咚咕咚——"
    
    // 检查玩家是否有足够的抗性
    choice: [
        // 有抗性：没事
        "（继续）" when has_magic_resistance() -> DirectDrinkSuccess,
        
        // 没有抗性：糟糕
        "（继续）" -> DirectDrinkFail
    ]
}

node DirectDrinkSuccess {
    text: "多亏你强大的魔法抗性，泉水的力量被完美吸收了！"
    with events: [
        0, play_sound("success.wav")
    ]
    
    text: "你感到前所未有的强大！"
    
} -> GoodEndingPower

node DirectDrinkFail {
    text: "糟糕！魔力太强了，你的身体承受不住！"
    with events: [
        0, screen_shake(),
        0, play_sound("magic_overload.wav")
    ]
    
    text: "你眼前一黑，倒在了地上..."
    
} -> BadEndingUnconscious

// ========== 离开分支 ==========
node ChooseLeave {
    text: "你决定还是保持谨慎，离开这个神秘的地方。"
    
    text: "走了几步，你回头看了一眼..."
    
    text: "那池泉水的光芒渐渐暗淡，仿佛在说：'机会已失。'"
    with events: [
        18, fade_out_effect()
    ]
    
} -> NormalEndingCautious

// ========== 结局节点 ==========
node GoodEndingPower {
    text: "=== 结局：力量觉醒 ==="
    with events: [
        0, play_music("victory_theme.ogg")
    ]
    
    text: "你获得了泉水的祝福，成为了一名强大的战士！"
    text: $"最终力量：{get_final_power()}"
    text: "从此在冒险的道路上所向披靡。"
    
    text: "【游戏结束】"
}

node GoodEndingWisdom {
    text: "=== 结局：智者之路 ==="
    with events: [
        0, play_music("wisdom_theme.ogg")
    ]
    
    text: "你展现了真正的智慧，知道如何利用宝物。"
    text: "圣泉之水成为了你最珍贵的收藏。"
    text: "在后来的冒险中，这瓶水救了你无数次。"
    
    text: "【游戏结束】"
}

node BadEndingUnconscious {
    text: "=== 结局：贪婪的代价 ==="
    with events: [
        0, play_music("bad_ending.ogg"),
        0, screen_fade_black()
    ]
    
    text: "当你醒来时，已经是第二天早上。"
    text: "泉水消失了，你的力量也消失了。"
    text: "你后悔没有更加谨慎..."
    
    text: "【游戏结束】"
}

node NormalEndingCautious {
    text: "=== 结局：平凡之路 ==="
    with events: [
        0, play_music("normal_ending.ogg")
    ]
    
    text: "你选择了安全，放弃了冒险。"
    text: "虽然没有获得力量，但也没有遭遇危险。"
    text: "平平淡淡，也是一种生活方式。"
    
    text: "【游戏结束】"
}

// ========== 函数声明 ==========
// 音效与视效
fn play_ambient(filename: String)
fn play_sound(file_name: String)
fn play_music(filename: String)
fn fade_in_music()
fn fade_out_effect()
fn screen_fade_black()

// 特效
fn flash_effect(color: String)
fn screen_flash(color: String)
fn screen_shake()
fn show_text_effect(effect_name: String)
fn show_item_obtained(item_name: String)

// 条件判断
fn has_magic_bottle() -> Bool
fn has_magic_resistance() -> Bool

// 数值获取
fn get_power_bonus() -> Number
fn get_final_power() -> Number
```

## 故事结构图

```
                    开场
                     │
         ┌───────────┼───────────┐
         │           │           │
      观察泉水    直接饮用    选择离开
         │           │           │
    ┌────┼────┐      │      普通结局_谨慎
    │    │    │      │
 谨慎  收集  离开  检查抗性
 饮用  泉水       /     \
    │    │      成功    失败
    │    │       │       │
    │    │    好结局   坏结局
    └────┴───力量    _昏迷
         │
      好结局
      _智慧
```

## 关键技巧解析

### 1. 多层选择

通过条件判断实现不同玩家看到不同选项：

```mortar
choice: [
    "普通选项" -> 普通节点,
    "特殊选项" when has_special_item() -> 特殊节点
]
```

### 2. 隐藏分支

`直接饮用` 节点的处理很巧妙：

```mortar
choice: [
    // 两个选项显示文字相同
    "（继续）" when has_magic_resistance() -> 成功,
    "（继续）" -> 失败
]
```

玩家看不出区别，但结果不同——这就是隐藏分支！

### 3. 字符串插值的妙用

动态显示数值：

```mortar
text: $"力量值提升了 {get_power_bonus()} 点！"
text: $"最终力量：{get_final_power()}"
```

### 4. 事件的同步

在同一位置触发多个事件：

```mortar
with events: [
    0, screen_flash("#00FFFF"),
    0, play_sound("power_up.wav")  // 同时触发
]
```

### 5. 章节式组织

用注释分隔不同部分：

```mortar
// ========== 开场 ==========

// ========== 观察分支 ==========

// ========== 结局节点 ==========
```

让代码更易读！

## 编译和测试

```bash
# 编译
mortar forest_story.mortar -o story.json --pretty

# 检查生成的 JSON 结构
cat story.json
```

## 游戏实现要点

在游戏中需要实现：

### 1. 条件判断函数

```csharp
bool has_magic_bottle() {
    return Inventory.HasItem("magic_bottle");
}

bool has_magic_resistance() {
    return Player.Stats.MagicResistance >= 50;
}
```

### 2. 数值计算函数

```csharp
float get_power_bonus() {
    return Player.Level * 10 + 50;
}

float get_final_power() {
    return Player.Stats.Power;
}
```

### 3. 音效和特效

```csharp
void play_sound(string filename) {
    AudioManager.Play(filename);
}

void screen_flash(string color) {
    ScreenEffects.Flash(ColorUtility.TryParseHtmlString(color, out Color c) ? c : Color.white);
}
```

## 扩展建议

你可以在此基础上：

1. **增加更多分支**
   - 添加"用手触摸泉水"的选项
   - 加入"向泉水许愿"的神秘分支

2. **加入状态记录**
   - 记录玩家的选择
   - 在结局中展示玩家的决策路径

3. **多结局变体**
   - 根据之前的游戏进度解锁隐藏结局
   - 加入"真结局"需要满足特定条件

4. **配合游戏系统**
   - 结局影响后续剧情
   - 给予不同的奖励

## 小结

这个例子展示了：
- ✅ 多分支剧情设计
- ✅ 条件判断的灵活运用
- ✅ 隐藏选项和分支
- ✅ 字符串插值
- ✅ 多个结局的实现
- ✅ 音效与特效的配合

这就是 Mortar 的真正威力——让复杂的分支剧情变得清晰易管理！

## 接下来

- 想了解具体集成？看 [接入你的游戏](./5_3_game-integration.md)
- 想了解 JSON 结构？看 [JSON 输出说明](../7_1_json-output.md)
- 回到示例总览：[完整示例与讲解](./5_0_examples)
