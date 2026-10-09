use super::*;

// The adapter reports two elapsed values, and they are not interchangeable:
// `elapsedTime` is a snapshot taken at `timestamp`, while `elapsedTimeNow` is
// the position at read time (the adapter only emits it for `get --now`). A
// backend that is playing can still report `elapsedTime: 0` — measured on
// NetEase on macOS 26 — so the snapshot alone makes a playing track look like
// it is stuck at zero.
//
// The fixtures below are trimmed copies of real adapter output.

#[test]
fn parse_get_prefers_elapsed_time_now() {
  // Real output captured while a track was playing: the snapshot is 0 and the
  // live value is the only usable one.
  let json = r#"{"playing":true,"playbackRate":1,"elapsedTime":0,"elapsedTimeNow":9.4306888580322266}"#;
  let state = parse_get(json).expect("parse");
  assert_eq!(state.elapsed_seconds, Some(9.4306888580322266));
}

#[test]
fn parse_get_falls_back_to_the_snapshot() {
  // Without `--now` the adapter omits `elapsedTimeNow` entirely, so older
  // adapter output and pinned tracks must keep working.
  let json = r#"{"playing":false,"playbackRate":0,"elapsedTime":177.024}"#;
  let state = parse_get(json).expect("parse");
  assert_eq!(state.elapsed_seconds, Some(177.024));
}

#[test]
fn parse_get_reads_both_fields_while_paused() {
  // While paused the two values agree (elapsed + 0 × rate), so preferring the
  // live one is not a behaviour change here.
  let json = r#"{"playing":false,"playbackRate":0,"elapsedTime":177.024,"elapsedTimeNow":177.024}"#;
  let state = parse_get(json).expect("parse");
  assert_eq!(state.elapsed_seconds, Some(177.024));
}

#[test]
fn parse_get_treats_explicit_nulls_as_absent() {
  let json = r#"{"playing":true,"elapsedTime":null,"elapsedTimeNow":null}"#;
  let state = parse_get(json).expect("parse");
  assert_eq!(state.elapsed_seconds, None);
}

#[test]
fn parse_get_null_literal_is_the_idle_state() {
  // The adapter emits the bare literal `null` when nothing owns the slot.
  let state = parse_get("null").expect("parse");
  assert_eq!(state, NowPlayingState::default());
}
