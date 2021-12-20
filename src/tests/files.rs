#[cfg(test)]
mod programs {
    use crate::{environment::Environment, error::Error, run_string};
    use std::{fs, process};

    fn run_file(file: String) -> Result<(), Error> {
        let code = fs::read_to_string(file).unwrap_or_else(|err| {
            eprintln!("Error reading file: {}", err);
            process::exit(1)
        });

        run_string(&code, &mut Environment::new(), false)?;

        Ok(())
    }

    #[test]
    fn programs() {
        let programs = fs::read_dir("test/programs").unwrap();

        for file in programs {
            let path = file.unwrap().path().display().to_string();

            run_file(path).unwrap();
        }
    }
}
