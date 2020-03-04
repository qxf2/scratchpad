"""
This script will be used to show examples of using asyncio as part of a group discussion

Goals for the session include:
a) Ability to scan (heading + code + diagrams) in a document
b) Active design of experiments to test methods
c) Work based on differences
d) Look to code as primary way of understanding concepts
e) Rapid prototyping of examples
f) Finding the right level of abstraction for your memory

Structure:
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
"""
import asyncio
import time 

async def print_me(name):
    "A method that sleeps for a second and prints a name"
    await asyncio.sleep(1)
    print(name)

async def my_method():
    "A method to gather and call other async methods"
    await asyncio.gather(print_me("Arunkumar"),print_me("Muralidharan"))

#----START OF SCRIPT
if __name__ == "__main__":
    start_time = time.time()
    asyncio.run(my_method())
    duration = time.time() - start_time 
    print('The script {} took {:.2f} seconds'.format(__file__,duration))

