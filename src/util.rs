//! lighttpd model — web server.

#[derive(Debug, Clone)]
pub struct Site {
    pub name: String,
    pub domain: String,
    pub port: u16,
    pub root: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub sites: Vec<Site>,
    pub running: bool,
}

impl Default for Model {
    fn default() -> Self { Self { sites: vec![], running: false } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_site(&mut self, name: &str, domain: &str, port: u16, root: &str) -> usize {
        let id = self.sites.len();
        self.sites.push(Site { name: name.into(), domain: domain.into(), port, root: root.into(), enabled: false });
        id
    }

    pub fn enable(&mut self, name: &str) -> bool {
        if let Some(s) = self.sites.iter_mut().find(|s| s.name == name) {
            s.enabled = true;
            true
        } else { false }
    }

    pub fn disable(&mut self, name: &str) -> bool {
        if let Some(s) = self.sites.iter_mut().find(|s| s.name == name) {
            s.enabled = false;
            true
        } else { false }
    }

    pub fn start(&mut self) { self.running = true; }
    pub fn stop(&mut self) { self.running = false; }

    pub fn list(&self) -> &[Site] { &self.sites }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn site_lifecycle() {
        let mut m = Model::new();
        m.add_site("blog", "blog.example.com", 80, "/var/www/blog");
        assert!(m.enable("blog"));
        m.start();
        assert!(m.running);
        assert!(m.disable("blog"));
        m.stop();
    }
}
