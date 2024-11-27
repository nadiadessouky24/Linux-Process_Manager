use sysinfo::{System, Process, SystemExt, ProcessExt};

/// Function to filter processes based on given criteria
pub fn filter_processes<'a>(
    sysinfo: &'a System,
    filtered_pid: Option<i32>,
    filtered_procname: Option<&'a str>,
    min_cpu_usage: Option<f32>,
) -> Vec<(i32, &'a Process)> {
    let mut processes: Vec<(i32, &'a Process)> = sysinfo
        .processes()
        .iter()
        .filter(|(&pid, proc)| {
            // Include processes that satisfy any one of the criteria
            filtered_pid.map_or(false, |p| p == pid)
                || filtered_procname.map_or(false, |name| proc.name().contains(name))
                || min_cpu_usage.map_or(false, |cpu| proc.cpu_usage() >= cpu)
        })
        .map(|(&pid, proc)| (pid, proc))
        .collect();

    // Prioritize processes that satisfy all criteria
    let all_criteria = processes
        .iter()
        .filter(|(pid, proc)| {
            (filtered_pid.map_or(true, |p| *pid == p))
                && (filtered_procname.map_or(true, |name| proc.name().contains(name)))
                && (min_cpu_usage.map_or(true, |cpu| proc.cpu_usage() >= cpu))
        })
        .cloned()
        .collect::<Vec<_>>();

    // Remove duplicates by combining the prioritized list with the rest
    processes.retain(|(pid, proc)| {
        !all_criteria.iter().any(|(ap, _)| *ap == *pid)
    });

    [all_criteria, processes].concat()
}