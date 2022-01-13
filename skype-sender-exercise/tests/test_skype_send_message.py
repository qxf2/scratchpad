"""
Test to send a message to skype channel using "/send-message" endpoint of Qxf2's Skype sender

"""
import json
import requests
import os, sys
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from conf import skype_conf as creds

SKYPE_ENDPOINT = 'https://skype-sender.qxf2.com/send-message'

def test_skype_send_message():   
    "Send the given message" 
    msg = "Hi, This is a message from Indira"
    data = json.dumps({"msg": msg, "channel": creds.SKYPE_CHANNEL_ID, "API_KEY": creds.API_KEY})    
    headers = {'Content-Type': 'application/json'}  
    response = requests.post(SKYPE_ENDPOINT, headers=headers, data=data)    
    print (response.status_code)

#---START OF SCRIPT
if __name__ == '__main__':    
    test_skype_send_message()