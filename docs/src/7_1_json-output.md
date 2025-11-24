# JSON Output Format

Mortar v0.4 reorganizes the `.mortared` artifact so that every playable action appears in a single ordered stream. This chapter summarizes the v0.4 structure and shows how to consume it from tooling.

## Top-Level Layout

Each file contains the following top-level arrays and objects:

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

* `variables` mirror the `let` declarations from your script (formalized in v0.4) and ship initial values if present.
* `constants` include `pub const` entries with a `public` flag so engines can expose localized strings.
* `enums` describe the symbolic sets required by branch interpolation.

## Metadata

The metadata block records the compiler version and timestamp. Both values are strings, and `generated_at` follows ISO 8601 in UTC. Use it for compatibility checks or caches.

## Nodes and Linear Content

`nodes` is now an **array**. Each node object has:

```json
{
  "name": "Start",
  "content": [ ... ],
  "branches": [ ... ],         // optional asymmetric text tables
  "variables": [ ... ],        // optional node-scoped mutable values
  "next": "NextNode"           // optional default jump
}
```

The `content` array is the authoritative execution order. Text, inline events, choices, and `run` statements appear exactly where they were written, removing the need to merge `texts`/`runs`/`choice_position` as older guides required.

### Content Item Types

Every element inside `content` has a `type` field:

1. **`type: "text"`** — A dialogue line or interpolated string.
   - `value`: rendered text (placeholders already flattened so clients can display immediately).
   - `interpolated_parts`: optional array describing each string fragment, expression, or branch case so you can rebuild smart previews.
   - `condition`: optional AST for `if/else` guards introduced in v0.4.
   - `pre_statements`: assignments that must run before showing the line.
   - `events`: inline triggers tied to the literal characters. Each event contains an `index`, optional `index_variable`, and an `actions` array (`{ "type": "play_sound", "args": ["intro.wav"] }`).

2. **`type: "run_event"`** — Inserts a named event definition into the flow.
   - `name`: references the entry in the top-level `events` array.
   - `args`: serialized arguments passed to the underlying action.
   - `index_override`: `{ "type": "value" | "variable", "value": "..." }` to re-time the event relative to the surrounding text block.
   - `ignore_duration`: `true` when the call should fire immediately instead of respecting the event’s intrinsic `duration`.

3. **`type: "run_timeline"`** — Executes a timeline defined under `timelines`. Timelines let you orchestrate multiple `run`/`wait` statements (debuted in v0.4) and are ideal for cinematic sequences.

4. **`type: "choice"`** — Presents selectable options exactly where they are authored.
   - `options`: an array of objects with `text`, optional `next`, optional `action` (`"return"`/`"break"`), optional nested `choice` arrays, and optional `condition` blocks (function calls with arguments). This replaces the old `choices` array and no longer needs `choice_position`.

### Branch Definitions

If a node uses `$"..."` with `branch` placeholders, the compiler emits a `branches` array so clients can cache the localized pieces. Each case carries its own optional `events`, enabling the per-branch timing rules defined in v0.4.

## Named Events and Timelines

The top-level `events` array contains reusable cues:

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

`run_event` references use these definitions, so an action can appear inline (`with events`) or be invoked elsewhere without duplicating parameters.

`timelines` describe scripted sequences:

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

Each `run` statement inherits the arguments specified in the corresponding event, while `wait` pauses the playback cursor.

## Example Node

```json
{
  "name": "Start",
  "content": [
    {
      "type": "text",
      "value": "Welcome to the adventure!",
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
        { "text": "Begin", "next": "GameStart" },
        { "text": "Let me think", "action": "break" }
      ]
    }
  ],
  "next": "MainMenu"
}
```

## Parsing Tips

Strongly type your reader so that new fields are easier to adopt. The following TypeScript and Python sketches mirror the serializer output:

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

Continue modelling timelines, named events, and choice options in a similar fashion so your runtime can follow the same execution semantics as the Mortar compiler.
