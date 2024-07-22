# AWS Lambda Runtime for WP

## Introduction

Experimental runtime to run WP on AWS Lambda.

## Status

This is a proof of concept in early development. For the implementation there have been several attempts. One of such was using a customized Bref runtime (written in PHP). This was the first successful attempt on running WP with basic functionalities working. However, it had a very slow startup time which made it unusable for production in cold starts. 

After that I tried several approaches with a custom runtime written in Rust and built on top of AWS's [`lambda-http`](https://github.com/awslabs/aws-lambda-rust-runtime). The main idea was to stop using `php-fpm`, as in a function we don't need a process manager, and use some other PHP runtime.

I went through different ideas while looking for docs, such as using FastCGI without the process manager, then I thought of using LiteSpeed, but in the end, I didn't like the idea to have to start 2 different processes in the function.

Then I decided to go with the difficult but apparently optimal approach, which is to start the PHP's interpreter in the process of the runtime written in Rust. This is the most recent code, though it didn't go to far before I had to stop. It lives in `php-embed-sys`. I tried to write a safe abstraction in Rust over the C API, but it was too difficult, as I had to implement many unnecessary types and functions. Finally I realized that I need to write a wrapper in C to abstract the Zend Engine API, and then write the safe abstraction over the wrapper.

Note: Recently I learned that the wrapper in C is how WordPress Playground implemented this, so I plan on trying to build some POC using Playground on serverless functions on top of Node.js, and once I see how it performs, I'll probably come back to this project.