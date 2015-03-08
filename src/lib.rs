extern crate "python27-sys" as py;
extern crate libc;

#[repr(C)]
struct AltPyMethodDef {
    pub ml_name: *const libc::c_char,
    pub ml_meth: Option<py::PyCFunction>,
    pub ml_flags: libc::c_int,
    pub ml_doc: *const libc::c_char,
}

unsafe impl Sync for AltPyMethodDef {}


unsafe extern "C" fn foo(_slf: *mut py::PyObject, args: *mut py::PyObject) -> *mut py::PyObject {
    println!("lol");
    py::Py_INCREF(args);
    args
}

const METHODS: [AltPyMethodDef ; 2] = [
    AltPyMethodDef {
        ml_name: &[0x41i8, 0x42, 0] as *const i8,
        ml_meth: Some(foo),
        ml_flags: 1,
        ml_doc: &[0x41i8, 0x42, 0] as *const i8,
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

    py::Py_InitModule4(&[102i8, 111, 111, 0] as *const i8,
                       transmute(&METHODS),
                       &[102i8, 111, 111, 0] as *const i8,
                       0 as *mut py::PyObject,
                       1013);
}
