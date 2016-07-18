# stable-borrow-state

Hack to use borrow_stable method on stable until #27733 is resolved.

```
extern crate stable_borrow_state;

use stable_borrow_state::{BorrowState, borrow_state};
use std::cell::RefCell;

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

```

## License

MIT or Apache-2.0 at your option
