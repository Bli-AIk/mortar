# JSON 输出说明

Mortar v0.4 将 `.mortared` 文件整理成一条有序的执行流。本章对新的结构做英文、中文并行的总结，方便引擎按顺序重放内容。

## 顶层结构

```json
{
  "metadata": { "version": "0.4.0", "generated_at": "2025-01-31T12:00:00Z" },
  "variables": [ ... ],
  "constants": [ ... ],
  "enums": [ ... ],
  "nodes": [ ... ],
  "functions": [ ... ],
  "events": [ ... ],
  "timelines": [ ... ]
}
```

- `variables` 对应脚本中的 `let` 声明（v0.4 中正式引入），初始值会直接写入。
- `constants` 保存 `pub const`，并带有 `public` 标记，便于导出本地化文本。
- `enums` 记录枚举及其分支，供 branch 插值使用。

## metadata

`metadata` 中包含 `version` 与 `generated_at`，两者均为字符串。时间戳遵循 UTC 的 ISO 8601 格式，可用于缓存或回滚判定。

## 节点与线性内容

`nodes` 现在是 **数组** 而不再是字典。每个节点形如：

```json
{
  "name": "Start",
  "content": [ ... ],
  "branches": [ ... ],
  "variables": [ ... ],
  "next": "NextNode"
}
```

`content` 数组就是对话/事件的真实顺序，已经把旧版本的 `texts`、`runs`、`choice_position` 全部折叠到一起，引擎只需从头到尾迭代即可。

### 内容项（Content Item）类型

所有元素都拥有 `type` 字段：

1. **`type: "text"`** — 对话行或插值文本。
   - `value`：可直接显示的字符串。
   - `interpolated_parts`：按片段描述文本/表达式/branch case，方便编辑器回放。
   - `condition`：当该行来自 `if/else` 时记录完整条件树。
   - `pre_statements`：在显示前需要执行的赋值语句。
   - `events`：与具体字符位置绑定的事件，元素格式为 `{ "index": 4.2, "index_variable": null, "actions": [{ "type": "set_color", "args": ["#FF6B6B"] }] }`。

2. **`type: "run_event"`** — 调用顶层 `events` 中的命名事件。
   - `name`：事件名。
   - `args`：序列化后的参数。
   - `index_override`：`{ "type": "value" | "variable", "value": "..." }`，用于重写触发位置（可绑定变量）。
   - `ignore_duration`：为 `true` 时忽略事件自带的 `duration`，立即执行。

3. **`type: "run_timeline"`** — 执行一条 `timelines` 描述的演出序列（参见计划文档第 5 节的演出系统）。

4. **`type: "choice"`** — 在脚本写入的位置展示选项。
   - `options` 数组中，每个选项拥有 `text`、可选的 `next`、可选的 `action`（`"return"` 或 `"break"`）、可选的嵌套 `choice`、以及可选的 `condition`（函数名与参数）。这完全取代了旧版 `choices`/`choice_position`。

### Branch 定义

若节点包含 `$"..."` 的 `branch` 插值，编译器会在节点对象中生成 `branches`。每个 case 自带文本与可选 `events`，满足 v0.4 中“分支插值拥有独立索引”的要求。

## 命名事件与时间线

顶层 `events` 记录可复用的事件：

```json
{
  "name": "ColorYellow",
  "index": 1.0,
  "duration": 0.35,
  "action": {
    "type": "set_color",
    "args": ["#FFFF00"]
  }
}
```

在节点里 `run_event` 调用这个定义，从而保证“with events”与“单独运行”两种场景使用同一套参数。

`timelines` 用于复杂演出：

```json
{
  "name": "IntroScene",
  "statements": [
    { "type": "run", "event_name": "ShowAlice" },
    { "type": "wait", "duration": 2.0 },
    { "type": "run", "event_name": "PlayMusic", "ignore_duration": true }
  ]
}
```

`run` 会触发指定事件，`wait` 则暂停光标，可组合出 Unity Timeline 风格的序列。

## 节点示例

```json
{
  "name": "Start",
  "content": [
    {
      "type": "text",
      "value": "欢迎来到冒险！",
      "events": [
        {
          "index": 0,
          "actions": [{ "type": "play_music", "args": ["intro.mp3"] }]
        }
      ]
    },
    {
      "type": "choice",
      "options": [
        { "text": "开始", "next": "GameStart" },
        { "text": "再想想", "action": "break" }
      ]
    }
  ],
  "next": "MainMenu"
}
```

## 解析建议

建议使用强类型来建模 `.mortared` 文件，便于在扩展字段时快速升级。下面给出与序列化代码一致的 TypeScript/Python 草图：

```typescript
type ContentItem =
  | {
      type: "text";
      value: string;
      interpolated_parts?: StringPart[];
      condition?: Condition;
      pre_statements?: Statement[];
      events?: EventTrigger[];
    }
  | {
      type: "run_event";
      name: string;
      args?: string[];
      index_override?: { type: "value" | "variable"; value: string };
      ignore_duration?: boolean;
    }
  | { type: "run_timeline"; name: string }
  | { type: "choice"; options: ChoiceOption[] };

interface MortaredFile {
  metadata: Metadata;
  variables: VariableDecl[];
  constants: ConstantDecl[];
  enums: EnumDecl[];
  nodes: Node[];
  functions: FunctionDecl[];
  events: EventDef[];
  timelines: TimelineDef[];
}
```

```python
@dataclass
class EventTrigger:
    index: float
    actions: List[Action]
    index_variable: Optional[str] = None

@dataclass
class ContentText:
    type: Literal["text"]
    value: str
    interpolated_parts: Optional[List[StringPart]] = None
    condition: Optional[Condition] = None
    pre_statements: Optional[List[Statement]] = None
    events: Optional[List[EventTrigger]] = None

@dataclass
class ContentRunEvent:
    type: Literal["run_event"]
    name: str
    args: List[str] = field(default_factory=list)
    index_override: Optional[IndexOverride] = None
    ignore_duration: bool = False

@dataclass
class ContentChoice:
    type: Literal["choice"]
    options: List[ChoiceOption]
```

依照同样的方式定义 timelines、命名事件与选项结构，就能让运行时代码和 Mortar 编译器保持一致。
