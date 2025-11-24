# Event System and Timelines

Events have always been core to Mortar, but v0.4 expands them into first-class definitions that you can reuse or orchestrate through timelines. This section mirrors `examples/performance_system.mortar`, which demonstrates every feature end to end.

## Declaring Named Events

Use the `event` keyword at the top level:

```mortar
event SetColor {
    index: 0
    action: set_color("#228B22")
}

event MoveRight {
    action: set_animation("right")
    duration: 2.0
}

event MoveLeft {
    action: set_animation("left")
    duration: 1.5
}

event PlaySound {
    index: 20
    action: play_sound("dramatic-hit.wav")
}
```

Each event can define a default `index`, an action, and an optional `duration`. In the serialized JSON it appears under the global `events` array so engines can trigger it anywhere.

## Running Events Inline

Inside a node you have two primary tools:

1. **`run EventName`** executes the event immediately, ignoring the stored index (duration still matters unless you explicitly override it).
2. **`with EventName`** or **`with events: [ EventA EventB ]`** attaches events to the previous `text` block so their indices line up with characters.

```mortar
node WithEventsExample {
    text: "Welcome to the adventure!"
    with SetColor  // single event shortcut

    text: "The forest comes alive with sound and color."
    with events: [
        MoveRight
        PlaySound
    ]
}
```

When Mortar serializes this node, inline associations become `events` arrays nested under the corresponding `text` content items.

## Custom Indices

You can override an event’s index at runtime while still using the same definition. This is how `examples/performance_system.mortar` lines up `PlaySound` with specific characters:

```mortar
let custom_time: Number = 5

node CustomIndexExample {
    text: "Be quiet...until you hear...Blast!"
    run PlaySound with custom_time      // immediate run, index override stored for metadata

    custom_time = 28

    // Attach the run to the text so the sound waits until character 28
    with run PlaySound with custom_time
}
```

Any `run ... with <NumberOrVariable>` statement becomes a `ContentItem::RunEvent` with `index_override` populated in the `.mortared` file. Prepending `with` means “attach this run to the previous text block”.

## Timelines

Timelines (`timeline OpeningCutscene { ... }`) provide a scriptable playlist of `run`, `wait`, and `now run` statements:

```mortar
timeline OpeningCutscene {
    run MoveRight
    wait 1.0
    run MoveLeft
    wait 0.5
    now run PlaySound   // ignore PlaySound's duration and continue immediately
    wait 10
    run SetColor
}
```

Inside a node you can call `run OpeningCutscene` to execute the entire sequence:

```mortar
node TimelineExample {
    text: "Watch the opening cutscene..."
    run OpeningCutscene
}
```

Timelines are perfect for cutscenes, choreographed animations, or any moment where several systems must stay in sync.

## Practical Tips

- Define reusable events for anything more complex than a one-off inline action.
- Use `with` to tie events to a specific text block; use `run` for global beats.
- Override indices when the same cue needs different timing in each context.
- Combine timelines with `now run` if you need to skip an event’s duration.

The v0.4 JSON exposes both `events` and `timelines` arrays so tooling and engines can inspect or reuse staging logic independent of text content.
