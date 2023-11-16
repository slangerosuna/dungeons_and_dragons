use bevy::prelude::*;
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
    pub fn_registrator: Sender<Box<PyFnInfo<dyn IntoPy<Py<PyTuple>>, dyn IntoPy<Py<PyTuple>>>>>,
    pub fn_registrator_recv: Receiver<Box<PyFn<dyn IntoPy<Py<PyTuple>>, dyn IntoPy<Py<PyTuple>>>>>,
}

impl ScriptingResource {
    pub fn new() -> (Self, PythonManager) {
        let (rx1, tx1) = channel();
        let (rx2, tx2) = channel();
        (Self {
            fn_registrator: tx1,
            fn_registrator_recv: rx2,
        }, PythonManager {
            fn_registrator_recv: rx1,
            fn_registrator: tx2,
        })
    }
    pub fn register_func<TSend: IntoPy<Py<PyTuple>>,
                         TRecv: IntoPy<Py<PyTuple>>>(
        &self,
        fn_info: Box<PyFnInfo<TSend, TRecv>>,
    ) -> Box<PyFn<TSend, TRecv>> {
        self.fn_regisrator.send(fn_info);
        self.fn_registrator_recv.recv()
    }
}

pub struct PythonManager {
    pub fn_registrator_recv: Receiver<Box<PyFnInfo<dyn IntoPy<Py<PyTuple>>, dyn IntoPy<Py<PyTuple>>>>>,
    pub fn_registrator: Sender<Box<PyFn<dyn IntoPy<Py<PyTuple>>, dyn IntoPy<Py<PyTuple>>>>>,
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
    //temporary fix for not using generics
    pub send: Box<SendType>, //TODO remove
    pub recv: Box<ReceiveType>, //TODO remove
    //TODO
}

pub struct PyFn<SendType, ReceiveType> 
    where SendType: IntoPy<Py<PyTuple>> + ?Sized,
          ReceiveType: IntoPy<Py<PyTuple>> + ?Sized,
{
    pub caller: Sender<SendType>,
    pub caller_recv: Receiver<PyResult<ReceiveType>>,
}

impl<SendType: IntoPy<Py<PyTuple>>, ReceiveType: IntoPy<Py<PyTuple>>> 
    PyFn<SendType, ReceiveType> {
    fn call (
        &mut self,
        to_send: SendType,
    ) -> ReceiveType {
        self.caller.send(to_send);
        self.caller_recv.recv().unwrap() //Will panic on error
    }
}

//TODO add TODOS

/*
 * allow the ai and user to write python scripts to control the game in a homebrew like way
 *    - have system oriented around callbacks for things like spell casting, reactions, etc.
 */
