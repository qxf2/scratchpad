import requests
import os, json
from skype_config import channel, FirstName

def test_send_msg_testchannel():
    #send a message to skype channel "Test Qxf2Bot"
    API_KEY = os.environ.get('API_KEY')
    #FirstName = os.environ.get('FirstName')
    
    headers = {'Content-Type': 'application/json',}
	
    msg = "Hi, This is a message from %s" %(FirstName)    
    
    data = {"msg" : msg, "channel" : channel, "API_KEY" : API_KEY}

    data = json.dumps(data)
    response = requests.post('https://skype-sender.qxf2.com/send-message', headers=headers, data= data)
    
    print(response.status_code)

if __name__=='__main__':
    test_send_msg_testchannel()
