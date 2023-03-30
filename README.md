# Serve Static Website from c@e with object store
This is to serve a static website from c@e with object store, no backend required. Object store hold object with an object key as the index, no folder structure. So when uploading static files to the object store, we hash the comparative path&file name into the object key as the python script in /uploadscript shows. When serve from c@e with rust code, we hash the request url path into the object key as well, c@e using this object key to get the related file from object store.

/uploadscript/upload.py is used to upload all files including file in subdirectory to object store using object store api.
/uploadscript/del.py is used to delete all objects in the object store.

https://strongly-credible-bass.edgecompute.app/index.html is a static website totally served from object store.
test
# Object Store Rust Template

Fastly Object Store is a globally consistent key-value storage accessible across the Fastly Network. This makes it possible for your Compute@Edge application to read and write from Object-stores. The application shows how to use Fastly Object Store within a Rust Compute@Edge application.

**For more details about other starter kits for Compute@Edge, see the [Fastly developer hub](https://developer.fastly.com/solutions/starters)**

## Getting Started

### Installing the Fastly CLI

The [Fastly CLI](https://developer.fastly.com/learning/tools/cli/) is available for multiple operating systems.

- **MacOS**: Install from Homebrew:

    ```
    brew install fastly/tap/fastly
    ```

- **Windows**: [Visit the GitHub repository](https://github.com/fastly/cli/releases/latest) to download the prebuilt binary for your architecture.
- **Linux**: Packages are available for many distributions, along with prebuilt binaries. [Visit the GitHub repository](https://github.com/fastly/cli/releases/latest) to download the package for your distribution.

### Initialise a new application

Create a new Fastly Compute@Edge project using this template:

```shell
fastly compute init --from https://github.com/JakeChampion/object-store-rust-template
```

### Run the application locally

To run the application locally we only need to run one command:

```shell
fastly compute serve
```

The application should now be running on <http://127.0.0.1:7676>

The `fastly.toml` file has a section named `[local_server]` which contains the configuration for Object Store when running locally.

## Understanding the code

### [./src/main.rs](./src/main.rs)

A single main function is defined which uses the [`#[fastly::main]` macro](https://docs.rs/fastly/latest/fastly/attr.main.html), this is the function that recieves the incoming `Request` and returns a `Result<Response>`, which will be sent to the user-agent.

## Deploying the project to Fastly

Note that Fastly Services have to have unique names within a Fastly Account.

To create and deploy to a new Fastly Service run the command and follow the instructions:

```shell
fastly compute publish
```

That is it, we now have a Fastly Service, a Fastly Object Store and have them linked together!

You can view real-time logs for the Fastly Service by running:
```shell
fastly log-tail
```

## Adding entries to the Object Store

It is possible to add key-value pairs to an Object Store using the Fastly CLI like so:
```shell
fastly objectstore insert --id=$FASTLY_OBJECT_STORE_ID --k=$KEY --v=$VALUE
```

For example, here is how you could add to the Object Store named `my-store` a key named `readme` whose value is the contents of `README.md`:
```shell
fastly objectstore get --id="$(fastly objectstore list | grep -A 3 'my-store' | tail -n 1 | grep -o '[a-z0-9]*')" --k="readme" --v="$(cat README.md)"
```

## Security issues

Please see our [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.