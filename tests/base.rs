use event_rs_sys::{self, Watcher};

#[test]
fn base_test() {
    #[derive(Debug, PartialEq)]
    enum Event {
        A,
        B,
        C,
    }
    #[derive(Debug, PartialEq)]
    enum _Result {
        A,
        B(String),
        C,
    }
    let mut w = event_rs_sys::EventWatcher::new();
    let ret = w.add_event(Event::A, || {_Result::A}).unwrap();
    assert_eq!(w.emit(Event::A), Some(_Result::A));
    assert_eq!(w.emit(Event::B), None);
    let ret = w.add_event(Event::B, || {_Result::B("B".to_string())}).unwrap();
    assert_eq!(w.emit(Event::B), Some(_Result::B("B".to_string())));
    
}