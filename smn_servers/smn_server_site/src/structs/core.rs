pub struct Request {
    pub raw: String,
    pub path: String,
    pub method: String,
}

impl Request {
    pub fn new(raw: String) -> Self {
        let lines: Vec<&str> = raw.lines().collect();

        let request_line = lines.get(0).unwrap_or(&"");
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        
        let method = parts.get(0).unwrap_or(&"").to_string();
        let path = parts.get(1).unwrap_or(&"").to_string();

        Request {
            raw,
            path,
            method,
        }
    }
}

pub struct Response {
    pub body: String,
}