#[derive(Debug, Clone, Default)]
pub struct ProcessInfo {
    pub pid: String,
    pub user: String,
    pub cpu: String,
    pub mem: String,
    pub vsz: String,
    pub rss: String,
    pub tty: String,
    pub stat: String,
    pub start: String,
    pub time: String,
    pub command: String,
}

impl ProcessInfo {
    pub fn parse_ps_output(stdout: &[u8]) -> Vec<ProcessInfo> {
        let output = String::from_utf8_lossy(stdout);
        let mut processes = Vec::new();

        for (i, line) in output.lines().enumerate() {
            // Skip header line
            if i == 0 {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 11 {
                let process = ProcessInfo {
                    user: parts[0].to_string(),
                    pid: parts[1].to_string(),
                    cpu: parts[2].to_string(),
                    mem: parts[3].to_string(),
                    vsz: parts[4].to_string(),
                    rss: parts[5].to_string(),
                    tty: parts[6].to_string(),
                    stat: parts[7].to_string(),
                    start: parts[8].to_string(),
                    time: parts[9].to_string(),
                    command: parts[10..].join(" "),
                };
                processes.push(process);
            }
        }

        processes
    }
}
