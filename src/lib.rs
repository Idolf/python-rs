#![feature(unsafe_destructor, core)]
extern crate "python27-sys" as raw;
extern crate libc;

pub use object::*;
pub use tuple::*;
pub use list::*;
pub use none::*;
mod object;
mod tuple;
mod list;
mod none;

/* This is the user-implemented method */
fn foo(args: &PyTuple) -> (&PyTuple, &PyTuple, (Option<&PyObj>, &PyNone, ())) {
    (args, args, (args[0], none(), ()))
}

/* Boiler plate for creating the module.
   Should be put inside a macro at some point. */
unsafe extern "C" fn foo_raw(_slf: *mut raw::PyObject, args: *mut raw::PyObject) -> *mut raw::PyObject {
    /* This helper function is a weird hack to prevent the lifetime from exceeding
       he current function call. */
    fn foo_helper(args: &PyObj) -> PyBox<PyObj> {
        match args.upgrade::<PyTuple>() {
            Some(args) => {
                foo(args).python()
            },
            None => panic!("Could not upgrade args into a tuple")
        }
    }
    foo_helper(PyObj::from_ptr(args)).into_ptr()
}

#[repr(C)]
struct AltPyMethodDef {
    pub ml_name: *const libc::c_char,
    pub ml_meth: Option<raw::PyCFunction>,
    pub ml_flags: libc::c_int,
    pub ml_doc: *const libc::c_char,
}

unsafe impl Sync for AltPyMethodDef {}

const METHODS: [AltPyMethodDef ; 2] = [
    AltPyMethodDef {
        ml_name: &[b'f', b'o', b'o', 0] as *const u8 as *const i8,
        ml_meth: Some(foo_raw),
        ml_flags: 1,
        ml_doc: &[b'd', b'o', b'c', 0] as *const u8 as *const i8,
    },
    AltPyMethodDef {
        ml_name: 0 as *const i8,
        ml_meth: None,
        ml_flags: 0,
        ml_doc: 0 as *const i8,
    }
];

#[no_mangle]
pub unsafe extern "C" fn initfoo() {
    use std::mem::transmute;
    raw::Py_InitModule4(&[b'f', b'o', b'o', 0] as *const u8 as *const i8,
                       transmute(&METHODS),
                       &[b'l', b'o', b'l', 0] as *const u8 as *const i8,
                       0 as *mut raw::PyObject,
                       1013);
}
