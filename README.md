# S02E01.server_2

## Конечные точки

1. `/registration` - POST -> создать нового пользователя в БД
2. `/auth` - POST -> инициализировать сессию для текущего пользователя
3. `/logout` - POST -> разорвать соединение
4. `/user/{id}` - GET -> получить пользователя из БД
5. `/user/{id}` - DELETE -> удалить пользователя из БД
6. `/user/{id}` - PUT -> обновить user_role


## Полезные команды

Инициализация diesel

```
diesel setup
```

Создать миграцию

```
diesel migration generate migration_name
```

Прогнать миграции

```
diesel migration run / redo
```

