extern crate "python27-sys" as raw;
use object::{PyBox, PyVal, PyObj, ToPython};
use std::mem::transmute;

pub enum PyNone {}

impl PyVal for PyNone {
    #[inline(always)]
    fn upgrade_from(obj: &PyObj) -> Option<&PyNone> {
        unsafe {
            if (obj as *const PyObj as *const raw::PyObject) == raw::Py_None() as *const raw::PyObject {
                Some(transmute(obj))
            } else {
                None
            }
        }
    }
}


#[inline(always)]
pub fn none() -> &'static PyNone {
    unsafe {
        transmute(raw::Py_None())
    }
}

impl<T: ToPython> ToPython for Option<T> {
    fn python(&self) -> PyBox<PyObj> {
        match self {
            &Some(ref v) => v.python(),
            &None => none().take().downgrade()
        }
    }
}
