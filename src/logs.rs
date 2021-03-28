use chrono::prelude::*;


/// This struct should be pretty much always mutable
pub struct Logs {
    data: Vec<String>,    // All the logs (new element = new line)
    updated: bool,        // Determine if Logs has been updated since last read
    pub capacity: usize,  // Capacity can be changed without any issues
}

impl Logs {
    #![allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }

    fn prepare_msg(msg: &str) -> String {
        return if !msg.is_empty() {
            let x = msg.replace("\n", "");
            let now = Local::now();

            let hour = if now.hour() < 10 {
                format!("0{}", now.hour())
            } else {
                now.hour().to_string()
            };

            let minute = if now.minute() < 10 {
                format!("0{}", now.minute())
            } else {
                now.minute().to_string()
            };

            format!(" [{}:{}] {}\n", hour, minute, x)
        } else {
            // Add a new line if the msg is empty
            String::from("\n")
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.updated = true;
    }

    pub fn add(&mut self, msg: &str) {
        if self.data.len() >= self.capacity {
            self.clear();
        }

        self.data.push(Self::prepare_msg(msg));
        self.updated = true;
    }

    pub fn to_string(&mut self) -> String {
        self.updated = false;

        let mut logs_string = String::new();
        for msg in self.data.iter() {
            logs_string.push_str(msg.as_str());
        }

        logs_string
    }

    #[allow(dead_code)]
    pub fn is_updated(&self) -> bool {
        self.updated
    }
}

impl Default for Logs {
    fn default() -> Self {
        Self {
            data: vec![],
            updated: true,
            capacity: 50,
        }
    }
}
