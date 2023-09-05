# CyborgAI IPFS Rust

This is the repository for the CyborgAI IPFS Rust project. It is a Rust-based implementation of IPFS (InterPlanetary File System) with an HTTP server that allows users to upload, download, and preview files stored on the IPFS network.

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Test local](#test-local)
- [API Endpoints](#api-endpoints)
  - [Upload](#upload)
  - [Download](#download)
  - [Preview](#preview)
- [Contribution](#contribution)
- [Support](#support)
- [License](#license)

## Introduction

CyborgAI IPFS Rust is a project that aims to provide a Rust-based implementation of IPFS, enabling users to interact with the IPFS network using HTTP endpoints. It allows you to upload files to IPFS, download files from IPFS, and even preview content stored on IPFS.

## Getting Started

To get started with CyborgAI IPFS Rust, follow these steps:

## 1. Clone the repository:
```sh
git clone https://github.com/cyborg-ai-git/cyborgai_ipfs-rust.git
```
# Deploying an IPFS node (Kubo) to fly.io
[https://fly.io](https://fly.io)
## 2. Create the fly app

```sh
fly launch --copy-config
```

Give the app a name when promoted. Do not deploy the app yet if asked.


## 3. Create fly volume

```sh
fly volumes create data --size 20
```

## 4. Deploy the app

```sh
fly deploy
```

To get the IPv4 of the container, run the following command:

```sh
fly ips list
```


## 6. Use fly proxy to interact with the Kubo RPC API

To open a proxy to the Kubo node, run the following command:

```sh
fly proxy 5001:5001
```

Run the ipfs CLI against the deployed Kubo node:

```sh
ipfs id --api /ip4/127.0.0.1/tcp/5001/
```

## (Optional) Scale the node's container memory

To increase the memory allocated (which by default is 256):

```sh
fly scale memory 512
```


## Test Local
```sh
# Navigate to the project directory
cd cyborgai_ipfs-rust

# Build the project
cargo build

# Run the HTTP server
cargo run
```
### Download Ipfs Desktop for local tests
[Ipfs Desktop ](https://docs.ipfs.tech/install/ipfs-desktop/)

## API Endpoints

### Upload
The upload API endpoint allows you to upload files to the IPFS network. You can send one or more files as a multipart request, and the API will return a content identifier (CID) for each uploaded file.

Example usage:

```sh
curl -F "file=@/path/to/file1.txt" -F "file=@/path/to/file2.jpg" http://your-server/upload
```

### Download
The download API endpoint enables you to download files from the IPFS network. You provide the CID (content identifier) of the file you want to retrieve, and the API will return the file's content.

Example usage:

```sh
curl http://your-server/download/QmXyZaBcDeF...
```

### Preview
The preview API endpoint allows you to preview content stored on IPFS. You provide the CID (content identifier) of the content you want to preview, along with the desired file extension, and the API will serve the content with the appropriate MIME type.

Example usage:

```sh
curl http://your-server/preview/QmXyZaBcDeF.../jpg
```

For a comprehensive guide on installing and using IPFS, including the IPFS Desktop application, you can refer to the IPFS Desktop Documentation.


## Contribution

We welcome contributions from the community to help improve and expand the capabilities of CyborgAI IPFS Rust. Contributions can take various forms, including:

Implementing new features
- Fixing bugs and issues
- Improving documentation
- Enhancing performance
- Providing feedback and suggestions
- Contributors play a vital role in making this project better for everyone. If you're interested in contributing, please follow these steps:

### 1. Fork the repository.
Create a new branch for your feature or bug fix:

```sh
git checkout -b feature/my-feature
```

### 2. Make your changes and commit them:
```sh
git commit -am "Add new feature"
```


### 3. Push your changes to your forked repository:
```sh
git push origin feature/my-feature
```

Open a pull request (PR) to the main repository, describing your changes and their purpose.
Our team will review your PR and collaborate with you to ensure it aligns with the project's goals and standards. Thank you for your contributions!

## Support

If you have questions, encounter issues, or need assistance with CyborgAI IPFS Rust, please feel free to open an issue in the GitHub repository: CyborgAI IPFS Rust Issues

Your feedback and inquiries are valuable to us, and we will do our best to address them promptly.

## License

This project is licensed under the Apache License Version 2.0.