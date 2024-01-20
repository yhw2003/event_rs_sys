pub struct EventWatcher<T,R> {
    handled_event: Vec<(T, Box<dyn Fn() -> R>)>,
}

impl<T,R> EventWatcher<T,R> {
    pub fn new() -> Self {
        Self {
            handled_event: vec![],
        }
    }
}


pub trait Watcher<T, R> {
    type E;
    fn add_event(&mut self, t: T, handle: impl Fn() -> R + 'static) -> Result<(), Self::E>;
    fn emit(&mut self, t: T) -> Option<R>;
}

impl<T,R> Watcher<T, R> for EventWatcher<T, R>
where T: PartialEq
{
    type E = ();

    fn add_event(&mut self, t: T, handle: impl Fn() -> R + 'static) -> Result<(), Self::E> {
        self.handled_event.push((t, Box::new(handle)));
        Ok(())
    }

    fn emit(&mut self,t: T) -> Option<R> {
        for (event, handle) in &self.handled_event {
            if event == &t {
                return Some(handle());
            }
        }
        None
    }
}