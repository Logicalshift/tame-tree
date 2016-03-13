//! # CloneCell
//!
//! Variant of std::rc::Cell that works using cloning instead of copying.

use std::cell::*;

pub struct CloneCell<TContentType> {
    content: UnsafeCell<TContentType>
}

impl<TContentType:Clone> CloneCell<TContentType> {
    #[inline]
    pub fn new(value: TContentType) -> CloneCell<TContentType> {
        CloneCell { content: UnsafeCell::new(value.to_owned()) }
    }

    #[inline]
    pub fn get(&self) -> TContentType {
        unsafe { (*self.content.get()).to_owned() }
    }

    #[inline]
    pub fn set(&self, new_value: TContentType) {
        unsafe { *self.content.get() = new_value.to_owned() }
    }
}

impl<TContentType:Clone> Clone for CloneCell<TContentType> {
    fn clone(&self) -> CloneCell<TContentType> {
        CloneCell::new(self.get())
    }
}

#[cfg(test)]
mod clonecell_tests {
    use std::rc::*;
    use std::cell::*;
    use super::*;

    // This would be much easier to test if it was possible to read out the reference count for Rc (we could check it's behaving as expected)
    // However, that feature is considered 'not clearly useful' so we have to reimplement everything to make sure that reference counts
    // get updated properly. 

    struct RefCount {
        count: i32
    }

    type RefCountRef = Rc<Cell<RefCount>>;

    impl RefCount {
        fn new() -> RefCount {
            RefCount { count: 0 }
        }

        fn inc(&self) -> RefCount {
            RefCount {  count: self.count+1 }
        }

        fn dec(&self) -> RefCount {
            RefCount { count: self.count-1 }
        }

        fn get(&self) -> i32 {
            self.count
        }
    }

    struct Droppable {
        counter: RefCountRef
    }

    impl Copy for RefCount {
    }

    impl Clone for RefCount {
        fn clone(&self) -> RefCount {
            RefCount { count: self.count }
        }
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            (*self.counter).set(self.counter.get().dec());
        }
    }

    impl Clone for Droppable {
        fn clone(&self) -> Droppable {
            (*self.counter).set(self.counter.get().inc());
            Droppable { counter: self.counter.clone() }
        }
    }

    impl Droppable {
        fn new(counter: &RefCountRef) -> Droppable {
            (*counter).set((*counter).get().inc());
            Droppable { counter: counter.clone() }
        }
    }

    #[test]
    fn clonecell_update_rc() {
        let ref_count = Rc::new(Cell::new(RefCount::new()));
        let ref_count2 = Rc::new(Cell::new(RefCount::new()));

        assert!(ref_count.get().get() == 0);
        assert!(ref_count2.get().get() == 0);

        {
            let rc_standin = Droppable::new(&ref_count);

            assert!(ref_count.get().get() == 1);

            let arefcell = CloneCell::new(rc_standin.clone());
            assert!(ref_count.get().get() == 2);

            let rc_standin2 = Droppable::new(&ref_count2);

            assert!(ref_count2.get().get() == 1);

            arefcell.set(rc_standin2.clone());
            assert!(ref_count2.get().get() == 2);
            assert!(ref_count.get().get() == 1);
        }

        assert!(ref_count.get().get() == 0);
        assert!(ref_count2.get().get() == 0);
    }
}
