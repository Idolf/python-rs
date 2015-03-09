extern crate "python27-sys" as raw;
use std::mem::{forget, transmute};
use std::ops::Deref;

pub enum PyObj {}
pub struct PyBox<T: PyVal>(*mut T);

pub trait PyVal: Sized {
    #[inline(always)]
    unsafe fn to_ptr(&self) -> *mut raw::PyObject {
        self as *const Self
             as *mut Self
             as *mut raw::PyObject
    }

    #[inline(always)]
    unsafe fn from_ptr<'a>(ptr: *mut raw::PyObject) -> &'a PyObj {
        transmute(ptr)
    }

    #[inline(always)]
    fn downgrade(&self) -> &PyObj {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    fn take(&self) -> PyBox<Self> {
        unsafe {
            raw::Py_INCREF(self.to_ptr());
            PyBox(self as *const Self as *mut Self)
        }
    }

    #[inline(always)]
    fn upgrade_from(&PyObj) -> Option<&Self>;
}

impl<T: PyVal> PyBox<T> {
    #[inline(always)]
    pub fn downgrade(self) -> PyBox<PyObj> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub unsafe fn from_ptr(ptr: *mut raw::PyObject) -> PyBox<T> {
        PyBox(ptr as *mut T)
    }

    #[inline(always)]
    pub unsafe fn into_ptr(self) -> *mut raw::PyObject {
        let res = self.0;
        forget(self);
        res as *mut raw::PyObject
    }

    #[inline(always)]
    pub fn upgrade_from(val: PyBox<PyObj>) -> Result<PyBox<T>, PyBox<PyObj>> {
        if let Some(_) = (*val).upgrade::<T>() {
            Ok(unsafe { transmute(val) })
        } else {
            Err(val)
        }
    }
}

impl<T: PyVal> Clone for PyBox<T> {
    #[inline(always)]
    fn clone(&self) -> PyBox<T> {
        self.take()
    }
}

impl PyObj {
    #[inline(always)]
    pub fn upgrade<T: PyVal>(&self) -> Option<&T> {
        PyVal::upgrade_from(self)
    }
}

impl PyBox<PyObj> {
    #[inline(always)]
    pub fn upgrade<T: PyVal>(self) -> Result<PyBox<T>, PyBox<PyObj>> {
        PyBox::upgrade_from(self)
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
            raw::Py_DECREF((*self.0).to_ptr());
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

pub trait ToPython {
    fn python(&self) -> PyBox<PyObj>;
}

impl<T: PyVal> ToPython for T {
    #[inline(always)]
    fn python(&self) -> PyBox<PyObj> {
        self.take().downgrade()
    }
}

impl<T: PyVal> ToPython for PyBox<T> {
    #[inline(always)]
    fn python(&self) -> PyBox<PyObj> {
        self.take().downgrade()
    }
}

impl<'a, T: ToPython> ToPython for &'a T {
    #[inline(always)]
    fn python(&self) -> PyBox<PyObj> {
        (*self).python()
    }
}
