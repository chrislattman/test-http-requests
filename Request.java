import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.net.http.HttpClient.Redirect;
import java.net.http.HttpResponse.BodyHandlers;
import java.time.Duration;
import java.util.ArrayList;
import java.util.concurrent.CompletableFuture;

public class Request {
    static final int NUM_REQUESTS = 100;
    static boolean bad = false;
    static boolean asyncBad = false;

    public static void main(String[] args) throws IOException, InterruptedException {
        HttpClient client = HttpClient.newBuilder().followRedirects(Redirect.NORMAL).build();

        long startTime = System.currentTimeMillis();
        ArrayList<CompletableFuture<Void>> futures = new ArrayList<>(NUM_REQUESTS);
        for (int i = 0; i < NUM_REQUESTS; i++) {
            HttpRequest request = HttpRequest.newBuilder()
                .uri(URI.create("https://www.example.com"))
                // .setHeader("header name", "value")
                // .POST(HttpRequest.BodyPublishers.ofString("request body"))
                .timeout(Duration.ofSeconds(1))
                .build();
            CompletableFuture<Void> future = client.sendAsync(request, BodyHandlers.ofString())
                // .thenApply(response -> {
                //     String body = response.body();
                //     System.out.println(body);
                //     String contentType = response.headers().firstValue("Content-Type").orElse(null);
                //     System.out.println(contentType);
                //     return response;
                // })
                .thenApply(HttpResponse::statusCode).thenAccept(statusCode -> {
                    if (statusCode != 200) {
                        asyncBad = true;
                    }
                });
            futures.add(future);
        }
        CompletableFuture.allOf(futures.toArray(new CompletableFuture<?>[0])).join();
        long endTime = System.currentTimeMillis();
        double elapsedTime = (endTime - startTime) / 1000.0;
        if (asyncBad) {
            System.out.println("Had failed asynchronous request");
        } else {
            System.out.println("Asynchronous elapsed time for " + NUM_REQUESTS + " requests = " + elapsedTime + " seconds");
        }

        startTime = System.currentTimeMillis();
        for (int i = 0; i < NUM_REQUESTS; i++) {
            HttpRequest request = HttpRequest.newBuilder()
                .uri(URI.create("https://www.example.com"))
                // .setHeader("header-name", "value")
                // .POST(HttpRequest.BodyPublishers.ofString("request body"))
                .timeout(Duration.ofSeconds(1))
                .build();
            HttpResponse<String> response = client.send(request, BodyHandlers.ofString());
            // String body = response.body();
            // System.out.println(body);
            // String contentType = response.headers().firstValue("Content-Type").orElse(null);
            // System.out.println(contentType);
            if (response.statusCode() != 200) {
                bad = true;
            }
        }
        endTime = System.currentTimeMillis();
        elapsedTime = (endTime - startTime) / 1000.0;
        if (bad) {
            System.out.println("Had failed synchronous request");
        } else {
            System.out.println(" Synchronous elapsed time for " + NUM_REQUESTS + " requests = " + elapsedTime + " seconds");
        }
    }
}
