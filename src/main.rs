use std::process::Command;
use std::{time::Duration, thread::sleep};

/**
Objective: To activate the success flag after running the program in batch mode.

Function extracted by Ghidra:
------------------------------------------------------------
int main(int argc,char **argv) {
  size_t First_argument_length;

  /* Check if we have one argument */
  if (argc == 2) {
    First_argument_length = strlen(argv[1]);
        /* Check if the first argument is 10 char long */
        if (First_argument_length == 10) {
            /* Check if the fifth char is an '@' char */
            if (argv[1][4] == '@') {
                puts("Nice Job!!");
                printf("flag{%s}\n",argv[1]);
            } else {
                usage(*argv);
            }
        } else {
          usage(*argv);
        }
  } else {
    usage(*argv);
  }
  return 0;
}
------------------------------------------------------------
*/

fn run_rev50_remotly(){
    /* commands to open terminal and run bash */
    Command::new("gnome-terminal")
        .arg("--")
        .arg("bash")
        .arg("-c")
        .arg(
            "cd ~/Downloads/rev50 && \
             ./rev50_linux64-bit 1234@56789 > /tmp/rev50_output.txt; \
             exec bash"
        )
        .spawn()
        .expect("failed to open terminal");

    sleep(Duration::from_secs(2));

    /* transform the output in file.txt to read */
    let result = std::fs::read_to_string("/tmp/rev50_output.txt")
        .expect("failed to read output");

    println!("Resultado final:\n{}", result);
}

fn main() {
    run_rev50_remotly();
}
