use regex::Regex;

use std::{
    fs,
    time::SystemTime,
    collections::HashSet,
    
    io::{
        self, 
        Write,
    },
};

use crate::{
    constants::regexp::RegExp, 
    ui::report_alerts::ReportAlerts, 
    plugins::reports_pdf::ReportsPdfs, 
    
    utils::{
        file::FileUtils, 
        generate::Generate,
    }
};

pub struct ReportsHandlers {
    path: String,
    interval: usize,
    counter: usize,
    yes: Option<bool>,
}

impl ReportsHandlers {

    pub fn new(path: &str, interval: usize, counter: usize, yes: Option<bool>) -> Self {
        Self {
            path: path.to_string(),
            interval,
            counter,
            yes
        }
    }

    fn make_question(&self) {
        let file = Generate.random_string(16) + ".pdf";

        if !self.yes.unwrap_or(false) {
            ReportAlerts::make_question();
            let mut answer = String::new();

            io::stdout().flush().expect("Error flushing buffer");
            io::stdin().read_line(&mut answer).expect("Error reading input");

            if answer.to_lowercase().trim() == "y" || answer.to_lowercase().trim() == "yes" {
                let _ = ReportsPdfs::new(
                    &file, &self.path, self.interval,  self.counter, self.yes
                ).dump();
            }
        } else {
            let _ = ReportsPdfs::new(
                &file, &self.path, self.interval,  self.counter, self.yes
            ).dump();
        }
    }

    pub fn extract_table_names(&self, sql_file_path: &str) -> Option<HashSet<String>> {
        let sql_content = fs::read_to_string(sql_file_path).ok()?;
        let re = Regex::new(RegExp::CREATE_TABLE_INSERTS).ok()?;
        
        let tables: HashSet<String> = re.captures_iter(&sql_content)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect();
        
        if tables.is_empty() {
            None
        } else {
            Some(tables)
        }
    }

    pub fn get_most_recent_sql_file(&self, dump_file_path: &str) -> Option<(String, String)> {
        fs::read_dir(&dump_file_path)
            .ok()?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map(|ext| ext == "sql").unwrap_or(false))
            .max_by_key(|entry| entry.metadata().ok().and_then(|meta| meta.modified().ok()).unwrap_or(SystemTime::UNIX_EPOCH))
            .and_then(|entry| {
                let path = entry.path();
                let file_size = entry.metadata().ok()?.len();
                Some((path.display().to_string(), FileUtils::size(file_size)))
            })
    }

    pub fn report(&self) {
        if let Some((last_dump, size)) = self.get_most_recent_sql_file(&self.path) {
            ReportAlerts::report(&self.path, self.counter, &last_dump, &size, self.interval as usize);

            if let Some(tables) = &self.extract_table_names(&last_dump) {
                ReportAlerts::tables(tables);
            } else {
                ReportAlerts::no_tables();
            }

            self.make_question();
        }
    }

}
