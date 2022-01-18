# to send message to one of the qxf2 skype channel using endpoint

import json
import requests

import conf

BASE_URL = 'https://skype-sender.qxf2.com/send-message'


def test_send_msg(msg_to_post, channel_id = conf.channel_id):
    "Create a binary file"
    newHeaders = {'Content-type': 'application/json', 'Accept': 'text/plain'}

    data = {'API_KEY' : conf.A_SECRET_API_KEY_ARUN_GIVES_YOU,
    'msg' : msg_to_post,
    'channel' : channel_id}

    Response = requests.post(BASE_URL,data=json.dumps(data), headers=newHeaders)
    print(Response.status_code)

#---START OF SCRIPT---
if __name__ == '__main__':
    msg_to_post = 'Hi, This is a message from Raghava'
    test_send_msg(msg_to_post)

