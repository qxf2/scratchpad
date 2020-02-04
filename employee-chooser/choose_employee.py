"""
This is a scratch script to choose employees in random order and then delete the employee from the list

To run, simply do `python choose_employee.py`
To stop a timer and go to the next employee, press Ctrl+C
"""
import random
import sys 
import time 

TIME_PER_PERSON = 120 #in seconds
EMPLOYEES = ['arun',\
    'raji',
    'avinash',
    'shiva',
    'annapoorani',
    'rohanD',
    'smitha',
    'indira',
    'rohanJ',
    'nilaya',
    'mohan',
    'rohini',
    'sravanti',
    'akkul',
    'kiran',
    'rahul',
    'preedhi',
    'raj']

def run_countdown_timer(max_time):
    "Create a simple countdown timer"
    #Src: https://stackoverflow.com/questions/5852981/python-how-do-i-display-a-timer-in-a-terminal
    try:
        for count in range(max_time,-1,-1):
            sys.stdout.write("\r")
            sys.stdout.write("{:2d} seconds to finish".format(count))
            sys.stdout.flush()
            time.sleep(1)
    except KeyboardInterrupt:
        sys.stdout.write("\n")
        return

def run_chooser(my_list,max_time=TIME_PER_PERSON):
    "Choose an employee at random"
    input("Before you begin, make sure you have altered the:\n    a) EMPLOYEES \n    b) TIME_PER_PERSON \n to your liking.\nPress any key to begin")
    while len(my_list) > 0:
        name = random.choice(my_list)
        my_list.remove(name)
        print(name)
        run_countdown_timer(max_time=max_time)
    print("Yay! Everyone has had their turn!")

#----START OF SCRIPT
if __name__=='__main__':
    run_chooser(EMPLOYEES)
