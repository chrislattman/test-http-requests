const NUM_REQUESTS = 100;
var asyncBad = false;

async function getResponse() {
    const response = await fetch("https://www.example.com", {
        // method: "POST",
        // body: "request body",
        // headers: {
        //     "header-name": "value"
        // },
        signal: AbortSignal.timeout(1000)
    });
    if (response.status !== 200) {
        asyncBad = true;
    } else {
        // const contentType = response.headers.get("Content-Type");
        // console.log(contentType);
        // const body = await response.text();
        // console.log(body);
    }
}

async function asyncMain() {
    const indices = [...Array(NUM_REQUESTS).keys()];
    const promises = indices.map(getResponse);
    await Promise.all(promises);
}

async function runner() {
    const startTime = Date.now();
    await asyncMain();
    const endTime = Date.now();
    const elapsedTime = (endTime - startTime) / 1000.0;
    if (asyncBad) {
        console.log("Had failed asynchronous request");
    } else {
        console.log(`Asynchronous elapsed time for ${NUM_REQUESTS} requests = ${elapsedTime} seconds`);
    }
}

runner();
