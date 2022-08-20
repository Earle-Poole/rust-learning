use file_handlers;

use crate::structs::Count;

const COUNT_PATHNAME: &str = "src/count.json";

pub fn func() -> Result<Count, serde_json::Error> {
    let file = file_handlers::read_from(COUNT_PATHNAME).unwrap_or("".to_string());
    let file_json = serde_json::from_str(&file);

    if file_json.is_err() {
        Err(file_json.unwrap_err())
    } else {
        let mut file_json: Count = file_json.unwrap();

        file_json.count = file_json.count + 1;

        let _new_file =
            file_handlers::write_to(COUNT_PATHNAME, serde_json::to_string(&file_json).unwrap());
        Ok(file_json)
    }
}
