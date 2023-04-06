use fastly::ObjectStore;
use fastly::{Error, Request, Response};

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
        "/second" => {
            let entry = store.lookup("second")?;

            match entry {
                // Stream the value back to the user-agent.
                Some(entry) => Ok(Response::from_body(entry)),
                None => Ok(Response::from_body("Not Found").with_status(404)),
            }
        }
        _ => {
            //retrieve the file from object store with the request path
            let path_string = String::from(req.get_path());
            let entry = store.lookup(&path_string)?;

            match entry {
                // Stream the value back to the user-agent.
                Some(entry) => Ok(Response::from_body(entry)),
                None => Ok(Response::from_body("Not Found").with_status(404)),
            }
        }
    }

}
