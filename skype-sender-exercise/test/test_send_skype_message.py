"""
Test script to verify
 - Posting of message to 'Test Qxf2Bot' channel on Skype
"""
from src import send_skype_message as ssm

def test_post_skype_message(base_obj):
    """
    Verify if message gets posted correctly onto the Test Qxf2Bot channel.
    """

    try:
        test_obj = base_obj
        result_flag = False

        # 1. Verify successful response on sending message to Skype
        if ssm.post_skype_message()['msg'] == 'Posted message':
            result_flag = True
        test_obj.log_result(result_flag,\
            success_msg='Message posted successfully onto Test Qxf2Bot channel',\
            failure_msg='Message not posted on Skype channel')

        test_obj.verify_all_checks_passed()

    except Exception as error:
        test_obj.log_result(False, failure_msg='Unable to run test due to {}'.format(str(error)))


