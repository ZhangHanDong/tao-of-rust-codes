// extern crate csv_challenge;
#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use csv_challenge::{
        Opt,
        {load_csv, write_csv},
        replace_column,
    };
    #[test]
    fn test_csv_challenge(){
        test_load_csv();
        test_replace_column();
        test_write_csv();
    }
    fn test_load_csv(){
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());
    }
    fn test_replace_column(){
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename).unwrap();
        let modified_data = replace_column(csv_data, "City", "Beijing");
        assert!(modified_data.is_ok());
    }

    fn test_write_csv(){
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename).unwrap();
        let modified_data = replace_column(csv_data, "City", "Beijing").unwrap();
        let output_file = write_csv(&modified_data, "output/test.csv");
        assert!(output_file.is_ok());
    }

}
