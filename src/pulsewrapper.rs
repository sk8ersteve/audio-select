use pulse::callbacks::ListResult;
use pulse::context::introspect::{ServerInfo, SinkInfo, SourceInfo};
use pulse::context::Context;
use pulse::mainloop::standard::{IterateResult, Mainloop};
use pulse::operation::State;
use pulse::proplist::Proplist;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::vec::Vec;

pub struct PulseWrapper {
    mainloop: Rc<RefCell<Mainloop>>,
    context: Rc<RefCell<Context>>,
    connected: bool,
}

impl PulseWrapper {
    pub fn new() -> Self {
        let mut proplist = Proplist::new().unwrap();
        proplist
            .set_str(pulse::proplist::properties::APPLICATION_NAME, "AudioSelect")
            .unwrap();

        let mainloop = Rc::new(RefCell::new(Mainloop::new().unwrap()));
        let context = Rc::new(RefCell::new(
            Context::new_with_proplist(
                mainloop.borrow_mut().deref(),
                "AudioSelectContext",
                &proplist,
            )
            .unwrap(),
        ));

        Self {
            mainloop: mainloop.clone(),
            context: context.clone(),
            connected: false,
        }
    }

    pub fn connect(&mut self) {
        if self
            .context
            .borrow_mut()
            .connect(None, pulse::context::FlagSet::NOFLAGS, None)
            .is_err()
        {
            eprintln!("[PAInterface] Error while connecting context");
            // return Err(PAError::MainloopConnectError.into());
        }

        // wait for context to be ready
        loop {
            match self.mainloop.borrow_mut().iterate(false) {
                IterateResult::Quit(_) | IterateResult::Err(_) => {
                    eprintln!("Iterate state was not success, quitting...");
                    return;
                }
                IterateResult::Success(_) => {}
            }
            match self.context.borrow().get_state() {
                pulse::context::State::Ready => {
                    break;
                }
                pulse::context::State::Failed | pulse::context::State::Terminated => {
                    eprintln!("Context state failed/terminated, quitting...");
                    return;
                }
                _ => {}
            }
        }
        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        if self.connected {
            self.context.borrow_mut().disconnect();
            self.connected = false;
        }
    }

    pub fn get_sources(&self) -> Vec<(String, String)> {
        let result = Rc::new(RefCell::new(Vec::new()));
        let result2 = Rc::clone(&result);
        let op = self.context.borrow().introspect().get_source_info_list(
            move |x: ListResult<&SourceInfo>| {
                if let ListResult::Item(e) = x {
                    let name = String::from(e.name.as_ref().unwrap().deref());
                    let description = String::from(e.description.as_ref().unwrap().deref());
                    result2.borrow_mut().push((name, description));
                }
            },
        );
        while op.get_state() == State::Running {
            match self.mainloop.borrow_mut().iterate(true) {
                IterateResult::Quit(_) | IterateResult::Err(_) => {
                    eprintln!("Iterate state was not success, quitting...");
                }
                IterateResult::Success(_) => {}
            }
        }
        Rc::try_unwrap(result).unwrap().into_inner()
    }

    pub fn get_sinks(&self) -> Vec<(String, String)> {
        let result = Rc::new(RefCell::new(Vec::new()));
        let result2 = Rc::clone(&result);
        let op = self.context.borrow().introspect().get_sink_info_list(
            move |x: ListResult<&SinkInfo>| {
                if let ListResult::Item(e) = x {
                    let name = String::from(e.name.as_ref().unwrap().deref());
                    let description = String::from(e.description.as_ref().unwrap().deref());
                    result2.borrow_mut().push((name, description));
                }
            },
        );
        while op.get_state() == State::Running {
            match self.mainloop.borrow_mut().iterate(true) {
                IterateResult::Quit(_) | IterateResult::Err(_) => {
                    eprintln!("Iterate state was not success, quitting...");
                }
                IterateResult::Success(_) => {}
            }
        }
        Rc::try_unwrap(result).unwrap().into_inner()
    }

    pub fn get_defaults(&self) -> (String, String) {
        let source = Rc::new(RefCell::new(String::new()));
        let sink = Rc::new(RefCell::new(String::new()));
        let source_clone = Rc::clone(&source);
        let sink_clone = Rc::clone(&sink);
        let op = self
            .context
            .borrow()
            .introspect()
            .get_server_info(move |x: &ServerInfo| {
                let source_name = x.default_source_name.as_ref().unwrap().deref();
                let sink_name = x.default_sink_name.as_ref().unwrap().deref();
                *source_clone.borrow_mut() = String::from(source_name);
                *sink_clone.borrow_mut() = String::from(sink_name);
            });
        while op.get_state() == State::Running {
            match self.mainloop.borrow_mut().iterate(true) {
                IterateResult::Quit(_) | IterateResult::Err(_) => {
                    eprintln!("Iterate state was not success, quitting...");
                    // return result;
                }
                IterateResult::Success(_) => {}
            }
        }
        (
            Rc::try_unwrap(source).unwrap().into_inner(),
            Rc::try_unwrap(sink).unwrap().into_inner(),
        )
    }

    pub fn set_default_source(&mut self, name: &str) {
        let op = self.context.borrow_mut().set_default_source(name, |_| ());
        while op.get_state() == State::Running {
            match self.mainloop.borrow_mut().iterate(true) {
                IterateResult::Quit(_) | IterateResult::Err(_) => {
                    eprintln!("Iterate state was not success, quitting...");
                    // return result;
                }
                IterateResult::Success(_) => {}
            }
        }
    }

    pub fn set_default_sink(&mut self, name: &str) {
        let op = self.context.borrow_mut().set_default_sink(name, |_| ());
        while op.get_state() == State::Running {
            match self.mainloop.borrow_mut().iterate(true) {
                IterateResult::Quit(_) | IterateResult::Err(_) => {
                    eprintln!("Iterate state was not success, quitting...");
                    // return result;
                }
                IterateResult::Success(_) => {}
            }
        }
    }
}

impl Drop for PulseWrapper {
    fn drop(&mut self) {
        if self.connected {
            self.context.borrow_mut().disconnect();
        }
    }
}
