pub mod calendar_event;
pub mod user;
pub mod calendar;

fn to_json<T>(value: T) -> String
where
    T: serde::Serialize,
{
    match serde_json::to_string(&value) {
        Ok(json) => json,
        Err(e) => format!("{}:#?", e),
    }
}
