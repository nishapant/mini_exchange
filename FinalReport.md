# An Order Matching Engine, Gateway, Dropcopy and Tickerplant written in Rust

## Team:

**Akhil Isanaka:** (isanaka2@illinois.edu) (Co-Leader)
Akhil is a current senior at the University of Illinois at Urbana-Champaign studying Computer Science and Philosophy with an expected graduation of December 2022. In the summer of 2022, Akhil will be working with Professor Alex Bartik in his labor markets economics research group (UTIL).

**Nisha Pant:** (nishap2@illinois.edu) (Co-Leader)
Nisha is a current Senior at the University of Illinois at Urbana-Champaign studying Computer Science graduating in Spring 2022. Post-graduation, Nisha will be working as a software engineer at Modern Treasury, a payment operations company. Nisha enjoys programming in Python and Ruby focusing on backend web-based development.

**Akul Gupta:** (akulg3@illinois.edu)
Akul is a current sophomore at the University of Illinois Urbana-Champaign studying Computer Science with an expected graduation of Spring 2024. In the summer of 2022, Akul will be working at Opensense as a software development engineer.

**Aman Khinvasara:** (amantk2@illinois.edu)
Akul is a current sophomore at the University of Illinois Urbana-Champaign studying Computer Engineering with an expected graduation of Spring 2023. In the summer of 2022, Aman will be working at Amazon as a software development engineer intern.

## Project Description:

This was our final project for IE498-High Frequency Trading under professor [David Lariviere](https://david.web.illinois.edu) . We developed an orderbook-based mini-exchange using the programming language Rust. The program consists of four primary applications: a gateway, an order matching engine, a dropcopy, and a market data ticker plant, as well as client machines to interact with the exchange.

Traders connect directly to the gateway using TCP to send orders and receive order updates. Currently traders input orders using a basic command line interface. After the order is submitted, it is sent to the Gateway where we validate the information contained in the order. If it is valid, the trade is sent to the order matching engine using UDP multicast. The matching engine receives orders and determines if the incoming order matches with any resting orders on the book. If no trades can be executed, and the incoming order is a limit order, we rest the order on the orderbook. Otherwise we use the gateway to send information back to the client indicating that the trade was not executed. The tickerplant publishes public market data using UDP multicast whenever a trade is executed. The dropcopy aggregates all private market data related to a single company and sends it to the main trader/client from the company.

## Technology:

In order to develop and build our exchange we utilized a variety of technologies. The main technologies we used included Rust, CI/CD & RPI, UFW Firewall, Vagrant & VM, and Git.

## Rust:

In order to develop a fast and efficient system we decided to choose a systems programming language. We chose Rust as our primary development language instead of C or C++ as Rust is a newer language that has been becoming increasingly popular among developers and we wanted to gain more experience using it in a project.

### CI/CD & RPI:

CI/CD is extremely useful because when new code changes are made to our project during development the code is regularly built, tested, and merged to the repository. Our CI/CD server is externally set up on a Raspberry Pi instead of using a cloud provider such as AWS because it was cheaper to run 24/7 and one of our group members had an extra Raspberry Pi.

### UFW Firewall:

Because we wanted to connect our Pi to this project for at least the duration of the semester, we needed to find a way to make it more secure. We chose to create a firewall using Uncomplicated Firewall (UFW) because it was easy to set up. As of May 16th 2022, the firewall is configured to accept only SSH connections from our IP addresses and connect to GitLab.

### Vagrant & VM:

We decided to use virtual machines (VMs) for each component of our project because we wanted our system architecture to be as similar to a real world exchange as possible. The virtual machines ran a slightly modified version of CentOS that was designed to use minimal compute resources. We chose to use this operating system because our professor made it publically available to our class.

In order to simplify VM creation and management we used a software called Vagrant. Vagrant allows us to automate the creation of virtual machines and simplifies the process of managing and configuring a set of VMs.

### Git:

We choose to use git as our version control software (VCS) because it is the most commonly used VCS in industry. Version control software is important because it allows us to split the workload and collaborate efficiently.

## Components:

#### Gateway:

The gateway is the middle-man between the clients and the rest of the exchange. Clients communicate with the gateway to send orders and receive updates on submitted orders. The clients and gateway communicate via the transfer control protocol (TCP).  TCP is a reliable protocol (meaning dropped packets are automatically retransmitted), packets are ordered, and is unicast (one to one only). TCP is ideal for this component of the exchange compared to UDP as we can not have packets being dropped and need to ensure the ordering of trades.

### Client:
The client represents a single trader submitting orders to the exchange. They only interact with the gateway via TCP. Additionally all the clients have a UDP multicast listener to pick up packets from the ticker plant and dropcopy.

### Order Matching Engine:

The order matching engine receives the orders that were sent from the client through the gateway. It then utilizes an orderbook to determine if the order matches with any current resting orders and then adds that order to the order book and sends the result to the esb, dropcopy, and tickerplant.  An orderbook is an electronic list of buy and sell orders for a specific security or financial instrument organized by price level. Orderbook’s help improve market transparency as they provide information on price, availability, depth of trade, and who initiates transactions. The order matching engine utilizes UDP multicast to send packets reflecting changes to the order book to other internal components of the exchange simultaneously.



### Ticker Plant:

The market data ticker plant utilizes the information sent from the order matching engine and creates a market data book update message. The tickerplant data update message is sent using UDP multicast, to which multiple clients can connect to and ensure simultaneous receipt of market data to minimize providing unfair advantages to a few clients.  

### Dropcopy:

The dropcopy aggregates order and trade data for a single trading entity across all of its clients. The data that the gateway sends to each client is replicated in a single stream to a single machine, often used as a kill switch system at the broker-dealer. For example, one broker-dealer that has 10 different trading desks, each with its own connection to the gateway, can have a single back-office system that receives data from the dropcopy, to provide a holistic view.

## Git Repo Layout:

- On the main branch, src/ has all of our code
- Everything is run from src/main.rs
- In order to run our project you must have cargo, rust, and vagrant installed locally on your computer.

## Instructions to Run Project:

- To generate, provision, and turn on the VMs, run vagrant up. Make sure that VirtualBox or another VM manager is installed. We have already installed all the packages that the VMs need to run.

- `cd group-05-project`
- `vagrant up`


To enter the terminal of a single VM, we need to SSH into a single VM. Open a new terminal for each entity in the Vagrantfile and ssh into the respective entity. To run a specific entity, we can provide command line arguments after running our main file. Below is an example of SSH’ing into a single VM and running that entity.

`vagrant status      # lists the statuses of the VMs`

`vagrant ssh gateway`

`cargo run gateway`


Make sure that each entity is up and running BEFORE you start the gateway. For the client specifically, there will be STDIN prompts that the user will need to provide to generate the orders. If you do not want to do this manually, you can create a simple python script that periodically pipes data into the STDIN of the client entity.

## Testing:
We developed tests for our exchange in order to ensure its functionality. They were particularly useful in validating orderbook/matching engine functionality. We also implemented a basic ci/cd pipeline that is able to run our test suite and maintain functionality of the project as we progressed throughout the course of the semester.

## Project Results:


## Post-Mortem Project Analysis
### Nisha Pant Post-Mortem
**1. What did you specifically do individually for this project?**
   - Planned the entire networking architecture
   - Multithreaded and channel communication
   - Client
   - Gateway
   - TCP communications between Client and Gateway
   - Initial unicast UDP architecture, Akhil picked up where I left off there to work on multicast

**2. What did you learn as a result of doing your project?**

   - I learned at an in-depth level about how exchanges work and the individual entities
   - How to create a multithreaded distributed network using both TCP and UDP protocols
   - How UDP multicast works in contrast to TCP unicast and the benefits and tradeoffs of each
   - Better understanding of networking in general and the chronological order to establish a connection in TCP and UDP
   - Learned how to program in Rust and how Rust handles memory management

**3. If you had a time machine and could go back to the beginning, what would you have done differently?**
   - Start earlier
   - Choose a project where everyone could sufficiently contribute
   - Pseudocode the entities a bit more before starting the full implementation of all of them
   This would remove a lot of blockers when in the process of coding

**4. If you were to continue working on this project, what would you continue to do to improve it, how, and why?**
   - Create much more in-depth tests
   - We didn’t get a chance to create automated tests for a lot of the networking components of the project which are very
   - Implement a check for dropped UDP packets to ensure that all entities receive all the multicast packets
   - Create a better user interface for the client and write a script to pipe in stdin data programmatically
   - Write a script to run each of the entities on the VMs

**5. What advice do you offer to future students taking this course and working on their semester-long project (besides “start earlier”... everyone ALWAYS says that). Providing detailed thoughtful advice to future students will be weighed heavily in evaluating your responses.**

- Evaluating the strengths of each group member at the beginning of the project and making sure that each group member is able to execute their portion of the project
- Have frequent check-ups throughout the semester to ensure that the project stays on track and each member is able to contribute

### Akhil Isanaka Post Mortem
**1. What did you specifically do individually for this project?**
   - Full OME implementation & Unit Testing
   - Basic Tickerplant & Dropcopy Implementation
   - ESB design & UDP implementation. Nisha helped me gain a better understanding of networking concepts for this implementation. She also created a high level design which I followed during my implementation.
   - Final Report & Demo
   - Full CI/CD setup
   - Initial Vagrant configuration & environment setup
   - Gateway Validation contribution & Gateway order_id creation. Nisha completed the integration of these methods into the Gateway
   - Basic STDIN Client implementation. Nisha implimented interfacing with the Gateway using this version of our client.



**2. What did you learn as a result of doing your project?**
   - Gained familiarity with Rust
   - Gained a strong understanding networking concepts and experience with network programming.
   - Better understanding of CI/CD and the importance of thorough testing

**3. If you had a time machine and could go back to the beginning, what would you have done differently?**
   - Communicated more clearly with group members so we could finish the project within the semester.

**4. If you were to continue working on this project, what would you continue to do to improve it, how, and why?**
   - Enhance authentication between tcp and gateway. Right now pretty much anyone can spoof an IP address and connect to our gateway. Ideally we would add a firewall and some sort of key based authentication method.
   - Improve ESB functionality to maintain some sort of naive ordering/handle dropped packets better
   - Optimize orderbook speed via compiler intrinsics, more defined memory allocation.
   This includes messing with compiler to change stack size
   - Add in checking similar to LULD SEC. This would help ensure people don't place trades just to bog down orderbook speed
   - Create a client interface that is easier to use than command line input
   - Setup network monitoring to be able to be able to track an order from one end of the system end to the other end (submitting an order -> filling it -> getting info back to client).
   This would help us realize what parts of our system are slow.
   - Gateway OME completion
   - More Unit Testing & System integration testing

**5. What advice do you offer to future students taking this course and working on their semester-long project (besides “start earlier”... everyone ALWAYS says that). Providing detailed thoughtful advice to future students will be weighed heavily in evaluating your responses.**
   - No need to start earlier
   - Spread your workload evenly
   - Honestly evaluate group member skillsets when deciding project and work allocation
   - Take time to evaluate potential technologies and their tradeoffs
   - Python v Rust
   - Hold group members accountable

   

### Akul Gupta Post-Mortem
**1. What did you specifically do individually for this project?**
   - Collaborated on overall exchange design
   - Worked on initial trading structure
   - Tickerplant development
   - Worked UDP development and understanding how it internally works within the exchange
   - Final Report

**2. What did you learn as a result of doing your project?**
   - I learned about all the different components within an exchange and how all the different components interact with each other both from the networking side and actual data flow
   - Gained a better understanding of networking specifically TCP and UDP
   - Understood how the UDP protocol works mainly UDP multicast
   - Learned how the order matching engine works within the exchange
   - Learned the programming language Rust
   - Gained a better understanding of working with Rust libraries and testing

**3. If you had a time machine and could go back to the beginning, what would you have done differently?**
   - Distribute workload more efficiently
   - Learned even more about Rust thoroughly
   - Greater communication during development
   - Better planning of the development of the individual components of the exchange

**4. If you were to continue working on this project, what would you continue to do to improve it, how, and why?**
   - Implement a creative and better user interface so the user experience is better with the exchange
   - Enhance our testing even more thoroughly
   - Add better documentation of the code so it's easier to navigate and improve upon
   - Eventually add modifiers of orders such as hidden, icebergs, etc to create a more advanced trading exchange
   - Possibly add functionally to capture the traffic from exchange and to analyze the performance, latency, etc.

**5. What advice do you offer to future students taking this course and working on their semester-long project (besides “start earlier”... everyone ALWAYS says that). Providing detailed thoughtful advice to future students will be weighed heavily in evaluating your responses.**
- Designate/schedule multiple specific times during the week to meet with your team on a consistent basis through the semester that doesn’t usually change
- When working with a technology stack your not as familiar with include extra time to learn that tech stack and understanding it thoroughly so development can be a lot smoother
- Conduct testing development a lot more incrementally by having smaller unit tests and then continue to build out extensive tests as soon as you are able to do so
- Meet with your team in-person (if able to) to enhance collaboration, communication, and create a stronger team in general




### Aman Khinvasara Post-Mortem
**1. What did you specifically do individually for this project?**
   - Collaborated on overall exchange design
   - Dropcopy
   - Gateway order validation logic
   - Bash script for integration testing
   - Final Report

**2. What did you learn as a result of doing your project?**
   - I developed a much more nuanced and sophisticated understanding of the internal workings of an exchange, and about the complexity and edge cases involved. Particularly with regulation - for example, considering if the OME prevents wash trading
   - Making the concrete tradeoffs between TCP, UDP unicast, and UDP multicast for various interactions between components
   - Using Vagrant and VirtualBox to configure and provision virtual machines on their own subnet
   - An introduction to programming in Rust and the differences between rust and other languages I am familiar with using

**3. If you had a time machine and could go back to the beginning, what would you have done differently?**
   - Should have begun a lot earlier, and done more groundwork at the beginning to understand the real demands and difficulties of the project. For example, we thought the primary weight of the project would be in designing and implementing the logic for each of the components, and severely underestimated the difficulty of the networking aspect.
   - Set more concrete deadlines for various components and actually hold each other accountable to them

**4. If you were to continue working on this project, what would you continue to do to improve it, how, and why?**
   - Complete the client-level bash script to perform system/integration testing, and integrate that into the CI/CD pipeline. This was begun but not completed due to time constraints
   - Complete end-to-end automation of exchange setup, such as startup scripts on each of the VMs
   - Many of the aspects of the exchange were simplified. Would have added OME functionality to actually use incoming data such as the ticker symbol and if orders can be partially filled. Would have supported multiple assets.
   - Adding redundancy with multiple OMEs and other components
   - Create an API for the client side that could be used to perform algorithmic trading on top of our exchange

**5. What advice do you offer to future students taking this course and working on their semester-long project (besides “start earlier”... everyone ALWAYS says that). Providing detailed thoughtful advice to future students will be weighed heavily in evaluating your responses.**

- Think very carefully and ask a lot of questions about the different aspects of the project, and the difficulty and time associated with each of them
- Meet early and meet often for work sessions, which creates accountability and helps raise and discuss a lot of the system-level questions (i.e. How are we going to handle XYZ, or communicate between A and B?”)
- Setup a complete dev environment as soon as possible and set aside blocks of time for initial on-ramping, particularly if you are using technologies with which you are not already familiar 
