# GamePlan - Esports Manager Dashboard

Comprehensive esports management dashboard with decentralized authentication and AI-powered analytics.

## Architecture
- **Frontend**: Next.js with TypeScript (Web2 hosting)
- **Backend**: Rust Canister on Internet Computer (ICP)
- **Authentication**: Internet Identity
- **UI**: Shadcn/ui + Aceternity UI

## Development Setup

### Backend (ICP Canister)
cd backend
dfx start --background
dfx deploy

### Frontend (Next.js)
cd frontend
npm install
npm run dev