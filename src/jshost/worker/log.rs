use scorched::{log_this, LogData};

pub fn worker_log(data: LogData, name: Option<&String>) {
    match name {
        Some(name) => {
            log_this(LogData {
                message: format!("{}: {}", name, data.message),
                ..data
            });
        }
        None => {
            log_this(data);
        }
    }
}
