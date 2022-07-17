pub struct FromTo {
    pub from_ip: String,
    pub from_port: Option<String>,
    pub to_ip: String,
    pub to_port: Option<String>,
}
impl FromTo {
    pub fn new(from_ip: String, to_ip: String) -> Self {
        Self {
            from_ip,
            from_port: None,
            to_ip,
            to_port: None,
        }
    }
    pub fn set_ports(&mut self, from_port: String, to_port: String) {
        self.from_port = Some(from_port);
        self.to_port = Some(to_port);
    }
    pub fn get_from(&self) -> String {
        format!(
            "{}:{}",
            self.from_ip,
            if let Some(from_port) = &self.from_port {
                from_port
            } else {
                "?"
            }
        )
    }
    pub fn get_to(&self) -> String {
        format!(
            "{}:{}",
            self.to_ip,
            if let Some(to_port) = &self.to_port {
                to_port
            } else {
                "?"
            }
        )
    }
    pub fn get_from_to(&self) -> String {
        format!("{} -> {}", self.get_from(), self.get_to())
    }
}
