# Example: Go Exam

This is a fictitious example exam using the Go programming language.
It consists of several parts:

* tasks with solutions
* tests to be delivered with the assignments
* extra tests used for grading
* solutions handed in by students

The student submissions contain solutions of various quality:

* completely correct solutions
* compiling solutions where tests fail
* solutions that compile but crash
* solutions that compile but time out
* solutions that don't compile

The test driver library should be able to do the following:

* Discover all student submissions by scanning a directory.
* Copy the all submissions and all extra grading
  tests to a separate grading directory.
* Run the extra tests on all student submissions.
* Generate a report for each student/task that shows
  which submissions compile, crash, fail, or pass all tests.
* Create a summary report for each student.

In a first step, the student reports will be added as comments to the
end of the solution files. Additionally, a short comment block will be added
as a template for filling in the score and comments.

This should simplify further grading by providing all necessary information
in the files themselves. Additional remarks and the final score can be added
to the file and the final score. In a later step, additional reports can be
generated in different formats.

## Notes/Remarks

* Each task is a separate go package.
  This way, if one task doesn't compile, it doesn't affect the others.
* Only the extra tests are run, the tests that had been delivered with the
  assignments are ignored.
  * This is because the students could have modified their tests.
    (Not necessarily in a malicious way, they should be free to experiment.)
  * Thus, the extra tests should cover all aspects that are important for grading.
* The original tasks contain markers for the beginning and end of the solution.
  * These markers are used to create the assignment files from the tasks.
  * This is not relevant for the test driver, it is only included here
    for completeness and to provide a better overview of the whole process.
* The grading directory in this example is called `grading` (the default name)
  and is ignored in version control.
  * This is to allow local testing using tests/examples and not cluttering the
    repository with generated files.
  * In a production environment, the generated files might be added to
    version control, as they are worked on during grading.
