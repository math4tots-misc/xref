XRef and XRefMut
================

When you want to return some data that might be either
owned or borrowed, it is common to return a `Cow<T>`.

However, `Cow<T>` has the following limitations:

* `T` has to be `ToOwned`, and
* if the data you want to borrow is behind a `RefCell`,
    you can't.

`XRef<T>` is like `Cow<T>`, except addresses the above
limitations with the following differences:

* `T` does not have to be `ToOwned`, (this means that
    now the type contained in `Borrowed` and `Owned` have to
    contain the exact matching types), and
* `XRef<T>` has a third variant `Ref` which holds a
    `std::cell::Ref<T>`, for cases when the value is borrowed
    from a `RefCell`.

`XRefMut<T>` is like `XRef<T>` except that

* all references are borrowed mutably (i.e. `XRef::Borrowed` holds
    a `&mut T` instead of `&T`, `XRef::Ref` holds a `RefMut<T>`
    instead of a `Ref<T>`),
* `XRefMut<T>` implements `DerefMut<T>`, to allow borrowing
    data mutably without cloning.
