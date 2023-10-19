# cataars
Cat as a REST-Service

## Database setup
```
docker run -dit --name mysql -e MYSQL_ROOT_PASSWORD=root -e MYSQL_ROOT_HOST=% -e MYSQL_DATABASE=cataars -v cataars:/var/lib/mysql --restart unless-stopped -p 3306:3306 mysql
```