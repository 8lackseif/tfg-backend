# tfg-backend

docker run -p 3306:3306 -v $PWD/my_database:/var/lib/mysql -e MYSQL_ALLOW_EMPTY_PASSWORD=yes -e MYSQL_DATABASE=my_database --rm -d mariadb