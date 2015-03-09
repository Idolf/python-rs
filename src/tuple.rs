extern crate "python27-sys" as raw;
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};
use object::{PyVal, PyObj, PyBox, ToPython};
use std::mem::transmute;
use std::slice;

pub enum PyTuple {}

impl PyVal for PyTuple {
    #[inline(always)]
    fn upgrade_from(obj : &PyObj) -> Option<&PyTuple> {
        unsafe {
            if raw::PyTuple_CheckExact(obj.to_ptr()) != 0 {
                Some(transmute(obj))
            } else {
                None
            }
        }
    }
}

impl PyTuple {
    #[inline(always)]
    unsafe fn raw(&self) -> &raw::PyTupleObject {
        transmute(self.to_ptr())
    }

    #[inline(always)]
    unsafe fn items(&self) -> *const &PyObj {
        &self.raw().ob_item as *const *mut raw::PyObject as *const &PyObj
    }

    #[inline(always)]
    pub fn get(&self, index: isize) -> Option<&PyObj> {
        match normalize_index(index, self.len()) {
            IndexVal::Okay(n) => unsafe { Some(*self.items().offset(n)) },
            _ => None
        }
    }

    #[inline(always)]
    pub fn len(&self) -> isize {
        unsafe {
            if self.raw().ob_size < 0 {
                panic!("Invalid tuple length")
            } else {
                self.raw().ob_size as isize
            }
        }
    }

    #[inline(always)]
    pub fn new<T: ToPython>(vals: &[T]) -> PyBox<PyTuple> {
        unsafe {
            let ptr = raw::PyTuple_New(vals.len() as raw::Py_ssize_t);
            if ptr.is_null() { panic!("Out of memory!") }
            for (n, v) in vals.iter().enumerate() {
                raw::PyTuple_SET_ITEM(ptr, n as raw::Py_ssize_t, v.python().into_ptr())
            }
            PyBox::from_ptr(ptr)
        }
    }
}


enum IndexVal {
    Okay(isize),
    Small,
    Large
}

#[inline(always)]
fn normalize_index(index: isize, size: isize) -> IndexVal {
    if 0 <= index {
        if index < size {
            IndexVal::Okay(index)
        } else {
            IndexVal::Large
        }
    } else {
        if 0 <= size + index {
            IndexVal::Okay(size + index)
        } else {
            IndexVal::Small
        }
    }
}

const NONE: &'static Option<&'static PyObj> = &None;

impl<'py> Index<isize> for &'py PyTuple {
    type Output = Option<&'py PyObj>;

    #[inline(always)]
    fn index(&self, &index: &isize) -> &Option<&'py PyObj> {
        match normalize_index(index, self.len()) {
            IndexVal::Okay(n) => unsafe {
                transmute(self.items().offset(n))
            },
            _ => NONE,
        }
    }
}


const EMPTY: &'static [&'static PyObj] = &[];

impl<'py> Index<Range<isize>> for &'py PyTuple {
    type Output = [&'py PyObj];

    #[inline(always)]
    fn index(&self, range: &Range<isize>) -> &[&'py PyObj] {
        let start = match normalize_index(range.start, self.len()) {
            IndexVal::Small   => 0,
            IndexVal::Okay(n) => n,
            IndexVal::Large   => return EMPTY
        };
        let end = match normalize_index(range.end, self.len()) {
            IndexVal::Small   => return EMPTY,
            IndexVal::Okay(n) => n,
            IndexVal::Large   => self.len()
        };

        if start < end {
            unsafe {
                slice::from_raw_parts(self.items().offset(start), (end - start) as usize)
            }
        } else {
            EMPTY
        }
    }
}

impl<'py> Index<RangeFrom<isize>> for &'py PyTuple {
    type Output = [&'py PyObj];

    #[inline(always)]
    fn index(&self, range: &RangeFrom<isize>) -> &[&'py PyObj] {
        self.index(&Range { start: range.start, end: self.len() })
    }
}

impl<'py> Index<RangeTo<isize>> for &'py PyTuple {
    type Output = [&'py PyObj];

    #[inline(always)]
    fn index(&self, range: &RangeTo<isize>) -> &[&'py PyObj] {
        self.index(&Range { start: 0, end: range.end })
    }
}

impl<'py> Index<RangeFull> for &'py PyTuple {
    type Output = [&'py PyObj];

    #[inline(always)]
    fn index(&self, _range: &RangeFull) -> &[&'py PyObj] {
        self.index(&Range { start: 0, end: self.len() })
    }
}

impl ToPython for () {
    #[inline(always)]
    fn python(&self) -> PyBox<PyObj> {
        PyTuple::new::<PyObj>(&[]).downgrade()
    }
}

impl<A> ToPython for (A,) where A: ToPython {
    #[inline(always)]
    fn python(&self) -> PyBox<PyObj> {
        PyTuple::new(&[
            self.0.python()
        ]).downgrade()
    }
}

impl<A,B> ToPython for (A,B) where A: ToPython, B: ToPython {
    #[inline(always)]
    fn python(&self) -> PyBox<PyObj> {
        PyTuple::new(&[
            self.0.python(),
            self.1.python()
        ]).downgrade()
    }
}

impl<A,B,C> ToPython for (A,B,C) where A: ToPython, B: ToPython, C: ToPython {
    #[inline(always)]
    fn python(&self) -> PyBox<PyObj> {
        PyTuple::new(&[
            self.0.python(),
            self.1.python(),
            self.2.python(),
        ]).downgrade()
    }
}
