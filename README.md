# S02E01.server_2

## Конечные точки

1. `/user` - POST -> создать нового пользователя в БД
2. `/user/{id}` - GET -> получить пользователя из БД


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

