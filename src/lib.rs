use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
pub mod my_maturin_project {
    use super::*;
    use numpy::PyReadonlyArrayDyn;

    pub struct RustyStruct {
        foo: Option<f64>,
    }

    impl RustyStruct {
        pub fn new(init_val: f64) -> Self {
            Self {
                foo: Some(init_val),
            }
        }
        pub fn rusty_sum(&mut self, y: f64) {
            match &mut self.foo {
                Some(x) => *x += y,
                None => (),
            }
        }
    }

    #[pyclass]
    pub struct PythonicClass {
        pub rusty_object: RustyStruct,
    }

    #[pymethods]
    impl PythonicClass {
        #[new]
        fn new(init_val: f64) -> Self {
            Self {
                rusty_object: RustyStruct::new(init_val),
            }
        }

        fn numpy_array_add_one(&mut self, array: PyReadonlyArrayDyn<f64>) -> PyResult<Option<f64>> {
            array
                .as_slice()?
                .iter()
                .for_each(|y| self.rusty_object.rusty_sum(*y));
            Ok(self.rusty_object.foo)
        }
    }
}
