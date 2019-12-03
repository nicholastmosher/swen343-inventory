# TERM-PROJECT: KennUWare

An ERP system developed using Rust and React, with a PostgreSQL database.

## Team Inventory


- **Nick Mosher** (Team Coordinator/VC Coordinator)
- **Leon Kuhne** (Integration Coordinator)
- **Bryce Murphy** (Requirements Coordinator/Configuration Coordinator)
- **Fikayo Olutunji** (Quality Assurance Coordinator) 

---

## Prerequisites

A typical development flow requires installing the following tools:

* [Rust](https://rustup.rs)
* [Docker](https://docs.docker.com/v17.09/engine/installation/#desktop)
* [Docker Compose](https://docs.docker.com/compose/install/)

---

## How to setup the backend

Start by launching the development docker-compose image. This will start a
postgres database on port 5432 which the backend service can use.

```
docker-compose -f deployments/development/docker-compose.yml up
```

After postgres is running, navigate to the `backend/` directory and use
the `diesel_cli` tool to initialize the database and run the migrations.

```
cd backend/
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration run
```

At this point you can connect to postgres using a db client (such as IntelliJ
Ultimate), the development credentials for the database are as follows:

```
POSTGRES_DB=inventory
POSTGRES_USER=postgres
POSTGRES_PASSWORD=inventory-development
```

After running the diesel migrations, you should see the application's tables
in postgres. In order to tell the backend service how to connect to the
database, we use the `DATABASE_URL` environment variable, which can be set by
placing a definition in the `.env` file. An example `.env.dev` file is
provided with the development username, password, and database name filled out.
You can get started right away by simply copying `.env.dev` to `.env`.

```
# In backend/
cp .env.dev .env
```

Finally, launch the backend service using cargo:

```
cargo run
```

---

## How to setup the frontend
Firstly, make sure yarn is installed.

To start the server, run the following commands in the `frontend` directory
```
yarn
yarn build
yarn start
```

This should open up the url in a browser, if not navigate to `localhost:3000`

---

## Connecting to Production and Deploying

### Set up SSH

To connect to our production machine on EC2, be sure to first log in to the AWS
console and generate a new keypair for yourself. This can be done in the EC2
dashboard. You'll be prompted to download a `.pem` file, which you should put
in your `~/.ssh/` folder, e.g. as `~/.ssh/my_key.pem`. This is your private key,
make sure you don't share it with anybody. If you're on Linux, you'll also have
to change the permissions of this file to be read-only to your user.

```
chmod 400 ~/.ssh/my_key.pem
```

The next thing we need to do is generate your public key from the private key you
downloaded. I thought that AWS was supposed to automatically add your public key
to the EC2 instance but it seems to not be doing that. To find your public key,
run this command:

```
ssh-keygen -y -f ~/.ssh/my_key.pem > ~/.ssh/my_key.pub
```

Once you have your `my_key.pub`, send it to one of the members of the team who
already has access to the EC2 instance and all they have to do is append your key
to the end of the `~/.ssh/authorized_keys` file _on the EC2 instance_.

Next, you should set up your SSH config file `~/.ssh/config` with the hostname
and login information of the EC2 instance so you can ssh to the EC2 instance
easier. If that file doesn't exist, create it and add this configuration to it.
If it does already exist, just append this configuration to the end.

```
Host swen343
	Hostname ec2-3-16-181-169.us-east-2.compute.amazonaws.com
	IdentityFile ~/.ssh/my_key.pem
	User ec2-user
```

After you save this config file, you should be able to SSH into our production
EC2 instance using just `ssh swen343`.

### Pushing code to Production

On your local machine, build and push the project docker image using the
following commands:

```
docker build -t nicholastmosher/swen343-inventory:latest .
docker push nicholastmosher/swen343-inventory:latest
```

Then ssh into the production machine and run the following:

```
cd ~/inventory
docker-compose pull inventory && docker-compose up -d
```

This will launch the project as a daemon, which will not allow you to see logs
but will let you detach the terminal and keep running. If you would instead
like to see the logs, run this (without the `-d`):

```
docker-compose pull inventory && docker-compose up
```

Note that when you exit from the non-daemon command, the project will quit and
you'll need to re-run it with `-d`.

### Running the Postman Integration Tests

Testing for this project is done via the postman api. To run these tests,
make sure to first install newman, the command line collection-runner for
Postman. To install newman globally, run the following command:

```
npm install -g newman
```

If you wish to install it locally, simply remove the '-g' tag.
If you do not have the npm package manager, you can install that
here: https://www.npmjs.com/get-npm.

Once newman is installed, launch the docker-compose image if it is
not already running:

```
docker-compose -f deployments/development/docker-compose.yml up
```

The newman tests assume that the database is empty and will make
its judgments based on that. In order to ensure that the database
is in a proper state for this, in a new terminal, run:

```
diesel database reset
```

Now, launch the backend service with the following command
(remember that you have to be in the 'backend' folder for this)

```
cargo run
```

With another new terminal, navigate to the postman tests directory
(from the root directory, this is 'backend/postman') and run the
tests with this:

```
newman run Inventory-Full-V1.postman_collection.json -e Development.postman_environment.json
```

---

## Stubbing

For testing purposes, environment variables can be set in the `.env` file, such that other silos' endpoints can be tested against, given the correct url. The variables are case sensitive and once they're added, the code will automatically try these endpoints. If any of these silo URLs are not given, then Inventory will pretend to receive dummy responses without actually calling out to the service.

```
MANUFACTURING_URL=
ACCOUNTING_URL=
SALES_URL=
```

---

## Architecture

The inventory component of the ERP system is a web application which hosts a REST service as a backend and a React client-side application as a frontend. The inventory frontend provides a graphical user interface for interacting with the inventory system for use cases which are isolated to the inventory domain. However, the backend exposes more endpoints which are available for other components of the ERP system to use, which is how we enable integration with those other components. Separating our architecture allowed for more of our team to work on the code at the same time, and for a more directed architecture.  

![alt text](docs/InventoryArchitecture.png "Inventory Architecture")

### Frontend

Our frontend is a statically-served is a nodejs application, using ReactDOM, WebPack, and pug, which dynamically fetch content to render. These calls target our backend service, which hosts a server to respond to the requests of the client application. We chose this stack along with npm, a powerful package manager, because of their simple, fast, and well documented capabilities. (HTML) templates are used to allow for much of our frontend UI to be duplicated, making for less work during later modifications.

### Backend

Our backend is a Rust web service which uses the actix-web web framework for handling HTTP requests. This framework is supported by the actix Actor framework which enables high concurrency and load balancing even with local resources. Actix-web pairs nicely with the Serde (serialization/deserialization) Rust library, which allows for automatic deserialization of requests and serialization of responses. Finally, the application communicates with our Postgres database through the Diesel Object Relational Mapper (ORM) for Rust, which allows us to use native Rust functions to describe a query, and auto generates the corresponding SQL with the proper variable bindings.

![alt text](docs/BackendArchitecture.png "Backend Architecture")

### Database

![alt text](docs/DatabaseArchitecture.png "Database Architecture")

---

## Order Fulfillment Family Flow

![alt text](docs/InventorySequence-OrderFulfillment.png "Order Fulfillment Flow")

---

## API Endpoints

All our API Documentation is filled out in our Postman Documentation, located [here](http://dev-inventory.kennuware.xyz)!

---

## Design Patterns

* Actor Model - The Actix-web framework we’re using is built on top of the Actix actor system. This means that requests are handled by passing strongly typed messages to actors which understand how to handle those message types. This gives the runtime a great amount of flexibility to load-balance actors and message queues on thread pools and to make our request handlers asynchronous.
* Domain Model - Because the hope for this software is to have it grow to be an enterprise software solution, we plan to implement the Domain Model design pattern for managing our data and component interactions. Having the system be divided into objects that all contain their own data and behavior will make the system easier to work with as it grows larger and more complex
* Transform View - We are using the transform view pattern for the frontend mainly because that is the default flow of our chosen frontend technology, React. By default, React steps through the domain data it is working with and directly manipulates the DOM based on what is found and in any format desired.

---

## Potential Risks

**Miscommunication**
* Issue
  * We’ve learned from the series of events leading up to R1 that miscommunication between our team members can be a major problematic factor for us. There were rather large misunderstandings regarding how our system was to be built, resulting in costly delays.
* Plan
  * In the future, we plan to engage in more whiteboard demonstrations and pair programming sessions to ensure that all team members are on the same page regarding the actual physical implementation of what we wish to build or accomplish.

**Lack of Software Familiarity (relates to #3)**
* Issue
   * The technologies we have chosen were in large part decided upon due to the benefits the offer, such as API Endpoint Governance for Rust and massive community support for Postgres. However, not all members are familiar with all the pieces of our technology stack, so there are issues when it comes to cross checking and feedback of work between team members
 * Plan
   * Because there is at least one member that is familiar with each piece of our stack, whenever work is done for the project, we’d like to have that member run through what was done with the rest of the team during work reviews. This way, the non-familiar members will be able to learn about and become caught up on the technology, and the explaining team member will now, through the process of explaining, be double-checking the work they have done. To elaborate, should the explaining team member be unable to express the work they’ve worked towards, there is a good chance something is amiss somewhere
**Random changing of team members**
 * Issue
   * When working in teams, team members become strongly integrated in the work they perform in said team. Planning, work responsibility, roles, understanding of strengths and weaknesses, and other such forming and storming phase activities take time and effort to get through. Having team members be forced to switch teams mid-project causes the teams having their members switched around be forcibly pushed back to these early-stage team phases, which is especially problematic when a lot of plans and procedures have been solidified.
 * Plan
   * Our plan for this is to keep a solid line of documentation of our work and plans - done mostly through our master document - to allow new team members a way to get caught up to the team’s flow as quickly as possible. In addition to this, each team member must have at least one other member that is able to understand and/or perform the work they are doing so that there is someone to cover for them if they are moved to another team and to also give the new members time to catch up.

**Cross-team integration**
 * Issue
   * Moving forward, a big issue will be combining our team’s work with that of other component teams. While we’ve already experienced some pains in combining visions for the front and back end into one cohesive system, it may very well be escalated when we start talking with other teams. We already know from our past meetings that different teams have different ideas of how communication will be handled through APIs, and with Inventory’s large network of people who depend on us, we could be in for some long discussions and refactoring.
 * Plan
   * Already, we’ve been talking with other CT’s about general practices when designing their endpoints, but there really isn’t any way to know what they’re doing until we start working with them. Therefore, the best plan of action is to communicate early with these teams so we can squash any issues we have right away, and start working on our component.



**Unexpected additional requirements**
 * Issue
   * We have experienced earlier in this project that it is possible for the client to add in previously undisclosed requirements to the project. The further along a project goes, the more costly additional requirements become, and, depending on the nature of the new requirement, may potentially damage the project’s progress greatly.
 * Plan
   * The best course of action for this is to keep in touch with the client to ensure that we are regularly on the right path, and the event of an unexpected extra requirement, must always be prepared to estimate the costs of implementing said requirement. The costs must then be relayed to the client and a conversation must happen on whether or not the client is willing to accept the extra costs (or other such consequences) of trying to add the new requirement at whatever stage of the project it is proposed within.


---

## License

MIT License

See LICENSE for details.
