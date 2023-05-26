pub trait RecordTrait {
    fn create(&self, args: &[&str]);
    fn delete(&self, args: &[&str]);
    fn list(&self, args: &[&str]);
    fn read(&self, args: &[&str]);
    fn update(&self, args: &[&str]);
}
