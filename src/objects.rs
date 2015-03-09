use std::mem::{forget, transmute};
use std::ops::Deref;
use raw;

pub enum PyObj {}
pub enum PyTuple {}
pub enum PyNone {}

pub struct PyBox<T: PyVal>(*mut T);

pub trait PyVal: Sized {
    #[inline(always)]
    fn ptr(&self) -> *mut raw::PyObject {
        self as *const Self
             as *mut Self
             as *mut raw::PyObject
    }

    #[inline(always)]
    fn downgrade(&self) -> &PyObj {
        unsafe { transmute(self) }
    }

    fn upgrade_from(&PyObj) -> Option<&Self>;

    #[inline(always)]
    fn take(&self) -> PyBox<Self> {
        unsafe {
            raw::Py_INCREF(self.ptr());
            PyBox(self as *const Self as *mut Self)
        }
    }
}

impl<T: PyVal> PyBox<T> {
    #[inline(always)]
    pub fn downgrade(self) -> PyBox<PyObj> {
        unsafe { transmute(self) }
    }
}

impl PyObj {
    #[inline(always)]
    pub fn upgrade<T: PyVal>(&self) -> Option<&T> {
        PyVal::upgrade_from(self)
    }

    #[inline(always)]
    pub unsafe fn from_ptr<'a>(ptr: *mut raw::PyObject) -> &'a PyObj {
        transmute(ptr)
    }
}

impl PyBox<PyObj> {
    #[inline(always)]
    pub fn upgrade<T: PyVal>(self) -> Result<PyBox<T>, PyBox<PyObj>> {
        if let Some(_) = (*self).upgrade::<T>() {
            Ok(unsafe { transmute(self) })
        } else {
            Err(self)
        }
    }

    #[inline(always)]
    pub unsafe fn from_ptr(ptr: *mut raw::PyObject) -> PyBox<PyObj> {
        PyBox(ptr as *mut PyObj)
    }

    #[inline(always)]
    pub unsafe fn into_ptr(self) -> *mut raw::PyObject {
        let res = self.0;
        forget(self);
        res as *mut raw::PyObject
    }
}

impl<T: PyVal> Deref for PyBox<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        let v : *mut T = self.0;
        unsafe { transmute(v) }
    }
}

#[unsafe_destructor]
impl<T: PyVal> Drop for PyBox<T> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            raw::Py_DECREF((*self.0).ptr());
            self.0 = 0 as *mut T;
        }
    }
}

impl PyVal for PyObj {
    #[inline(always)]
    fn upgrade_from(obj : &PyObj) -> Option<&PyObj> {
        Some(obj)
    }
}

impl PyVal for PyTuple {
    #[inline(always)]
    fn upgrade_from(obj : &PyObj) -> Option<&PyTuple> {
        unsafe {
            if raw::PyTuple_CheckExact(obj.ptr()) != 0 {
                Some(transmute(obj))
            } else {
                None
            }
        }
    }
}

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
