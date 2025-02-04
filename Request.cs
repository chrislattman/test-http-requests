const int NUM_REQUESTS = 100;
bool bad = false;
bool asyncBad = false;

HttpClient client = new()
{
    Timeout = TimeSpan.FromSeconds(1)
};

long startTime = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();
var tasks = new List<Task>(NUM_REQUESTS);
for (int i = 0; i < NUM_REQUESTS; i++)
{
    // HttpRequestMessage request = new(HttpMethod.Post, "https://www.example.com")
    // {
    //     Content = new StringContent("request body")
    // };
    // request.Headers.Add("header-name", "value");
    //
    // tasks.Add(Task.Run(() => client.SendAsync(request).ContinueWith(responseTask =>
    //
    // or without a header:
    //
    // tasks.Add(Task.Run(() => client.PostAsync("http://www.example.com/",
    //     new StringContent("request body")).ContinueWith(responseTask =>
    tasks.Add(Task.Run(() => client.GetAsync("http://www.example.com/").ContinueWith(responseTask =>
    {
        HttpResponseMessage response = responseTask.Result;
        if (((int)response.StatusCode) != 200)
        {
            asyncBad = true;
        }
        else
        {
            // string? contentType = null;
            // if (response.Headers.TryGetValues("Content-Type", out IEnumerable<string>? values))
            // {
            //     contentType = values.First();
            // }
            // Console.WriteLine(contentType);
            // string body = response.Content.ReadAsStringAsync().Result;
            // Console.WriteLine(body);
        }
    })));
}
Task.WaitAll(tasks);
long endTime = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();
double elapsedTime = (endTime - startTime) / 1000.0;
if (asyncBad)
{
    Console.WriteLine("Had failed asynchronous request");
}
else
{
    Console.WriteLine("Asynchronous elapsed time for " + NUM_REQUESTS + " requests = " + elapsedTime + " seconds");
}

startTime = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();
for (int i = 0; i < NUM_REQUESTS; i++)
{
    HttpRequestMessage request = new(HttpMethod.Get, "https://www.example.com");
    // HttpRequestMessage request = new(HttpMethod.Post, "https://www.example.com")
    // {
    //     Content = new StringContent("request body")
    // };
    // request.Headers.Add("header-name", "value");
    HttpResponseMessage response = client.Send(request);
    if (!response.IsSuccessStatusCode) // code is not in range 200-299
    {
        bad = true;
    }
    else
    {
        // string? contentType = response.Content.Headers.ContentType?.ToString();
        // Console.WriteLine(contentType);
        // string body = response.Content.ReadAsStringAsync().Result;
        // Console.WriteLine(body);
    }
}
endTime = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();
elapsedTime = (endTime - startTime) / 1000.0;
if (bad)
{
    Console.WriteLine("Had failed synchronous request");
}
else
{
    Console.WriteLine(" Synchronous elapsed time for " + NUM_REQUESTS + " requests = " + elapsedTime + " seconds");
}
