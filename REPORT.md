#CIS 198 Final Project
##Q-Learning- Alexander Ma
###Summary

My final project is a rust implementation of the Q-Learning algorithm developed by a Ph.D from Cambridge University. At the moment it is able to take in a program/game that outputs the state of the system in a structured way, and learn how to perform better in the system based on some metric.   
Q-Learning is a model free reinforcement learning technique that can take in a system, such as a game, and learn the best route to the state with the highest reward after ‘playing’ the system multiple times. The algorithm keeps a memory of its actions via a 2D Q-value table, with one axis denoting the state in which the system is in, and another axis denoting an action that takes the system to the next state. The values in the table represent how favorable taking a certain action is given a certain state. The goal is for the algorithm to know what the best action to take in each state in order to get the highest reward is once an iteration through a system is complete. The learning algorithm takes in multiple parameters denoting things such as how much the algorithm learns from each iteration.  
The game I used to test my algorithm is a tweaked version of the castle game from an earlier homework. There is no longer any concept of health, and the player must simply reach from one end of a castle to another end without entering certain rooms that would result in a poor score. The program takes in input and output in a formatted way so that it would be compatible with my algorithm. I also created three variants of the game. They differ in the difficulty of finding a correct path from one castle to the other.  
In terms of files, there are four key ones:  

-	An executable containing the castle game
-	A json file containing data on how to build the castle for the game
-	An executable rust file that implements the algorithm and plays whatever system it is used on
-	A rust file containing structs used in the algorithm

The motivation behind implementing this algorithm comes from my interest in AI in general. I thought rust would be a great language to write the algorithm because of its speed.

###Approximate Time Spent

-	Proposal Presentation: 3 hours. I used this time to research and understand fully how the algorithm worked. I planned what sort of files I would need for my program and how I will eventually test my algorithm. I also wrote the project proposal in this time.
-	Milestone 1: 5 hours. Coded up the algorithm, created the structs needed for the algorithm, and researched ways in which the algorithm could interact with the executable file with the game. Experimented with pipes and ended up going with the processes module in the standard library. Created documentation for my code.
-	Milestone 2: 8 hours. Tweaked the game so it would be compatible with the algorithm and I changed the rules of the game. Linked the executable file to my rust file using the processes module. Created documentation for my code.
-	Final Presentation: 8 hours. Further optimizations to the algorithm, and split up the main function in my code into separate functions for readability. Created a new action policy and created variants of the game. Wrote up the final report. Created documentation for my code.

###Accomplishments
- The algorithm: I consider this a success because the algorithm is able to adapt to different variants of the game and beat them. I introduced two action policies- normally, when more than one action can be performed at a given state, the action that puts the player in the most favorable state is picked. But, by chance, it is possible for the algorithm to pick another state. The first policy simply picks an action at random, while the second policy picks the action that takes the player to the second most favorable state. While testing my algorithm, I found that the first policy takes a few hundred iterations to learn a way to win the game consistently. However, the second policy learns blazingly fast and only requires a few iterations to win the game (around four iterations for even the hardest variant).
- The game: I managed to tweak the game in a way that allows me to judge how well my algorithm does (I used the number of iterations as a metric).
- Child processes: This only took up a couple of lines of my code but I think the main takeaway from this entire project is being able to wrap another executable into a child process and be able to interact with it using stdin and stdout handles. Here, I wrapped the executable game file in a child process and allowed the algorithm to interact with it and even spawn new ones.

        let mut child = Command::new("game")
                        .stdin(Stdio::piped()).stdout(Stdio::piped())
                        .spawn().unwrap_or_else(|e| { panic!("failed to execute child: {}", e)});
        let mut buffer = std::io::BufReader::new(child.stdout.take().unwrap());
        let mut input = child.stdin.take().unwrap();

###Design Decisions
- To implement the table, I used a Hashmap instead, believing that it would allow for fast inserts and searches. The keys of which needed to be a tuple, which stored the current state and the action taken, and the values corresponding to each key would simply be an integer.
- I decided to split main and the game loop so I could keep a single Q-value table that can be used over multiple iterations of the system.
- I also needed a way in which the program could tell when the child process terminated. I overcame this issue by making the game print something that was not in the format accepted by the Q-Learning program. This creates an error which allows me to handle by performing an early return.

###Testing Approach and Results
To test my program, I modified the castle game’s json file to create a castle with over 30 edges and 15 rooms. If the player got from one side of the castle to the other, they are awarded 1000 points. As the difficulty increased, I increased the number of ‘bad rooms’ which would end the iteration and output a negative score of -1000. In the easy game, there were only 2 bad rooms. In the hard one, 4. In the very hard one, 7 (there is only one correct path for this one!).  To see how well my program did, I simply needed to see how many iterations it took for the algorithm to start consistently finding a path to the other end of the castle. With the first decision policy, I noticed, unsurprisingly, that the number of iterations increased as the difficulty increased. (Easy: 500 iterations, Hard: 1000 iterations, Very Hard: >25000 iterations). However, with the second policy, the algorithm learned almost immediately how to traverse the castle (exactly four iterations for each difficulty). I still don’t really know why it works so well, or why there are always four iterations only.

### Possible Extensions

-	Increasing the number of variables to describe a state
-	Playing around the gamma number and changing the probability that the best action is picked at each step to see what combination leads to the fastest adaptation to a system.
-	Find a better way to figure out when the child process stops
-	Introduce criteria for the program for it to terminate, because at the moment the program runs for an infinite amount of time. Perhaps it could stop once the reward reaches a certain threshold, or consistently win a certain number of times.
-	Represent the table and the decision making process as a neural network, which would make the program more powerful and probably take less time to find a winning strategy to a game.

### Post-mortem
-	What went well: There are a lot of modules in the standard library that I haven’t explored yet. Child processes were really cool and I hope to find more ways to use a language beyond simple scripts. I ran into a few borrowing errors with the compiler, but because I’m using pretty standard primitives, I was able to mitigate the errors with the clone function.
=	What could have went better: I had to refactor my code a lot for readability and given enough time I would have liked to create another game on which to test my program.
