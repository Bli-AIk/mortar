use crate::parser::ParseHandler;
use crate::serializer::Serializer;
use serde_json::Value;

#[test]
fn test_serialize_event_def() {
    let source = r#"
        event Basic {
            index: 0
            action: set_background("bg_forest.png")
        }
        
        fn set_background(file: String)
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();
    let json = Serializer::serialize_to_json(&program, true).unwrap();
    let value: Value = serde_json::from_str(&json).unwrap();

    let events = value["events"].as_array().unwrap();
    assert_eq!(events.len(), 1);

    let event = &events[0];
    assert_eq!(event["name"], "Basic");
    assert_eq!(event["index"], 0.0);
    assert_eq!(event["action"]["type"], "set_background");
}

#[test]
fn test_serialize_event_with_duration() {
    let source = r#"
        event ShowAlice {
            action: show_character("alice.png", "center")
            duration: 2.0
        }
        
        fn show_character(image: String, position: String)
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();
    let json = Serializer::serialize_to_json(&program, true).unwrap();
    let value: Value = serde_json::from_str(&json).unwrap();

    let events = value["events"].as_array().unwrap();
    let event = &events[0];
    assert_eq!(event["name"], "ShowAlice");
    assert_eq!(event["duration"], 2.0);
    assert!(event["index"].is_null());
}

#[test]
fn test_serialize_timeline_def() {
    let source = r#"
        timeline IntroScene {
            run ShowAlice
            wait 2.0
            run PlayMusic
        }
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();
    let json = Serializer::serialize_to_json(&program, true).unwrap();
    let value: Value = serde_json::from_str(&json).unwrap();

    let timelines = value["timelines"].as_array().unwrap();
    assert_eq!(timelines.len(), 1);

    let timeline = &timelines[0];
    assert_eq!(timeline["name"], "IntroScene");

    let statements = timeline["statements"].as_array().unwrap();
    assert_eq!(statements.len(), 3);

    assert_eq!(statements[0]["type"], "run");
    assert_eq!(statements[0]["event_name"], "ShowAlice");

    assert_eq!(statements[1]["type"], "wait");
    assert_eq!(statements[1]["duration"], 2.0);

    assert_eq!(statements[2]["type"], "run");
    assert_eq!(statements[2]["event_name"], "PlayMusic");
}

#[test]
fn test_serialize_timeline_with_args() {
    let source = r#"
        timeline IntroScene {
            run DialogueNode("Start")
        }
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();
    let json = Serializer::serialize_to_json(&program, true).unwrap();
    let value: Value = serde_json::from_str(&json).unwrap();

    let timelines = value["timelines"].as_array().unwrap();
    let timeline = &timelines[0];
    let statements = timeline["statements"].as_array().unwrap();

    assert_eq!(statements[0]["type"], "run");
    assert_eq!(statements[0]["event_name"], "DialogueNode");

    let args = statements[0]["args"].as_array().unwrap();
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], "\"Start\"");
}

#[test]
fn test_serialize_multiple_events_and_timelines() {
    let source = r#"
        event EventA {
            index: 0
            action: func_a("a")
        }
        
        event EventB {
            action: func_b("b")
            duration: 1.5
        }
        
        timeline TimelineA {
            run EventA
        }
        
        timeline TimelineB {
            wait 1.0
            run EventB
        }
        
        fn func_a(arg: String)
        fn func_b(arg: String)
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();
    let json = Serializer::serialize_to_json(&program, true).unwrap();
    let value: Value = serde_json::from_str(&json).unwrap();

    let events = value["events"].as_array().unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0]["name"], "EventA");
    assert_eq!(events[1]["name"], "EventB");

    let timelines = value["timelines"].as_array().unwrap();
    assert_eq!(timelines.len(), 2);
    assert_eq!(timelines[0]["name"], "TimelineA");
    assert_eq!(timelines[1]["name"], "TimelineB");
}
