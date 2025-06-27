# Configuration

## Application Config

- Based on CLI Arguments, maybe even identical.
- Provides "base config" like base dir, config file name, verbose, dry run, flag for writing config etc.
- Also allows to create an exam description.

## Exam Config/Description

- Contains settings for the actual exam: subdir names, timeouts, ...
- Can be created from CLI args as well as files
- Can also be exported to files
- Optional idea: Make this "layered":
  - I.e. everything is optional, configs can be applied on top of each other
    to assemble them e.g. from CLI arguments and files.
  - A base config containing defaults can either be created thorugh constructors or the
    `Default` trait, or defaults can be part of the config trait.

## Exam Tester Config

- Combines at least a working dir with an exam description.
- Provides absolute paths to the actual working directories.
- Can also validate the config, create directories, print a summary, ...
- Question: Is this a separate config/trait, or can the Application Config be used?
