This document contains notes on an exercise we did. It helped in training folks to:

a) produce better first versions of Python code

b) Understand how to code review files submitted


### Background

Our coding standards have taken a hit. I am noticing that people do a good job when contributing to our open source initiatives. But when they are asked to produce something from scratch outside of that context (e.g.: on a client or a quick script for us), we are producing extremely poor first drafts of code. Further, our code reviews have not kept with the rising standards expected from our open source code. 

To take a first stab at improving, we had a 1-hour exercise where we looked at some files we know are awful and had everyone give their comments on them. At the end of the discussion, we came up with useful standards that we can easily implement in our first versions of code.

### Exercise

Code review the following files:

1. qxf2/samples/clients/BigBasket/bigbasket_link_click.py

    > No reusability, no modularity
    > Test pattern not broken into stages - setup, test, teardown()
    > I don't want it does - no comment (docstring)
    > locators, data are hard coded
    > Too many unecessary comments
    > Dead code
    > Hard to read

2. qxf2/samples/templates/Utils/mongo/mongo_util.py

    > Doc string is not that useful
    > ? Comments with source are not needed
    > connect/connect2 - naming is bad
    > too many print

3. qxf2/samples/clients/Gmail

    > no grouping via directory structure
    > no requirements.txt
    > no naming convention followed for files
    > credentials file is part of git
    > useless files - zip, env, etc.

4. qxf2-page-object-model/page_objects/DriverFactory.py

    > doc string is not very clear
    > too many multiline print()
    > run_local, run_mobile doc strings are not correct
    > hardcoded ports in run_mobile
    > missing methods for other browsers
    > whitespace and new line inconsistency
    > filename is inconsistent with everything else
    > no modularization at all
    > methods are too long
    > method signature is bad
    > method interface is unintuitive

5. qxf2-page-object-model/utils/csv_compare.py

    > underscore in class name
    > variable names file1, file2 is not descriptive
    > line 22 is unnecessary complexity
    > method is too long

6. qxf2-page-object-model/conftest.py

    > whitespace
    > better docstrings
    > too many arguments to a method
    > rearrange imports to be more logical

7. Codacy: An example


### Tips

These were some of the tips we collected as part of the discussion:

* Add a `if __name__ == '__main__'` if you want to execute your module standalone

* Use consistent whitespacing (operators, empty lines, etc.)

* Use good doc strings

* Look for unused variables

* Don't ever use absolute filepaths

* Think hard about names for classes, files, methods, functions, variables

* Break down big methods

* Break down cyclomatic complexity 

* Don't duplicate code 

* Never (well, almost never) leave commented out code (learn to use git instead)

* Don't have duplicate files (learn to use git properly)

* Keep file names consistent

* Don't use long file names

* Make sure your code has a logical directory structures

* Keep line lengths below 80

* Remove trailing spaces

* Remove trailing lines at end of files

* Good read: https://realpython.com/python-pep8/

* Example Python module to check complexity: wily