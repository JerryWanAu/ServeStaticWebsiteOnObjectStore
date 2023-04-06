# Serve Static Website from c@e with object store
This is to serve a static website from c@e with object store, no backend required. 

Object store hold object with an object key as the index, no folder structure. So when uploading static files to the object store, we use the comparative path&file name as the object key as the python script in /uploadscript shows. When serve from c@e with rust code, we use the request url path as the object key, c@e using this object key to get the related file from object store.

/uploadscript/upload.py is used to upload all files including files in subdirectory to object store using object store api.
/uploadscript/del.py is used to delete all objects in the object store.

https://strongly-credible-bass.edgecompute.app/index.html is a static website totally served from object store.

# Object Store Rust

Fastly Object Store is a globally consistent key-value storage accessible across the Fastly Network. This makes it possible for your Compute@Edge application to read and write from Object-stores. The application shows how to use Fastly Object Store within a Rust Compute@Edge application.
https://developer.fastly.com/reference/api/services/resources/object-store/
https://docs.rs/fastly/latest/fastly/object_store/index.html
