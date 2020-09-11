use matrix_canonical_json::to_canonical_string;
use criterion::{criterion_group, criterion_main, Criterion};

fn power_levels() -> serde_json::Value {
    serde_json::json!({
        "content": {
            "ban": 50,
            "events": {
                "m.room.avatar": 50,
                "m.room.canonical_alias": 50,
                "m.room.history_visibility": 100,
                "m.room.name": 50,
                "m.room.power_levels": 100
            },
            "events_default": 0,
            "invite": 0,
            "kick": 50,
            "redact": 50,
            "state_default": 50,
            "users": {
                "@example:localhost": 100
            },
            "users_default": 0
        },
        "event_id": "$15139375512JaHAW:localhost",
        "origin_server_ts": 45,
        "sender": "@example:localhost",
        "room_id": "!room:localhost",
        "state_key": "",
        "type": "m.room.power_levels",
        "unsigned": {
            "age": 45
        }
    })
}

fn serialize_canonical(c: &mut Criterion) {
    let json_data = serde_json::from_value::<ruma::events::room::power_levels::PowerLevelsEvent>(power_levels()).unwrap();

    c.bench_function("serialize canonical T -> String", |b| {
        b.iter(|| {
            let _ = to_canonical_string(&json_data).unwrap();
        })
    });
}

fn serialize_roundtrip(c: &mut Criterion) {
    let json_data = serde_json::from_value::<ruma::events::room::power_levels::PowerLevelsEvent>(power_levels()).unwrap();

    c.bench_function("roundtrip T -> serde_json::Value -> String", |b| {
        b.iter(|| {
            let json = serde_json::to_value(&json_data).unwrap();
            let _ = serde_json::to_string(&json).unwrap();
        })
    });
}

criterion_group!(
    benches,
    serialize_canonical,
    serialize_roundtrip
);

criterion_main!(benches);
