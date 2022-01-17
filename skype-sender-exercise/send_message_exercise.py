"""
Exercise the /send-message endpoint of Skype sender
"""

import json
import requests
import skype_conf as cred


API_KEY = cred.API_KEY
BASE_URL = cred.BASE_URL
MESSAGE = cred.MESSAGE
CHANNEL_ID = cred.CHANNEL_ID


def test_send_message():
    "Send the message"
    
    headers = {'Content-Type': 'application/json'}

    
    if not cred.API_KEY:
        print('No API key')
        return False
    if not MESSAGE or not CHANNEL_ID:
        print("No Message and Channel ID specified")
        return False   
        
    post_data = {"msg":MESSAGE, "API_KEY":API_KEY, "channel":CHANNEL_ID}
    json_data = json.dumps(post_data)
    response = requests.post(BASE_URL, headers=headers , data=json_data)
    print(response)
    return True
   


 

#---START OF SCRIPT
if __name__ == '__main__':
    
    test_send_message()
    
