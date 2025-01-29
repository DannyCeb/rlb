pub struct LoadBalancerConfig {
    servers: Vec<Server>,
}

impl LoadBalancerConfig {
    pub fn new() -> LoadBalancerConfig {
        LoadBalancerConfig {
            servers: Vec::new(),
        }
    }

    fn add_server(&mut self, server: Server) {
        self.servers.push(server);
    }

    fn get_servers(&self) -> &Vec<Server> {
        &self.servers
    }

    pub fn read_file(&mut self, file_name: &str) {
        let file = std::fs::read_to_string(file_name).expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&file).expect("Unable to parse json");
        let servers = json["servers"].as_array().expect("Unable to get servers");
        for server in servers {
            let name = server["name"].as_str().expect("Unable to get name");
            let address = server["uri"].as_str().expect("Unable to get address");
            let is_default = server["is_default"]
                .as_bool()
                .expect("Unable to get is_default");
            self.add_server(Server {
                name: name.to_string(),
                address: address.to_string(),
                is_default: is_default,
            });
        }
    }
}

pub struct Server {
    name: String,
    address: String,
    is_default: bool,
}
