use std::time::{self, Duration};

use reqwest::{
    Client,
    blocking,
    // header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};

const NUM_REQUESTS: i32 = 10;

fn sync_main() {
    let mut bad = false;
    let client = blocking::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    // let mut headers = HeaderMap::new();
    // headers.insert("header-name", HeaderValue::from_static("value"));
    // let client = blocking::Client::builder()
    //     .timeout(Duration::from_secs(1))
    //     .default_headers(headers)
    //     .build()
    //     .unwrap();
    let start_time = time::SystemTime::now();
    for _ in 0..NUM_REQUESTS {
        // let response = client
        //     .post("https://www.example.com")
        //     .body("request body")
        //     .send()
        //     .unwrap();
        let response = client.get("https://www.example.com").send().unwrap();
        if !response.status().is_success() {
            // code is not in range 200-299
            bad = true;
        } else {
            // let value = response.headers().get(CONTENT_TYPE);
            // let content_type = match value {
            //     Some(val) => val.to_str().unwrap(),
            //     None => "",
            // };
            // println!("{content_type}");
            // let body = response.text().unwrap();
            // println!("{body}")
        }
    }
    let end_time = time::SystemTime::now();
    let elapsed_time = end_time.duration_since(start_time).unwrap().as_secs_f64();
    if bad {
        println!("Had failed synchronous request");
    } else {
        println!(" Synchronous elapsed time for {NUM_REQUESTS} requests = {elapsed_time} seconds");
    }
}

#[tokio::main]
async fn main() {
    let mut futures = vec![];
    let mut async_bad = false;
    let client = Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    // let mut headers = HeaderMap::new();
    // headers.insert("header-name", HeaderValue::from_static("value"));
    // let client = Client::builder()
    //     .timeout(Duration::from_secs(1))
    //     .default_headers(headers)
    //     .build()
    //     .unwrap();
    let start_time = time::SystemTime::now();
    for _ in 0..NUM_REQUESTS {
        let local_client = client.clone();
        let future = tokio::spawn(async move {
            // let response = local_client
            //     .post("https://www.example.com")
            //     .body("request body")
            //     .send()
            //     .await
            //     .unwrap();
            let response = local_client
                .get("https://www.example.com")
                .send()
                .await
                .unwrap();
            if !response.status().is_success() {
                // code is not in range 200-299
                true
            } else {
                // let value = response.headers().get(CONTENT_TYPE);
                // let content_type = match value {
                //     Some(val) => val.to_str().unwrap(),
                //     None => "",
                // };
                // println!("{content_type}");
                // let body = response.text().await.unwrap();
                // println!("{body}");
                false
            }
        });
        futures.push(future);
    }
    for future in futures {
        if future.await.unwrap() {
            async_bad = true;
            break;
        }
    }
    let end_time = time::SystemTime::now();
    let elapsed_time = end_time.duration_since(start_time).unwrap().as_secs_f64();
    if async_bad {
        println!("Had failed asynchronous request");
    } else {
        println!("Asynchronous elapsed time for {NUM_REQUESTS} requests = {elapsed_time} seconds");
    }

    tokio::task::spawn_blocking(|| {
        sync_main();
    })
    .await
    .unwrap();
}
