"Post message to Skype TestBot through API Endpoint"
import requests
import json
from conf import channel
from conf import API_KEY

def post_skype_message():
    "post mesage to test bot"
    msg = "Hi, This is a message from Nilaya"
    headers = {
        'Content-Type': 'application/json',
    }
    data = {"msg":msg, "channel":channel, "API_KEY":API_KEY}
    data_json = json.dumps(data)
    response = requests.post('https://skype-sender.qxf2.com/send-message', headers=headers, data=data_json)
    if(response.status_code==200):
        print("Hey!!message posted to skype successfully")
    else:
        print("unable to post message")

if __name__=='__main__':
    post_skype_message()