# TERM-PROJECT: KennUWare

An ERP system developed using Rust and React, with a PostgreSQL database.

## Team Inventory

- Nick Mosher (Team Coordinator/VC Coordinator)
- Leon Kuhne (Integration Coordinator)
- Bryce Murphy (Requirements Coordinator/Configuration Coordinator)
- Fikayo Olutunji (Quality Assurance Coordinator) 

## Prerequisites

A typical development flow requires installing the following tools:

* [Rust](https://rustup.rs)
* [Docker](https://docs.docker.com/v17.09/engine/installation/#desktop)
* [Docker Compose](https://docs.docker.com/compose/install/)

## How to begin

Start by launching the development docker-compose image. This will start a
postgres database on port 5432 which the backend service can use.

```
docker-compose -f deployments/development.yml up
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

You can set up a "prod" git remote that you can push to in order to upload the
latest code to our EC2 production machine. To do this, just run the following
(this assumes you've followed the SSH setup above)

```
git remote add prod swen343:~/git/inventory
```

You can then push and pull code to and from `prod` just as if it were a Github
repository, e.g. using `git push prod master`, etc. To launch that latest code
on the production machine, you'll need to do one more step.

Start by SSHing into the production machine

```
ssh swen343
```

There's a directory called "inventory" which is where our source code is stored
and built on production. It's a git repository, but it won't automatically have
the changes you pushed when you did `git push prod master` (that code lands in
the headless `~/git/inventory` repository on the production machine). In order
to deploy the latest code, we have to pull the code from the headless repository.
To do that, we just run

```
# On the swen343 machine, in ~/inventory
git pull origin master
```

After you've pulled the latest code, you can run the code like normal:

```
cargo run --release
```

## License

MIT License

See LICENSE for details.
