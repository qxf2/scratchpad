"Post message on skype using sype sender api endpoint"
import json
import requests
import conf

#Get API key and channel id fromm conf file 
API_KEY = conf.API_KEY
send_channel = conf.channel_id

def send_message():
    "Post message on skype using skype send-message api endpoint"
    msg = "Hi, This is a message from Akkul"
    skype_data = {"msg":msg, "API_KEY":API_KEY, "channel":send_channel}
    json_data = json.dumps(skype_data)
    response = requests.post('https://skype-sender.qxf2.com/send-message', headers={'Content-Type': 'application/json'} , data=json_data)
    if(response.status_code == 200):
        print("Message posted successfully")
    else:
        print("Couldnt post the message")

if __name__=='__main__':
    send_message()