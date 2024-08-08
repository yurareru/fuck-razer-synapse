# How to automatically startup the program

1. Get the [binary file](https://github.com/yuuraeru/fuck-razer-synapse/releases/download/v0.1.0/fuck-razer-synapse.exe) and store it somewhere.
2. Open Task Scheduler and `Create Task`.
3. Check `Run with highest privileges`.
4. Create a new trigger and change begin the task to `At log on`.
5. Set `Delay task for: x` to around 1 minute.
6. Create a new action and set `Program/script` to the stored binary file.
7. Under `Settings`, uncheck the `Stop the task if it runs longer than: x`.
