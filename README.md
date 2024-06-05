# Rust_tally_vote

### How to run

In order to be able to build the binary you should run the command `cargo build --release`. Then, in order to run the exe, the following command should be executed `./target/release/tally_votes`. 

If you want to run the project without creating the .exe file, you should run `cargo build` (if not executed previously as release), run `chmod +x run.sh` in order to give permissions to the bash file, and then `run.sh` the bash file.This will run the tests, and if successfull, will execute the real application.

### Setup

This project is divided in 3 main folders. 
    - `src`: Which contains the main logic of the application. It has 3 scripts, `lib`, `models`, that will contain the models, and `main` with the main logic of the project.
    - `test_files`: Which contains the test `json` and `jsonl`files. These can be changed to vary the test content and thus, the results.
    - `tests`: This folder contanis the tests that will showcase if the app works properly.

### Improvements
In order to improve the project I'd dockerize the solution, so to assure that the libraries and depedencies are the same in any environment, as the container will contain the characteristics of the correct environment for its execution. Also, I'd convert the solution into an API, so it can be call as a request and its parameters could be modified in test time. I'm unsure how to do it in Rust (I know how in Python and C++) but I believe it wouldn't be a big issue. Last but not least, I'll add more error messages so debugging and error handling is easier.