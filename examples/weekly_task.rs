use chrono::prelude::*;
use planif::enums::{DayOfWeek, TaskCreationFlags};
use planif::schedule::TaskScheduler;
use planif::schedule_builder::{Action, ScheduleBuilder};

fn main() -> anyhow::Result<()> {
    let ts = TaskScheduler::new()?;
    let com = ts.get_com();
    let sb = ScheduleBuilder::new(&com).unwrap();

    sb.create_weekly()
        .author("Matt")?
        .description("Test Weekly Trigger")?
        .trigger("test_weekly_trigger", true)?
        .action(Action::new("test", "notepad.exe", "", ""))?
        .start_boundary(&Local::now().to_rfc3339())?
        .days_of_week(vec![DayOfWeek::Sunday, DayOfWeek::Thursday])?
        .weeks_interval(3)?
        .build()?
        .register("WeeklyTaskName", TaskCreationFlags::CreateOrUpdate as i32)?;
    Ok(())
}
