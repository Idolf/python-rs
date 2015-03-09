extern crate "python27-sys" as raw;
// use std::ops::Index;
use object::{PyVal, PyObj};
use std::mem::transmute;

pub enum PyList {}

impl PyVal for PyList {
    #[inline(always)]
    fn upgrade_from(obj : &PyObj) -> Option<&PyList> {
        unsafe {
            if raw::PyList_CheckExact(obj.to_ptr()) != 0 {
                Some(transmute(obj))
            } else {
                None
            }
        }
    }
}


impl PyList {
    #[inline(always)]
    pub fn len(&self) -> isize {
        unsafe {
            let self_raw = self.to_ptr() as *const raw::PyTupleObject;

            if (*self_raw).ob_size < 0 {
                panic!("Invalid tuple length")
            } else {
                (*self_raw).ob_size as isize
            }
        }
    }

    // pub fn get(&self, index: isize) -> PyBox<PyObj> {
    //     panic!("foo");
    // }
}

// impl Index<isize> for PyTuple {
//     type Output = PyObj;

//     #[inline(always)]
//     fn index(&self, &index: &isize) -> &PyObj {
//         if 0 <= index && index < self.len() {

//             self.as_slice()[index as usize]
//         } else if index < 0 && 0 <= self.len() + index {
//             self.as_slice()[(self.len() + index) as usize]
//         } else {
//             panic!("Invalid tuple index")
//         }
//     }
// }
