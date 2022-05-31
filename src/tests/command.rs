#[cfg(test)]
mod command {
    use tempfile;

    #[test]
    fn test_init() -> Result<(), std::io::Error> {
        let _temp_dir = tempfile::tempdir()?;

        Ok(())
    }
}
