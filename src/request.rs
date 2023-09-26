pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<Header>,
}
type Header = (String, String);
impl Request {
    pub fn parse(string: &str) -> Self {
        let mut request = Self {
            method: String::new(),
            path: String::new(),
            headers: Vec::new(),
        };
        let req_lines = string.split("\r\n").collect::<Vec<&str>>();
        let first_line = req_lines.first().unwrap();
        let first_line_parts = first_line.split_whitespace().collect::<Vec<&str>>();
        request.method = String::from(first_line_parts[0]);
        request.path = String::from(first_line_parts[1]);

        for (_, line) in req_lines[1..].iter().enumerate() {
            let header = match line.split_once(":") {
                Some(val) => (String::from(val.0), String::from(val.1.trim())),
                None => ("".into(), "".into()),
            };
            request.headers.push(header);
        }
        request
    }
    pub fn get_header(&self, name: &str) -> String {
        for header in self.headers.iter() {
            if header.0 == name.to_owned() {
                return header.1.clone();
            }
        }
        return String::new();
    }
}
