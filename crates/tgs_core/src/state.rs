//! Globally accessible state store

use anymap::Map;

pub struct State {
    store: Map<dyn anymap::any::Any>,
}

impl Default for State {
    fn default() -> Self {
        State { store: Map::new() }
    }
}

impl State {
    pub fn insert<T: 'static>(&mut self, data: T) {
        self.store.insert::<T>(data);
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.store.get::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.store.get_mut::<T>()
    }

    /// Get data or return default if not exist
    ///
    /// Also inserts default into state store to ensure future gets don't fail
    pub fn get_or_default<T: 'static + Default>(&mut self) -> &T {
        if !self.store.contains::<T>() {
            self.insert(T::default());
        }

        // TODO i think this is safe?
        self.get::<T>().unwrap()
    }

    /// Get data or return default if not exist
    ///
    /// Also inserts default into state store to ensure future gets don't fail
    pub fn get_mut_or_default<T: 'static + Default>(&mut self) -> &mut T {
        if !self.store.contains::<T>() {
            self.insert(T::default());
        }

        self.get_mut::<T>().unwrap()
    }
}
