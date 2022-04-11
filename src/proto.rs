
const DATA: &str = "data";

pub struct Protocol {
    pub data: String,
}

impl Protocol {
    pub fn new(content: &str) -> Self {
        let data = build_data_json(content);

        Protocol {
            data,
        }
    }

    pub fn as_buf(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.data.as_bytes());
        buf
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        match String::from_utf8(buf) {
            Ok(data) => Some(Protocol { data }),
            Err(_) => None,
        }
    }

    pub fn as_json(&self) -> String {
        match parse_data_json(&self.data) {
            Some(d) => d,
            None => todo!(),
        }
    }

    pub fn from_json(json: &str) -> Option<Self> {
        let data = build_data_json(json);
        Some(Protocol { data })
    }

    pub fn get_data_value(&self) -> Option<String> {
        match parse_data_json(&self.data) {
            Some(d) => Some(d),
            None => None,
        }
    }
}

fn build_data_json(string: &str) -> String {
    let mut json = json::JsonValue::new_object();
    json[DATA] = json::JsonValue::from(string);
    json.dump()
}

fn parse_data_json(data: &str) -> Option<String> {
    match json::parse(data) {
        Ok(json) => {
            match json[DATA].as_str() {
                Some(data) => Some(data.to_string()),
                None => None,
            }
        }
        Err(_) => None,
    }
}


