use hash_map::HashMap;
use core::hash;
use core::cmp;

pub type DispatchFn<T> = fn (&T) -> bool;


pub struct DispatchTable<T: cmp::Eq + hash::Hash> {
    table: HashMap<T, DispatchFn<T>>,
}

impl<T: cmp::Eq + hash::Hash> DispatchTable<T> {

    pub fn new() -> Self{
        DispatchTable{ table: HashMap::new(16) }
    }

    pub fn dispatch(&mut self, event: &T) -> bool {
        match self.table.get(event) {
            Some(func) => func(event),
            None => false,
        }
    }

    pub fn register(&mut self, event: T, func: DispatchFn<T>) {
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

    fn on_vmexit(_: &Event) -> bool {
        true
    }

    #[test]
    fn test_dispatch_table_register() {
        let mut dt = DispatchTable::<Event>::new();
        dt.register(Event::VMExit, on_vmexit);
        assert!(dt.dispatch(&Event::VMExit));
        assert!(!dt.dispatch(&Event::PageFault));
    }

    #[test]
    fn test_dispatch_table_unregister() {
        let mut dt = DispatchTable::<Event>::new();
        dt.register(Event::VMExit, on_vmexit);
        assert!(dt.dispatch(&Event::VMExit));
        assert!(!dt.dispatch(&Event::PageFault));
        dt.unregister(Event::VMExit);
        assert!(!dt.dispatch(&Event::VMExit));
    }

}
