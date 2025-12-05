# Git Setup for Personal GitHub (Without Affecting Office GitLab)

## Current Situation
- Your office laptop has GitLab credentials configured globally
- You want to push this one project to personal GitHub
- You don't want to mess with your office git setup

## Solution: Local Git Config (Per-Repository)

### Step 1: Check your current office config (just to see)
```bash
git config --global user.name
git config --global user.email
```

### Step 2: Set credentials ONLY for this repository
```bash
cd /Users/sahil.gakhar/sahil-projects/linkdin-auto-reply

# Set your personal GitHub name and email (local to this repo only)
git config --local user.name "Sahil Gakhar"
git config --local user.email "your.personal.email@gmail.com"  # Replace with your email
```

### Step 3: Commit and push
```bash
# Commit
git commit -F COMMIT_MESSAGE.txt

# Add GitHub remote
git remote add origin https://github.com/sagkhr23/linkdin-auto-reply.git

# Push (will ask for GitHub credentials)
git branch -M main
git push -u origin main
```

### Step 4: Authenticate with GitHub
When you push, you'll need a **Personal Access Token** (not password):

1. Go to: https://github.com/settings/tokens
2. Click "Generate new token (classic)"
3. Give it a name: "LinkedIn Bot Laptop"
4. Select scopes: `repo` (full control of private repositories)
5. Generate token and copy it
6. When git asks for password, paste the token

## ✅ What This Does

- **Local config** (--local): Only affects this one repository
- **Global config** (--global): Your office GitLab config - UNCHANGED
- When you work on office projects, they still use office credentials
- This project uses your personal GitHub credentials

## 🔍 To Verify

```bash
# Check this repo's config (personal)
cd /Users/sahil.gakhar/sahil-projects/linkdin-auto-reply
git config --local user.name
git config --local user.email

# Check global config (office - should be unchanged)
git config --global user.name
git config --global user.email
```

## 📌 No Need to "Reset"

Your office config is never changed! Each repository can have its own credentials.
When you go back to office projects, they automatically use the global (office) config.

