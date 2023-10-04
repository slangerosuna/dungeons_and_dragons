use rustpython_vm:: {
    pymodule,
    function::{
        ArgCallable,
    },
    builtins::{
        PyStrRef,
    },
};

//TODO make resource for casting spells

/*
 * This is the module that will be imported into the python scripts
 * It will contain all the Rust functions that the script can call
 */
#[pymodule]
pub mod py_library {
    //TODO all
}
