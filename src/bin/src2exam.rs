use src2exam::cli::Args;

use clap::Parser;

fn main() {
    let args = Args::parse();

    // TODO: Test reading from a config file.
    // TODO: Test writing to a config file.
    // TODO: Handle config file extension automatically?
    // TODO: Remove nano seconds from the config file.
    args.write_config_to_file();

    let exam_tester = args.exam_tester();

    exam_tester.copy_submissions();
    exam_tester.copy_tests();
    exam_tester.run_tests();
}
