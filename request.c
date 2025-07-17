#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#include <curl/curl.h>

const int NUM_REQUESTS = 100;

size_t write_body_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t total_size = size * nmemb;
    char *response_ptr = (char *)userp;
    strncat(response_ptr, (char *)contents, total_size);
    return total_size;
}

size_t write_header_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t total_size = size * nmemb;
    char *header = (char *)contents;
    if (strstr(header, "Content-Type") != NULL || strstr(header, "content-type") != NULL) {
        char *content_type = (char *)userp;
        strncpy(content_type, header + 14, total_size - 14);
        content_type[total_size - 16] = '\0';
    }
    return total_size;
}

int main(void) {
    CURL *curl;
    // struct curl_slist *headers = NULL;
    CURLcode res;
    time_t start_time, end_time, elapsed_time;
    long response_code;
    char response_body[2048] = {0}, content_type[256] = {0};
    bool bad = false;

    curl = curl_easy_init();
    if (curl) {
        curl_easy_setopt(curl, CURLOPT_URL, "https://www.example.com");
        // curl_easy_setopt(curl, CURLOPT_POST, 1L);
        // curl_easy_setopt(curl, CURLOPT_POSTFIELDS, "request body");
        // headers = curl_slist_append(headers, "header-name: value");
        // curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_body_callback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, response_body);
        curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, write_header_callback);
        curl_easy_setopt(curl, CURLOPT_HEADERDATA, content_type);
        curl_easy_setopt(curl, CURLOPT_TIMEOUT, 1L);
        start_time = time(NULL);
        for (int i = 0; i < NUM_REQUESTS; i++) {
            memset(response_body, 0, sizeof(response_body));
            memset(content_type, 0, sizeof(content_type));
            res = curl_easy_perform(curl);
            if (res != CURLE_OK) {
                fprintf(stderr, "curl_easy_perform() failed: %s\n", curl_easy_strerror(res));
            } else {
                curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &response_code);
                if (response_code != 200) {
                    bad = true;
                } else {
                    // printf("%s\n", content_type);
                    // printf("%s\n", response_body);
                }
            }
        }
        end_time = time(NULL);
        elapsed_time = end_time - start_time;
        if (bad) {
            puts("Had failed synchronous request");
        } else {
            printf(" Synchronous elapsed time for %d requests = %ld seconds\n", NUM_REQUESTS, elapsed_time);
        }
        curl_easy_cleanup(curl);
    } else {
        fprintf(stderr, "Failed to initialize libcurl\n");
        return 1;
    }
    
    return 0;
}
