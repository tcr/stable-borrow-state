use std::cell::{UnsafeCell, Cell, RefCell};
use std::mem;

type BorrowFlag = usize;

struct RefCellMock<T: ?Sized> {
    borrow: Cell<BorrowFlag>,
    value: UnsafeCell<T>,
}

/// An enumeration of values returned from the `state` method on a `RefCell<T>`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BorrowState {
    /// The cell is currently being read, there is at least one active `borrow`.
    Reading,
    /// The cell is currently being written to, there is an active `borrow_mut`.
    Writing,
    /// There are no outstanding borrows on this cell.
    Unused,
}

// Values [1, MAX-1] represent the number of `Ref` active
// (will not outgrow its range since `usize` is the size of the address space)
const UNUSED: BorrowFlag = 0;
const WRITING: BorrowFlag = !0;

#[inline]
pub fn borrow_state<T>(target: &RefCell<T>) -> BorrowState {
    let mock: &RefCellMock<T> = unsafe {
        mem::transmute(target)
    };
    match mock.borrow.get() {
        WRITING => BorrowState::Writing,
        UNUSED => BorrowState::Unused,
        _ => BorrowState::Reading,
    }
}

#[cfg(test)]
mod tests {
    use super::{BorrowState, borrow_state};
    use std::cell::RefCell;

    #[test]
    fn it_works() {
        struct Dummy {
            a: Vec<bool>
        }

        let a = RefCell::new(Dummy {
            a: vec![true, false, false, false]
        });
        assert_eq!(borrow_state(&a), BorrowState::Unused);

        {
            let mut b = a.borrow_mut();
            b.a.push(true);
            assert_eq!(borrow_state(&a), BorrowState::Writing);
        }

        assert_eq!(borrow_state(&a), BorrowState::Unused);

        {
            let b = a.borrow();
            assert_eq!(borrow_state(&a), BorrowState::Reading);
            let c = a.borrow();
            assert_eq!(borrow_state(&a), BorrowState::Reading);

            drop(b);
            drop(c);
        }

        assert_eq!(borrow_state(&a), BorrowState::Unused);
    }
}
