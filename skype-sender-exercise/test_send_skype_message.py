"""
Test to send skype message to specified channel
"""
import json
import requests
import skype_config as config

def test_send_skype_message():

    headers = {
        'Content-Type': 'application/json',
    }
    
    data = {}
    data["msg"] = "Hi, This is a message from Sravanti"
    data["channel"] = config.SKYPE_CHANNEL
    data["API_KEY"] = config.API_KEY

    response = requests.post(config.SKYPE_URL, headers=headers, data=json.dumps(data))

    print(response.status_code)