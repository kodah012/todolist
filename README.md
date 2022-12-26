# Todolist

## Setting up Docker

### 1. Backend (Open new terminal)

1. `cd backend`
2. `docker build -t backend:latest .`
3. `docker run --init -p 3000:3000 backend:latest`

### 2. Frontend (Open new terminal)

1. `cd frontend`
2. `docker build -t frontend:latest .`
3. `docker run -p 8080:8080 frontend:latest`

### 3. Running everything

1. Go to `localhost:8080` in a browser