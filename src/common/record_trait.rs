pub trait RecordTrait {
    fn create(&self, args: &[&str]);
    fn delete(&self, args: &[&str]);
    fn list(&self);
    fn update(&self, args: &[&str]);
}

pub trait RecordDatabaseTrait {
    fn db_create(&self, arg: &str) -> Result<usize, Box<dyn std::error::Error>>;
    fn db_delete(&self, arg: &str) -> Result<usize, Box<dyn std::error::Error>>;
    fn db_list(&self) -> Result<Vec<(u64, String)>, Box<dyn std::error::Error>>;
    fn db_update(&self, args: &[&str]) -> Result<usize, Box<dyn std::error::Error>>;
}
