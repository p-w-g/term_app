mod prompts;
use prompts::{write_bashrc, write_git_conf, write_npmrc};

fn main() {
    write_git_conf();
    write_npmrc();
    write_bashrc();
}
