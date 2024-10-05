use std::fs::read_dir;

pub struct ProblemReader {
    problem_files: Vec<String>
}

impl ProblemReader {
    pub fn new(dir: &str) -> ProblemReader {
        return ProblemReader {
            problem_files: ProblemReader::scan_problem_files_in_dir(dir)
        };
    }

    pub fn get_problem_files(&self) -> &Vec<String> {
        return &self.problem_files;
    }

    fn scan_problem_files_in_dir(dir: &str) -> Vec<String>
    {
        let paths = read_dir(dir);
        let mut problem_files = Vec::new();
        if paths.is_ok() //Directory has files and permissions to read
        {   
            for path in paths.unwrap()
            {
                if path.is_err()
                {
                    continue;
                }
                let problem_path = path.unwrap().path();
                if problem_path.extension() == Some(std::ffi::OsStr::new("json"))
                {
                    problem_files.push(problem_path.to_str().unwrap().to_string());
                }
            }     
        }
        return problem_files;
    }
}
