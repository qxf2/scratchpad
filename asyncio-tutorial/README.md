This directory contains some notes from a group discussion session on asyncio. This is not meant to be readable to anyone who did not attend the session.

## Goals for the session include:

a) Ability to scan (heading + code + diagrams) in a document

b) Active design of experiments to test methods

c) Work based on differences

d) Look to code as primary way of understanding concepts

e) Rapid prototyping of examples

f) Finding the right level of abstraction for your memory

## Structure of the session:

1. Google to learn about asyncio - 3 minutes
    > Timebox 
    > *** This went ok

2. Read three articles - 6 minutes
    > Can you scan text?
    > *** This was tough for us 

3. Explain the code in English - 3 minutes
    > 1-line summary
        *** A bit of a struggle
    > 5-line summary
        *** Did not happen

4. Redo an exercise from memory - 5 minutes
    > How close to a black box are you?
    > *** we did surprisingly well here

5. Experiment with the example - 10 minutes
    > Were you imaginative when reading?
    > Can you report errors correctly?

6. List vocabulary terms - 2 minutes
    > *** we were ok

7. List questions - 3 minutes 
    > *** we were ok

8. Guess the interface - 2 minutes
    > *** not that well

9. Let's try a for loop - ?? async for doc in get_docs():

10. Read the documentation - 10 minutes

## Copy-paste of Google Meet chat output

```python
You2:13 PM
https://realpython.com/async-io-python/
https://pythonprogramming.net/asyncio-basics-intermediate-python-tutorial/
https://medium.com/@yeraydiazdiaz/asyncio-for-the-working-python-developer-5c468e6e2e8e
Rahul Bhave2:14 PM
https://quentin.pradet.me/blog/using-asynchronous-for-loops-in-python.html
You2:25 PM
1. This article shows how you can still use asyncio with older versions of python that do not support await and async keywords
Preedhi Vivek2:26 PM
his blog post details out about python's asyncio library and its usage by providing code snippets and explaining the same clearly.
Akkul DN2:26 PM
Async io is basically used to run multiple operations concurrently
Rahul Bhave2:26 PM

https://medium.com/@yeraydiazdiaz/asyncio-for-the-working-python-developer-5c468e6e2e8e

1- With the aynchronous CPU resourses can be utilised effectively to run processes so time to run the entire job/code can be reduced
Sravanti Tatiraju2:27 PM
The blogs how we can use ayncio divide tasks and schedule runs
Kiran CV2:27 PM
Asyncio is used to write asynchronous programming, which helps to distribute the tasks scheduling .
Mohan Kumar2:28 PM
A blog about Async Io python libraries walkthrough with example codes.

The blog says how to setup env with venv.
and with the asyncio package and Async/await
Rajkumar MeenakshiSundaram2:28 PM
I referred to the medium article: Asyncio is a python library which executes the coroutines task when sleep is made using await
```.
Preedhi Vivek2:29 PM

This blog post details out about python's asyncio library and its usage by providing code snippets and explaining the same clearly.
The author starts by explaining the concepts such as threads, loops and coroutines initially and later gets into explaining the same with code snippets.
Starts of with a small example and later explains with in depth concepts.
He also tells about the order of execution.
Rajkumar MeenakshiSundaram2:33 PM
import asyncio

async def test()
	print('test1')
	await asyncio.sleep(3)
	print('test2')

async def retest()
	print('retest1')
	await asyncio.sleep(2)
	print('retest2')
	
async def main():
    tasks = [test(), retest()]

asyncio.run(main())

Expected output:
test1
retest1
retest2
test2
Akkul DN2:36 PM
"""
The following program shows the working of asyncio.

"""

import asyncio

async def first():
 print("This is the first statement")
 await async.sleep(2)
 print("This is the second statement")

async def second()
  print("This this is the first statement")
  await async.sleeep(2)
  print("this is the second statement")

def main():
  async.gather(first().second())

async.run(main())
Rajkumar MeenakshiSundaram2:37 PM
import asyncio

async def test()
	print('test1')
	await asyncio.sleep(3)
	print('test2')

async def retest()
	print('retest1')
	await asyncio.sleep(2)
	print('retest2')
	
async def main():
    tasks = [test(), retest()]
	await asyncio.gather(*tasks)
asyncio.run(main())

Expected output:
test1
retest1
retest2
test2
Rahul Bhave2:37 PM
import asyncio

class asynch(self)
# asynch method
async def get_docs():

    print("get doc")

def get_doc1():

    print("not async")

if __name__ == "__main__":
Mohan Kumar2:38 PM
#asyncio.py

import asyncio 

asyncio def count():
    print("one")
    await asyncio.sleep(1)
    print("two")

asyncio def main():
    for count in range(3):
        count()


if __name__ == "__main__"
Sravanti Tatiraju2:38 PM
import asyncio

async def task1():
    print("Running task1")
    await asyncio.sleep(1)
    print("Running task1 again")


async def task2():
    print("Running task2")
    await asyncio.sleep(2)
    print("Running task2 again")


async def main():
    await asyncio.gather(task1, task2)
Kiran CV2:38 PM
"""
Snipet on Asyncio
"""
import asyncio

async def one():
    print('Running in one')
    await asyncio.sleep(2)
    print('Explicit context switch to two again')


async def two():
    print('print two')
    await asyncio.sleep(0)
    print('Implicit context switch back to one')


async def main():
    tasks = [one(), two()]
    await asyncio.gather(*tasks)


asyncio.run(main())
Preedhi Vivek2:39 PM
import asyncio

aynsc def prog1():
    print ("Inside prog1")
    await async. sleep (1)
    print ("Inside prog1 again")

async def prog2():
    print ("Inside prog2")
    await async. sleep (1)
    print ("Inside prog2 again")


async def main():
    async.gather (prog1(),prog2())

if __name__ == "__main__":
    asyncio.run(main())
Preedhi Vivek2:41 PM
{:.2f}
You2:45 PM
event loop, coroutine, tasks, gather, async, await, run, concurrency, parallel
Kiran CV2:45 PM
coroutine
event loop
async
await
asynci.gather
Sravanti Tatiraju2:45 PM
coroutines, gather, async, await, event loop, 
Rajkumar MeenakshiSundaram2:46 PM
coroutine, parallel, concurrent, async,await,gather
Rahul Bhave2:46 PM
StopAsyncIteration
Preedhi Vivek2:46 PM

await
async
concurrency
async sleep
parallelism
looping
events 
coroutines
asyncio
Mohan Kumar2:46 PM
async
await
sleep
time
count
library
package
venv py3async
Akkul DN2:47 PM
coroutine,eventloop
Rahul Bhave2:47 PM
popleft
buffer.popleft
Kiran CV2:52 PM
How it chooses one task from another...on what criteria the asyncio checks?
coroutine ?
event loop?
Sravanti Tatiraju2:52 PM

Understand how event loop works and is different from threads.
Understand about concurrency and parallelism with some examples.
How different is sleep function in asyncio
Akkul DN2:52 PM
Whats the use of the timer if asyncio.sleep passes the control to the eventloop anyways?
Preedhi Vivek2:52 PM
Difference between time.sleep and async.sleep (synchronous and asynchronous)
What's a coroutine?
How do i test this with a real IO like writing to a File and doing some math operations and verifying asynchio.
You2:52 PM
> How do I send a list of tasks to gather?
> How does this work with file handling?
> What is a coroutine?
> What is an 'event loop'?
> What is the difference between concurrency and parallelism?
> Are there similar modules in Python?
> What happens if one task takes too long? Is there a timeout?
> What happens if a method fails or has exception?
Rajkumar MeenakshiSundaram2:52 PM
How await asyncio.gather(*tasks)?
Mohan Kumar2:53 PM
disadvantages of asyncio?
-diff b/w time, sleep, async,await?
-coroutine and awaitables?
Rahul Bhave2:53 PM
1. will asynchrous module will reduce the time of total execution of testsuite?
2. How to decide yoeld points while writing coroutines?

3. Can we decide sequence of tasks?
Rajkumar MeenakshiSundaram2:54 PM
How await asyncio.gather(*tasks)?
Event loop meaning?
Rahul Bhave2:55 PM
Methods I can think off- Fetch, Buffer, 
Preedhi Vivek2:55 PM

Intterupt
exception handling
Rahul Bhave2:56 PM
schedule_task
Preedhi Vivek2:56 PM
await_until
Mohan Kumar2:57 PM

thread start
thread stop
await_until
You2:57 PM
* delete_task
* stop_run
* stop_task
* await_until
* list_tasks
* interrupt(task)
* reset_loop()
* callback_on()
* inspect_task()
* get_task_status()
Kiran CV2:57 PM
start or stops a task
random task selection
Rajkumar MeenakshiSundaram2:57 PM
fetch_task, iterrupt, await_untill
Sravanti Tatiraju2:57 PM
thread start, schedule task
Akkul DN2:57 PM
How does asyncio intterupt the task
schedule task
