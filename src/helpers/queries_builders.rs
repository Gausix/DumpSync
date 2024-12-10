pub struct QueriesBuilders;

impl QueriesBuilders {    

    pub fn show_tables(&self) -> String {
        "SHOW TABLES".to_string()
    }

    pub fn show_create_table(&self, table: &str) -> String {
        format!("SHOW CREATE TABLE `{}`;", table)
    }

    pub fn drop_table(&self, table: &str) -> String {
        format!("DROP TABLE IF EXISTS `{}`;", table)
    }

    pub fn create_database(&self, dbname: &str) -> Result<(String, String), String> {
        let create_db = format!("CREATE DATABASE IF NOT EXISTS `{}`;\n", dbname);
        let use_db = format!("USE `{}`;", dbname);

        Ok((create_db, use_db))
    }

    pub fn insert_into(&self, table: &str, values: Vec<String>, ignore: bool) -> String {
        if ignore {
            format!("INSERT IGNORE INTO `{}` VALUES ({});", table, values.join(", "))
        } else {
            format!("INSERT INTO `{}` VALUES ({});", table, values.join(", "))
        }
    }

    pub fn select(&self, table: &str, offset: Option<usize>, limit: Option<usize>) -> String {
        let mut query = format!("SELECT * FROM `{}`", table);

        if let Some(l) = limit {
            query.push_str(&format!(" LIMIT {}", l));
        }

        if let Some(o) = offset {
            query.push_str(&format!(" OFFSET {}", o));
        }

        query
    }
    
}
