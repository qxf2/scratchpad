"""
Post message to skype channel through Api Endpoint using API key.
"""

import json
import requests
import conf

def send_message():
    "post request to send a message"
    headers = {'Content-Type': 'application/json'}    
    body = {'API_KEY' : conf.API_KEY,
    'msg' : 'Hi, This message is from Mohan',
    'channel' : conf.CHANNEL }
    json_data = json.dumps(body)
    response = requests.post(conf.BASE_URL, headers = headers, data = json_data)
    print(response.status_code)

if __name__ == '__main__':
    send_message()