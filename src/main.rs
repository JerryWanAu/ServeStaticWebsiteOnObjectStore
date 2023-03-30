use fastly::ObjectStore;
use fastly::{Error, Request, Response};
use md5::{Md5, Digest};
use hex;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    
    // Construct an ObjectStore instance which is connected to the Object Store named `my_store`
    // 'my_store' is the object store you created on your account  
    let store =
        ObjectStore::open("my_store").map(|store| store.expect("ObjectStore exists"))?;
    
    return match req.get_path() {
        "/readme" => {
            let entry = store.lookup("readme")?;

            match entry {
                // Stream the value back to the user-agent.
                Some(entry) => Ok(Response::from_body(entry)),
                None => Ok(Response::from_body("Not Found").with_status(404)),
            }
        }
        _ => {
            //using md5 to hash the request url path
            let mut md5_hash = Md5::new();
            let path_string = String::from(req.get_path());
            println!("path {}", path_string);
            md5_hash.update(path_string);
            let finalized_md5hash =md5_hash.finalize();
            let hashed_path = hex::encode(finalized_md5hash);
            println!("path in md5 x {}", hashed_path);

            //retrieve the file from object store with the hashed url path
            let entry = store.lookup(hashed_path.as_str())?;

            match entry {
                // Stream the value back to the user-agent.
                Some(entry) => Ok(Response::from_body(entry)),
                None => Ok(Response::from_body("Not Found").with_status(404)),
            }
        }
    }

}
