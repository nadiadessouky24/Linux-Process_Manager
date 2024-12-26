# Linux Process Manager 
A task manager written in Rust that displays process information, and allows users to interact with processes. This project was done for the Operating Systems course at The American University in Cairo under the supervision and guidance of Dr. Amr Elkadi.

## Features
### GUI Features
#### Display Process Information
Users can display the following information in tabular format:
- PID
- Name
- Status
- CPU %
- Memory (KB)
- Core

  
Users can also see load average information (1 minute, 5 minutes, 15 minutes) and the number of processes currently on the machine, and the number of processes in each state (running, sleeping, stopped, zombie)
#### Load Average 
Users can display load average information in a separate window, and load average information is colour coded to indicate level of severity
#### System Calls
Users can interact with the system via commands through the GUI
#### Zombie Processes
Users can display a table separately that displays zombie processes only; their PID and name
#### Process Tree Viewer
Users can see the process tree. If there is a + sign next to the process, this means that it has children. Users can then expand this tree to see the parent/child relationship between processes. Users can also click on a process and get its information
#### Threshold Monitor
Users can place a threshold on the CPU usage that is allowed by a process. If this threshold is exceeded the user is given a warning. 
#### Filter Processes 
Users can filter processes based on criteria such as PID, name, minimum CPU usage, and minimum memory usage. 
### CLI Features 
#### CPU Utilisation Graph
When the user runs the CLI, a bar graph is displayed with the number of cores on your machine and the utilisation for each core. 
#### System Statistics
Statistics are displayed at the top of the terminal. This includes load average information and total number of tasks, including the number of processes in each state
#### Process Information
Users can view process information in the terminal, and can interact with the terminal via scrolling using keyboard arrow keys 
#### Process Tree
Users can click on 't' using their keyboard to display the process tree
#### Change Niceness, Kill Processes
Users can click on 'n' to change niceness for a process, and 'k' to kill a process 

## Installation and Usage

1. **Clone the repository:**
   ```bash
   git clone https://github.com/nadiadessouky24/Linux-Process_Manager.git
   cd processManager
2. **Run the program**
    ```bash
    cargo run
3. **Choose if you want to continue with GUI or CLI**



