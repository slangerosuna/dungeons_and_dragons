/* 
 * @Author: Sofia Langer-Osuna
 */
use bevy::prelude::*;
use std::any::Any;
use std::sync::{
    Mutex, 
    mpsc::{
        channel,
        Receiver,
        Sender,
    } 
};
use std::thread::*;
use pyo3::{
    prelude::*,
    types::PyTuple,
};
pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Resource)]
pub struct ScriptingResource {
    fn_registrator: Mutex<Sender<Box<dyn Any + Send + 'static>>>,
    fn_registrator_recv: Mutex<Receiver<Box<dyn Any + Send + 'static>>>,
}

impl ScriptingResource {
    pub fn new() -> (Self, PythonManager) {
        let (rx1, tx1) = channel();
        let (rx2, tx2) = channel();
        (Self {
            fn_registrator: Mutex::new(rx1),
            fn_registrator_recv: Mutex::new(tx2),
        }, PythonManager {
            fn_registrator_recv: tx1,
            fn_registrator: rx2,
        })
    }
    pub fn register_func<TSend: IntoPy<Py<PyTuple>> + Send + 'static,
                         TRecv: IntoPy<Py<PyTuple>> + Send + 'static>(
        &self,
        fn_info: Box<PyFnInfo<TSend, TRecv>>,
    ) -> Box<PyFn<TSend, TRecv>> {
        self.fn_registrator.lock().unwrap().send(fn_info);
        self.fn_registrator_recv.lock().unwrap().recv().unwrap().downcast::<PyFn<TSend, TRecv>>().unwrap()
    }
}

pub struct PythonManager {
    fn_registrator: Sender<Box<dyn Any + Send>>,
    fn_registrator_recv: Receiver<Box<dyn Any + Send>>,
}

impl PythonManager {
    pub fn run(&self) {
        //TODO python managing thread
        panic!("Not Implemented yet"); //Prevents empty infinite loop
        loop {
            //TODO find when to break
        }
    }
}

pub struct PyFnInfo<SendType, ReceiveType>
    where SendType: IntoPy<Py<PyTuple>> + ?Sized,
          ReceiveType: IntoPy<Py<PyTuple>> + ?Sized,
{
    pub send_type: std::marker::PhantomData<SendType>,
    pub receive_type: std::marker::PhantomData<ReceiveType>,
}

impl<SendType: IntoPy<Py<PyTuple>> + ?Sized, ReceiveType: IntoPy<Py<PyTuple>> + ?Sized> 
    PyFnInfo<SendType, ReceiveType> {
    pub fn create_py_fn(&self) -> 
        (PyFn<SendType, ReceiveType>,
         (Receiver<SendType>, Sender<PyResult<ReceiveType>>)) 
    {
        let (tx1, rx1) = channel();
        let (tx2, rx2) = channel();
        (PyFn {
            caller: tx1,
            caller_recv: rx2,
        }, (rx1, tx2))
    }

    pub fn new<TSend: IntoPy<Py<PyTuple>>,
               TRecv: IntoPy<Py<PyTuple>>>() -> Self {
        Self { 
            send_type: std::marker::PhantomData,
            receive_type: std::marker::PhantomData,
        }
    }
}

pub struct PyFn<SendType, ReceiveType> 
    where SendType: IntoPy<Py<PyTuple>> + ?Sized,
          ReceiveType: IntoPy<Py<PyTuple>> + ?Sized,
{
    pub caller: Sender<SendType>,
    pub caller_recv: Receiver<PyResult<ReceiveType>>,
}

impl<SendType: IntoPy<Py<PyTuple>> + ?Sized, ReceiveType: IntoPy<Py<PyTuple>> + ?Sized> 
    PyFn<SendType, ReceiveType> {
    fn call (
        &mut self,
        to_send: SendType,
    ) -> PyResult<ReceiveType> {
        self.caller.send(to_send).map_err(|_| pyo3::PyErr::from(pyo3::exceptions::PyRuntimeError::new_err("Failed to send to python thread")))?;
        self.caller_recv.recv().unwrap() //Will panic on error
    }
}

//TODO add TODOS

/*
 * allow the ai and user to write python scripts to control the game in a homebrew like way
 *    - have system oriented around callbacks for things like spell casting, reactions, etc.
 */
