use hash_map::HashMap;
use core::hash;
use core::cmp;

pub type DispatchFn<T, U> = fn(T, &mut U) -> bool;


pub struct DispatchTable<T: cmp::Eq + hash::Hash, U> {
    table: HashMap<T, DispatchFn<T, U>>,
}

impl<T: cmp::Eq + hash::Hash, U> DispatchTable<T, U> {
    pub fn new(size: usize) -> Self {
        DispatchTable { table: HashMap::new(size) }
    }

    pub fn dispatch(&self, event: T, arguments: &mut U) -> bool {
        match self.table.get(&event) {
            Some(func) => func(event, arguments),
            None => false,
        }
    }

    pub fn register(&mut self, event: T, func: DispatchFn<T, U>) {
        self.table.insert(event, func);
    }

    pub fn unregister(&mut self, event: T) {
        self.table.remove(event);
    }
}

#[cfg(test)]
mod test {
    use super::DispatchTable;

    #[derive(Hash, Eq, PartialEq, Debug)]
    enum Event {
        VMExit,
        PageFault,
    }

    fn on_vmexit(_: &Event, received: bool) -> bool {
        assert!(received);
        true
    }

    #[test]
    fn test_dispatch_table_register() {
        let mut dt = DispatchTable::<Event, bool>::new(16);
        dt.register(Event::VMExit, on_vmexit);
        assert!(dt.dispatch(&Event::VMExit, true));
        assert!(!dt.dispatch(&Event::PageFault, false));
    }

    #[test]
    fn test_dispatch_table_unregister() {
        let mut dt = DispatchTable::<Event, bool>::new(16);
        dt.register(Event::VMExit, on_vmexit);
        assert!(dt.dispatch(&Event::VMExit, true));
        assert!(!dt.dispatch(&Event::PageFault, false));
        dt.unregister(Event::VMExit);
        assert!(!dt.dispatch(&Event::VMExit, false));
    }

}
