use src2exam::cli::Args;

use clap::Parser;

fn main() {
    let args = Args::parse();
    let exam_tester = match args.exam_tester() {
        Ok(tester) => tester,
        Err(e) => {
            eprintln!("Error initializing exam tester: {}", e);
            return;
        }
    };

    exam_tester.print_info();

    exam_tester.copy_submissions();
    exam_tester.copy_tests();
    exam_tester.run_tests();
}
