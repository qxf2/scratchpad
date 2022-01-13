"""
Script to
 - Post a predefined message to the Test channel of Qxf2's Skpe account
"""
import json
import os
import requests
import sys
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from conf import skype_conf as skc

def post_skype_message():
    "Posts a predefined message on the set skype channel"
    headers = {\
        'Content-Type': 'application/json'\
        }
    payload = {\
        "msg" : skc.MESSAGE,\
        "channel": os.environ['TEST_CHANNEL_ID'],\
        "API_KEY": os.environ['API_KEY']\
        }
    response = requests.post(url = skc.SKYPE_SENDER_ENDPOINT, json = payload, headers=headers)
    print(response)
    return response.json()


if __name__ == '__main__':
    post_skype_message()
