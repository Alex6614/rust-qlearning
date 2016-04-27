var searchIndex = {};
searchIndex['main'] = {"items":[[5,"main","main","Initializes parameters for Q-learning and runs the game infinitely.",null,{"inputs":[],"output":null}],[5,"run_game","","Runs the game and updates the Q-Value table as it plays.",null,{"inputs":[{"name":"hashmap"},{"name":"f32"},{"name":"i32"}],"output":{"name":"string"}}],[5,"decide","","Depending on chance, outputs the actions with the highest reward, or decides on an action depending on the decision policy.",null,{"inputs":[{"name":"vec"},{"name":"vec"},{"name":"i32"}],"output":{"name":"string"}}],[5,"find_best","","Looks through actions available and the player's current state, and returns a vector containing two actions that provide the top two rewards",null,{"inputs":[{"name":"hashmap"},{"name":"i32"},{"name":"vec"}],"output":{"name":"vec"}}],[0,"structs","","",null,null],[3,"State","main::structs","Describes the current state at which the system is in. At the moment it only allows for one i32 variable, but this can be extended easily.",null,null],[12,"x_1","","",0,null],[3,"Key","","A tuple of a state and an action (which is a string), which is used as a key in the q_values HashMap to find the value of taking a certain action while in a certain state",null,null],[12,"state","","",1,null],[12,"action","","",1,null],[11,"fmt","","",0,{"inputs":[{"name":"state"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",0,{"inputs":[{"name":"state"},{"name":"state"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"state"},{"name":"state"}],"output":{"name":"bool"}}],[11,"assert_receiver_is_total_eq","","",0,null],[11,"hash","","",0,null],[11,"new","","",0,{"inputs":[{"name":"state"},{"name":"i32"}],"output":{"name":"state"}}],[11,"fmt","","",1,{"inputs":[{"name":"key"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"key"},{"name":"key"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"key"},{"name":"key"}],"output":{"name":"bool"}}],[11,"assert_receiver_is_total_eq","","",1,null],[11,"hash","","",1,null],[11,"new","","",1,{"inputs":[{"name":"key"},{"name":"state"},{"name":"string"}],"output":{"name":"key"}}]],"paths":[[3,"State"],[3,"Key"]]};
initSearch(searchIndex);
