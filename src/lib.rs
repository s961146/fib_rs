use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn say_hello(){
    println!("hello from rust");
}

#[pymodule]
fn flitton_fib_rs(_py:Python<'_>, m:&PyModule)-> PyResult<()>{
    m.add_wrapped(wrap_pyfunction!(say_hello));
    Ok(())
}