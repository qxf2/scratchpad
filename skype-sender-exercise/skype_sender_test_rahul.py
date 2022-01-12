"""
Test for posting message to skype channel
"""

import requests
import json
import skype_credentials as creds

def test_post_message():
    "post message"
    
    headers = {'Content-Type': 'application/json',}
    data = {'msg':creds.msg, 'channel':creds.channel_id, 'API_KEY': creds.API_KEY}
    data_string = json.dumps(data)
    response = requests.post('https://skype-sender.qxf2.com/send-message', headers=headers, data=data_string)
    
    print(response.status_code)

#----START OF SCRIPT
if __name__=='__main__':
    test_post_message()