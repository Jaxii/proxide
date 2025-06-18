// Todo implement
pub trait Reporter {
    fn report(&self, results: &[crate::scanner::ScanResult]);
}
