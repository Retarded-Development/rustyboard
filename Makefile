create_db:
	echo "CREATE DATABASE diesel_demo;" | sudo -u postgres psql 
	echo "CREATE USER username WITH password 'password';" | sudo -u postgres psql 
	echo "GRANT ALL ON DATABASE diesel_demo TO username;" | sudo -u postgres psql


