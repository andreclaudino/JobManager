## Job Manager

⚠️ This is a Work in Progress

## What is this

As a Data Scientist and Data Engineer it is common to run multiple long running inferences, ETLs or any kind of long running tasks that integrate with the front-end, it is better to process everything assincronouly, with tools like [Celery](https://docs.celeryq.dev/en/stable/getting-started/introduction.html).

Job Manager is a tool to submit tasks on queues, and orchestrate then on kubernetes as transparent as possible, that means, no need to specialize an ETL, inference or training to use this tool, any http or GRPC service is available.

## Sumiting tasks

Entrypoint receives a task submit over http, submiting a tasks is as simple as a POST http request with json body. The payload to your queue function should be passed via an object on `parameters` property.

```http
POST /task/{task name}
Content-Type: application/json

{
	parameters: {
		"parameter1": "value1",
		"parameter2": 2,
		"array_parameter": [1, 2, 3]
	}
}
```

## Flow

1. Entrupoint receives requests from user, requests are submited to a broker.
2. Submiter is scaled based on queue size via knative event, or by resource consumption via regular deployment, it sends tasks to expected workers via kantive serving, when it returns, the result is published to persistence or retried in case of fail.
3. Then, entrypoint can read the result from persistence and return to user.

![Architecture](docs/architechture.png)

## Why this tool

This tool is desined to be kubernetes first, it is distributed and allow fast scaling of workers and efficient task management. There is no need to use a specific tool to create workers, different from celery, it can run with any kind of http/grpc service that can be exposed as a pod, or published externally (extenal publising ignores auto-scaling constraints).