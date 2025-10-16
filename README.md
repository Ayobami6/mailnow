# MailNow - Developer Email Platform

A comprehensive email service platform built with modern technologies, providing API-first email delivery, admin management, and user dashboard.

## Architecture

### 🦀 Backend API (`/api`)
- **Tech**: Rust + Actix-Web + Diesel ORM
- **Database**: PostgreSQL
- **Port**: 3200
- **Features**: RESTful API, CORS enabled, connection pooling

### 🐍 Admin Panel (`/admin`) 
- **Tech**: Django + Django REST Framework
- **Purpose**: Internal admin management
- **Apps**: Core, Users, Developers, Utils

### ⚛️ Frontend (`/mailing-fro`)
- **Tech**: Next.js 15 + TypeScript + Tailwind CSS
- **Features**: Auth, Dashboard, Email Templates, AI Chatbot
- **UI**: Radix UI components

## Database Schema

```sql
-- Core table structure
CREATE TABLE team_members (
    id BIGSERIAL PRIMARY KEY,
    role VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    company_id BIGINT NOT NULL REFERENCES companies(id),
    user_id BIGINT NOT NULL REFERENCES users(id)
);
```

## Quick Start

### Prerequisites
- Rust 1.70+
- Python 3.12+
- Node.js 18+
- PostgreSQL 14+

### Database Setup
```bash
# Create database and user
psql -U postgres -f setup_db.sql
```

### Environment Setup
```bash
# Copy environment files
cp .env.example .env
cp admin/.env admin/.env
cp api/.env api/.env

# Update the values in each .env file
```

### Backend API
```bash
cd api
cargo run
# Server starts on http://127.0.0.1:3200
```

### Admin Panel
```bash
cd admin
uv sync
python manage.py migrate
python manage.py runserver
```

### Frontend
```bash
cd mailing-fro
pnpm install
pnpm dev
```

## Environment Variables

```env
# API
DATABASE_URL=postgresql://user:pass@localhost/mailnow
PORT=3200
DEBUG=1
RUST_LOG=actix_web=info

# Django
SECRET_KEY=your-secret-key
DEBUG=True
```

## Project Structure

```
mailnow/
├── api/                 # Rust API server
│   ├── src/
│   │   ├── config/      # DB configuration
│   │   ├── models/      # Database models
│   │   ├── repositories/# Data access layer
│   │   └── utils/       # Utilities
│   └── Cargo.toml
├── admin/               # Django admin panel
│   ├── admin/           # Django project
│   ├── core/            # Core app
│   ├── users/           # User management
│   └── pyproject.toml
├── mailing-fro/         # Next.js frontend
│   ├── app/             # App router
│   ├── components/      # React components
│   └── package.json
└── schema.sql           # Database schema
```

## Features

### API Endpoints
- `GET /` - Health check
- Authentication & user management
- Email sending & templates
- Webhook handling

### Admin Features
- User management
- Developer portal
- System configuration
- Analytics & logs

### Frontend Features
- User authentication
- Email template builder
- Dashboard & analytics
- API key management
- Webhook configuration

## Development

### API Development
```bash
cd api
cargo watch -x run  # Auto-reload on changes
```

### Database Migrations
```bash
cd api
diesel migration run
```

### Frontend Development
```bash
cd mailing-fro
pnpm dev  # Hot reload enabled
```

## Tech Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| API | Rust + Actix-Web | High-performance backend |
| ORM | Diesel | Type-safe database queries |
| Admin | Django + DRF | Admin interface |
| Frontend | Next.js + TypeScript | User interface |
| Database | PostgreSQL | Data persistence |
| UI | Tailwind + Radix UI | Component library |

## Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Run tests
5. Submit pull request

## License

MIT License