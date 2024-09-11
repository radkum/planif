use anyhow::Result;
use std::rc::Rc;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

/// Represents a COM runtime required for building schedules tasks
#[allow(dead_code)]
#[derive(Clone)]
pub struct ComRuntime(Rc<Com>);

impl ComRuntime {
    /// Creates a COM runtime for use with one or more
    /// [`ScheduleBuilder`](super::schedule_builder::ScheduleBuilder)
    pub fn new() -> Result<Self> {
        Ok(ComRuntime(Rc::new(Com::initialize()?)))
    }
}

struct Com {
    com_already_initialized: bool,
}

impl Com {
    fn initialize() -> Result<Self> {
        let res = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };

        let com_initialized = if let Err(_code) = res {
            //log::info!("COM is already initialized: {code}");
            false
        } else {
            true
        };

        let com_already_initialized = !com_initialized;

        Ok(Com {
            com_already_initialized,
        })
    }
    fn com_already_initialized(&self) -> bool {
        self.com_already_initialized
    }
}

impl Drop for Com {
    fn drop(&mut self) {
        if !self.com_already_initialized() {
            unsafe {
                CoUninitialize();
            }
        }
    }
}
