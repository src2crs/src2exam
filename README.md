# Src2Exam

A library providing tools for running automated tests on tasks
in the context of programming courses and exams.

## Background

This library targets programming courses where tasks/exams are provided along with
unit tests that use a standard framework.
Such a framework can be built-in, like e.g. in Rust or Go,
or it might be something like Catch2 for C++ or JUnit fpr Java.

I am developing this for my own courses where I aim to use test frameworks
that are commonly used in actual development processes with the respective languages.

The goal for this library is to provide a test runner that invokes tests using
existing frameworks and evaluates the results.
Most test frameworks can provide their results in some standardized format,
e.g. JUnit or a custom JSON format.
The test runner developed here will read such test results and generate reports
that are suited for evaluating assignments for tasks/exams.

Note that the test criteria differ from what you would expect in a regular development
context: for example, a test that crashes or doesn't even compile is considered a
regular outcome in the context of an exam and should be incorporated in the reports.
Furthermore, test runs shouldn't stop on errors and the same tests may be required to run
multiple times on different sources.

## Goals/Requirements

* Create a test runner that runs tests using a standardized framework
  and creates test reports.
* Add the ability to generate tests from a template in order to run them multiple
  times on different students' solutions.

## What this library is and isn't

While it may be possible to use test frameworks directly to achieve the desired results,
it will most likely be complicated to use.
This is especially true if multiple programming languages are to be used, each with
a different test framework.
Therefore, it seems sensible to create a library where the logic for running
tests and evaluating the results in a way tailored to exams is baked in.
This library will then contain adapters for all the desired languages/test frameworks.

As such, this project is a runner for existing test frameworks,
a kind of build system for running tests.
It is not meant as a test framework of its own,
i.e. it doesn't aim to replace tools like Catch2, or the Go/Cargo `test` subcommands.

## Development

Development starts by creating a couple of examples, in this case for tasks with tests
in Go. The reports created by `go test` are parsed and written in a form that seems
suitable for further evaluation.
Support for other languages and more features will follow later.
