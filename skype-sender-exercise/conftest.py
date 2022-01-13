"""
pytest configuration file
"""
import pytest
from src import common_functions as cf
from src import send_skype_message as ssm

@pytest.fixture
def base_obj():
    "Create a Base object"
    base_obj = None
    try:
        base_obj = cf.Base()
        return base_obj
    except Exception as error:
        print("Unable to create an instance of Base classs due to - {}".format(str(error)))
        raise error

@pytest.fixture
def skype_obj():
    "Create a Skype object"
    try:
        skype_obj = ssm.Skype()
        return skype_obj
    except Exception as error:
        print("Unable to create an instance of Skype class due to - {}".format(str(error)))
        raise error
