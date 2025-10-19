# Docker Setup for MailNow

## Quick Start

1. **Clone and navigate to project:**
```bash
cd mailnow
```

2. **Configure environment:**
```bash
cp .env.example .env
# Edit .env with your SMTP credentials
```

3. **Start all services:**
```bash
docker-compose up --build
```

4. **Access the application:**
- Frontend: http://localhost:3000
- API: http://localhost:3200
- PostgreSQL: localhost:5432
- Redis: localhost:6379

## Environment Variables

Update `.env` file with your credentials:

```env
SMTP_HOST=smtp.gmail.com
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
JWT_SECRET=your-secure-jwt-secret
```

## Individual Services

**Build API only:**
```bash
docker build -t mailnow-api ./api
docker run -p 3200:3200 --env-file .env mailnow-api
```

**Build Frontend only:**
```bash
docker build -t mailnow-frontend ./mailing-fro
docker run -p 3000:3000 --env-file .env mailnow-frontend
```

## Production Deployment

```bash
docker-compose up -d
```