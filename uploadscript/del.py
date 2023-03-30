import os
import requests
import json

# get Fastly Key from env variable and set on header, get storeid from env variable
FastlyKey = os.environ.get('PRIVATE_KEY')
headers = {"Fastly-Key": FastlyKey}
storeid = os.environ.get('objectstoreid')

# get all object keys in the store
keylist = []
geturl = 'https://api.fastly.com/resources/stores/object/' + storeid + '/keys'
response = requests.get(geturl, headers = headers)
jsonData = response.json()
keylist = jsonData['data']

# delete files with the object keys
for key in keylist:
    delurl= 'https://api.fastly.com/resources/stores/object/' + storeid + '/keys/' + str(key)
    requests.delete(delurl, headers = headers)
    print(key)
