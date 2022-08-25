# SysPerf : A system performance monitor for Linux

## 0 | Project Info
- Name : SysPerf
- Group number : 4

### Team Members : 

#### Member 1
- Name : P K Navin Shrinivas
- SRN : PES2UG20CS237
- Email : [karupal2002@gmail.com](mailto:karupal2002@gmail.com)
- GitHub : https://github.com/NavinShrinivas 

<!--Fill your details in the same format -->

## 1 | Project description
SysPerf is a new proposed project aiming to achieve impressive monitoring system information in few short sprints. SysPerf is more of a hobby project and complies with the open source nature of Linux. With SysPerf all developers and users alike can have a single go to tool for monitoring their system performance. Some of the initial functional requirements planned are : 
- A friendly TUI interface.
- Providing the tool through all famous package managers.
- Seeing memory, CPU and disk usage.
- List all processes with specific usage.

## 2 | The plan

- We plan to finish this project in 2 short sprints (8 weeks).
- This project is split into 2 phase (modules), this fits well into our 2 sprints and helps splitting work amongst us.
- The first sprints start with spending a short while on requirements, research and tool setup. 
- The remaining weeks of sprint 1 is spent in creating the various modules that communicate with the kernel and provide an interface for the TUI to display.
- Each person in the team takes up a moduke to implement, we will have the regualr scrum meetings every 3 days to make sure we have a consistent intetface and progress goes on as usual.
- As this is a systems based _tool_ we will not have any databse, but other functional requirements are as follows :
    - TUI, with stat testing
    - Unit testing is being made the standard.
    - Buferring stats for upto 30 seconds
- In sprint 2, we will start working on TUI. Here 1 of the team mates manages the structres of the TUI and the rest connect the previously mentioned interface to the TUI structures.
- As for qualitative requirements, we have the follwing in mind : 
    - No sudo perms to run the frontend 
    - Being very responsive 
    - Neat TUI with minimal configurations
