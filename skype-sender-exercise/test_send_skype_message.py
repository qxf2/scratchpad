"""
Send message to one of the qxf2 skype channel 
using skype-sender endpoint

"""
import json
import requests
import conf

BASE_URL = 'https://skype-sender.qxf2.com/send-message'

def test_send_msg(msg_to_post):
    "create the request with the passed message"
    Headers = {'Content-type': 'application/json', 
               'Accept': 'text/plain'}
    data = {'API_KEY':conf.API_KEY,
    'msg': msg_to_post,
    'channel': conf.channel}
    data=json.dumps(data)
    Response = requests.post(BASE_URL, data=data, headers=Headers)
    print(Response.status_code)

#---START OF SCRIPT---
if __name__ == '__main__':
    msg_to_post = 'Hi, This is a message from Rohini'
    test_send_msg(msg_to_post)