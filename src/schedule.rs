use windows::core::BSTR;
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL, VARIANT};
use windows::Win32::System::TaskScheduler::{
    IActionCollection, IRegistrationInfo, ITaskDefinition, ITaskFolder, ITaskService,
    ITaskSettings, ITrigger, ITriggerCollection, TaskScheduler, TASK_LOGON_INTERACTIVE_TOKEN,
};

use crate::com::ComRuntime;

#[derive(Debug, PartialEq)]
/// A schedule is created by a [schedule builder](crate::schedule_builder). Once created, the
/// Schedule can be registered with the Windows Task Scheduler.
pub struct Schedule {
    pub(crate) task_folder: ITaskFolder,
    pub(crate) actions: IActionCollection,
    pub(crate) force_start_boundary: bool,
    pub(crate) registration_info: IRegistrationInfo,
    pub(crate) settings: ITaskSettings,
    pub(crate) task_definition: ITaskDefinition,
    pub(crate) task_service: ITaskService,
    pub(crate) trigger: Option<ITrigger>,
    pub(crate) triggers: ITriggerCollection,
    //repetition: IRepetitionPattern,
}

impl Schedule {
    /// Registers the schedule. Flags can be set by using the [TaskCreationFlags](crate::enums::TaskCreationFlags) enum.
    pub fn register(self, task_name: &str, flags: i32) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            self.task_folder.RegisterTaskDefinition(
                &BSTR::from(task_name),
                &self.task_definition,
                flags,
                // TODO allow user to specify creds
                VARIANT::default(),
                VARIANT::default(),
                TASK_LOGON_INTERACTIVE_TOKEN,
                VARIANT::default(),
            )?;
        }

        Ok(())
    }
}

/// TaskScheduler represents the actions you can take for using the Windows Task Scheduler.
/// For example: Creating new schedules, fetching the COM, etc.
pub struct TaskScheduler {
    com: ComRuntime,
}

impl TaskScheduler {
    /// Create's a new TaskScheduler
    /// # Example
    /// ```
    /// use planif::schedule::TaskScheduler;
    ///
    /// let ts = TaskScheduler::new().unwrap();
    /// ```
    ///
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            com: ComRuntime::new()?,
        })
    }

    /// Returns the TaskScheduler's com runtime
    /// # Example
    /// ```
    /// use planif::schedule::TaskScheduler;
    ///
    /// let ts = TaskScheduler::new().unwrap();
    /// let com = ts.get_com();
    /// ```
    pub fn get_com(&self) -> ComRuntime {
        self.com.clone()
    }

    /// Removing task
    /// # Example
    /// ```
    /// use planif::schedule::TaskScheduler;
    ///
    /// let ts = TaskScheduler::delete("some_task_name");
    /// ```
    pub fn delete_task(task_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let _com = Self::new();
            let task_service: ITaskService = CoCreateInstance(&TaskScheduler, None, CLSCTX_ALL)?;
            task_service.Connect(
                VARIANT::default(),
                VARIANT::default(),
                VARIANT::default(),
                VARIANT::default(),
            )?;

            let task_folder: ITaskFolder = task_service.GetFolder(&BSTR::from("\\"))?;
            task_folder.DeleteTask(&BSTR::from(task_name), 0)?;

            Ok(())
        }
    }
}
