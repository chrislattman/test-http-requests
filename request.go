package main

import (
	// "bytes"
	"fmt"
	// "io"
	"net/http"
	"sync"
	"time"
)

const NUM_REQUESTS int = 100
var bad = false
var asyncBad = false

func main() {
	var wg sync.WaitGroup
	startTime := time.Now().UnixMilli()
	wg.Add(NUM_REQUESTS)
	for range [NUM_REQUESTS]int{} {
		go func() {
			defer wg.Done()
			// req, _ := http.NewRequest(http.MethodPost, "https://www.example.com", bytes.NewReader([]byte("request body")))
			req, _ := http.NewRequest(http.MethodGet, "https://www.example.com", nil)
			client := http.Client{
				Timeout: 1 * time.Second,
			}
			// req.Header.Set("header-name", "value")
			resp, err := client.Do(req)
			if err != nil {
				fmt.Println(err)
				return
			}
			// body, err := io.ReadAll(resp.Body)
			// if err != nil {
			// 	fmt.Println(err)
			// 	return
			// }
			// fmt.Println(string(body))
			// contentType := resp.Header.Get("Content-Type")
			// fmt.Println(contentType)
			if resp.StatusCode != 200 {
				asyncBad = true
			}
		}()
	}
	wg.Wait()
	endTime := time.Now().UnixMilli()
	elapsedTime := float64(endTime - startTime) / 1000.0
	if bad {
		fmt.Println("Had failed asynchronous request")
	} else {
		fmt.Printf("Asynchronous elapsed time for %v requests = %v seconds\n", NUM_REQUESTS, elapsedTime)
	}

	startTime = time.Now().UnixMilli()
	for range [NUM_REQUESTS]int{} {
		// req, _ := http.NewRequest(http.MethodPost, "https://www.example.com", bytes.NewReader([]byte("request body")))
		req, _ := http.NewRequest(http.MethodGet, "https://www.example.com", nil)
		client := http.Client{
			Timeout: 1 * time.Second,
		}
		// req.Header.Set("header-name", "value")
		resp, err := client.Do(req)
		if err != nil {
			fmt.Println(err)
			continue
		}
		// body, err := io.ReadAll(resp.Body)
		// if err != nil {
		// 	fmt.Println(err)
		// 	return
		// }
		// fmt.Println(string(body))
		// contentType := resp.Header.Get("Content-Type")
		// fmt.Println(contentType)
		if resp.StatusCode != 200 {
			bad = true
		}
	}
	endTime = time.Now().UnixMilli()
	elapsedTime = float64(endTime - startTime) / 1000.0
	if bad {
		fmt.Println("Had failed synchronous request")
	} else {
		fmt.Printf(" Synchronous elapsed time for %v requests = %v seconds\n", NUM_REQUESTS, elapsedTime)
	}
}
