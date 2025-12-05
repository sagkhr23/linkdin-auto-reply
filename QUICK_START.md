# Quick Start Guide

## To Commit and Push to GitHub

### 1. Commit your changes
```bash
git commit -F COMMIT_MESSAGE.txt
```

### 2. Create repository on GitHub
Go to: https://github.com/new
- Repository name: `linkdin-auto-reply`
- Description: LinkedIn auto-reply bot using Rust and Ollama
- Make it Public (so others can use it)
- Don't initialize with README (we already have one)

### 3. Push to GitHub
```bash
git remote add origin https://github.com/sagkhr23/linkdin-auto-reply.git
git branch -M main
git push -u origin main
```

### 4. (Optional) Add topics on GitHub
After pushing, add these topics to your repo for better discoverability:
- `linkedin`
- `rust`
- `ollama`
- `chrome-extension`
- `llm`
- `automation`
- `typescript`

---

## Your Personal Config (Local Only)

Before running the backend, create `backend/.env`:

```bash
cp backend/.env.example backend/.env
```

Then edit `backend/.env` with your info:
```env
PHONE_NUMBER=+91-8650030078
RESUME_LINK=https://drive.google.com/file/d/1CBBYwKQ6oEHW99sw4oQAOE47FjunAnLf/view?usp=sharing
USER_NAME=Sahil
```

These files stay on your machine and won't be pushed to GitHub.
