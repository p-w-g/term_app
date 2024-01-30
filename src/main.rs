mod prompts;
use prompts::{write_git_conf, write_npmrc};

fn main() {
    write_git_conf();
    write_npmrc()
}
