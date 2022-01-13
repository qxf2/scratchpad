"""
Script to contain all common methods to be used across the tests.
"""
import time
from loguru import logger


class Base():
    "Base class to contain common methods."

    def __init__(self):
        "Initialize the Base class"
        self.logger = logger
        self.pass_count = 0
        self.fail_count = 0
        self.check_count = 0

    def log_result(self, result_flag, success_msg=None, failure_msg=None):
        "Log results using loguru"
        try:
            self.check_count += 1
            def_msg = 'Check #%s ' % self.check_count

            if result_flag is True:
                self.pass_count += 1
                if success_msg is not None:
                    self.logger.success(def_msg+success_msg)
            else:
                self.fail_count += 1
                if failure_msg is not None:
                    self.logger.error(def_msg+failure_msg)
        except Exception as error:
            self.logger.error("Unable to log messages due to - {}".format(str(error)))

    def verify_all_checks_passed(self):
        "Verify if all checks have passed"
        if self.pass_count == self.check_count:
            self.logger.success("All checks have passed in this test run!")
        else:
            self.logger.error("Few checks in this test run have failed!")
