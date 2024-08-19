  
pub struct Config<'a> {
    addr: &'a str,
    port: u16,
    currencies: &'a [String],
}
    
impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config<'a>, String> {
        if args.len() < 3 {
            return Err(String::from("not enough arguments"));
        }

        let addr_port = &args[1];

        let addr_port = addr_port.trim_start_matches("wss://");
        let (addr, port_str) = match addr_port.split_once(':') {
            Some(pair) => (pair.0, pair.1),
            None => return Err(String::from("ip-address and port not found")),
        };

        let port: u16 = port_str.parse().map_err(|err|{
            format!("wrong port format: {}", err)
        })?;

        let currencies: &[String] = &args[2..];

        Ok(Config {
            addr,
            port,
            currencies,
        })
    }

    pub fn get_addr(&self) -> &str {
        &self.addr
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_currencies_collection(&self) -> Vec<String> {
        self.currencies.to_vec()
    }
}
    