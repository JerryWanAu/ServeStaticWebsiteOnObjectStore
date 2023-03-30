import os
import requests
import hashlib

# root path of the website on your local machine
path ="/Users/swan/Documents/ComputeProject/StaticSite"

#we shall store all the file names in this list
filelist = []
for root, dirs, files in os.walk(path):
	for file in files:
        #append the file name to the list
		filelist.append(os.path.join(root,file))

# get Fastly Key from env variable and set on header, get storeid from env variable
FastlyKey = os.environ.get('PRIVATE_KEY')
headers = {"Fastly-Key": FastlyKey}
storeid = os.environ.get('objectstoreid')

#upload all the files to the object store specified
for name in filelist:
    # hash relative path into objectkey
    uploadfiles = name
    name =name.replace(path, "")
    hash_obj = hashlib.md5(name.encode())
    objectkey = hash_obj.hexdigest()
    print(name, "->", objectkey)
    # upload file with the hashed objectkey
    myurl = 'https://api.fastly.com/resources/stores/object/' + storeid + '/keys/' + objectkey
    getdata = requests.put(myurl, data=open(uploadfiles, 'rb'), headers = headers)

