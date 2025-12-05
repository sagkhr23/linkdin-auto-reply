# LinkedIn Auto-Reply Bot

A semi-automated tool that generates personalized replies to LinkedIn recruiter messages using your resume and profile, powered by a local Rust backend and Ollama LLM.

## 🎯 What It Does

- **Detects recruiter messages** on LinkedIn (by keywords like "opportunity", "role", "CTC", etc.)
- **Generates contextual replies** using your resume and LinkedIn About section
- **Fills the message box** with the generated reply (you review and press Send)
- **Runs 100% locally** using Ollama (no OpenAI API costs)

## 🏗️ Architecture

```
Chrome Extension (TypeScript)
    ↓ (detects recruiter message, sends text)
Background Service Worker
    ↓ (calls backend API)
Rust Backend (Axum)
    ↓ (builds prompt with resume/profile)
Ollama (llama3 or other model)
    ↓ (generates reply)
Extension fills LinkedIn message box
```

## 📋 Prerequisites

1. **Rust** (latest stable): [rustup.rs](https://rustup.rs/)
2. **Node.js + npm** (for TypeScript compilation)
3. **Ollama** installed and running: [ollama.com](https://ollama.com/)
   - Pull a model: `ollama pull llama3`
4. **Chrome** or Chromium-based browser

## 🚀 Installation

### 1. Clone the repo

```bash
git clone https://github.com/sagkhr23/linkdin-auto-reply.git
cd linkdin-auto-reply
```

### 2. Set up your personal information

```bash
# Copy example files
cp resume-summary.txt.example resume-summary.txt
cp linkdin-about-section.txt.example linkdin-about-section.txt
cp backend/.env.example backend/.env

# Edit with your actual data
# - resume-summary.txt: your resume text
# - linkdin-about-section.txt: your LinkedIn About section
# - backend/.env: your phone number, resume link, and name
```

Edit `backend/.env`:
```env
PHONE_NUMBER=+XX-XXXXXXXXXX
RESUME_LINK=https://drive.google.com/your-resume-link
USER_NAME=YourName
```

### 3. Build the Rust backend

```bash
cd backend
cargo build --release
cd ..
```

### 4. Build the Chrome extension

```bash
cd extension
npm install
npx tsc
cd ..
```

### 5. Load the extension in Chrome

1. Go to `chrome://extensions/`
2. Enable **Developer mode** (top right)
3. Click **Load unpacked**
4. Select the `extension/` folder

## ▶️ Usage

### 1. Start Ollama (if not running)

```bash
ollama serve
```

### 2. Start the Rust backend

```bash
cd backend
cargo run --release
```

The server will listen on `http://127.0.0.1:8000`.

### 3. Use the extension

1. Open a **LinkedIn recruiter message** in your browser.
2. Click the **extension icon** in your toolbar.
3. The extension will:
   - Detect if it's a recruiter message
   - Call the backend to generate a reply
   - Fill the LinkedIn message box with the reply
4. **Review the reply** and press **Send** manually.

### Send Button Note

LinkedIn's editor requires a "real" user input to enable the Send button. After the extension fills the text:

- Press **Space + Backspace**, or
- Press **Cmd/Ctrl + A, Cmd/Ctrl + V**

This triggers LinkedIn's internal state update and enables Send.

## ⚙️ Configuration

### Customize the prompt

Edit `backend/src/main.rs`, specifically the `prompt` in the `generate_reply` function. You can adjust:

- Tone (formal, friendly, concise)
- What info to include/exclude (relocation, notice period, etc.)
- Response structure and formatting

### Change the Ollama model

In `backend/src/main.rs`, line ~120:

```rust
model: "llama3".to_string(),
```

Replace `"llama3"` with any model you've pulled (e.g., `"qwen2.5"`, `"mistral"`).

### Add more recruiter keywords

In `extension/content.ts`, update the `recruiterKeywords` array:

```ts
const recruiterKeywords = [
  "opportunity",
  "role",
  "position",
  // add more...
];
```

## 📝 How It Works

1. **Content script** (`extension/content.ts`) runs on all LinkedIn pages.
2. When you click the extension icon, it:
   - Finds the latest message text and sender headline.
   - Checks if it matches recruiter patterns.
3. Sends the message to the **background service worker** (`extension/background.ts`).
4. Background worker calls `POST http://localhost:8000/generate_reply`.
5. **Rust backend** loads your `resume-summary.txt` and `linkdin-about-section.txt`, builds a prompt with your personal info from `.env`, and calls Ollama.
6. Ollama generates a reply; backend returns JSON `{ reply, reason }`.
7. Extension inserts the reply into LinkedIn's `contenteditable` div with proper formatting.

## ⚠️ Important Disclaimers

- **LinkedIn Terms of Service**: Automated messaging can violate LinkedIn's ToS. This tool is designed as a **reply assistant** (you still manually press Send), not a fully automated bot.
- **Use at your own risk**: LinkedIn may restrict or ban accounts for automation.
- **Data privacy**: All processing happens locally (your resume never leaves your machine except to your local Ollama instance).

## 🛠️ Future Improvements

- [ ] Add conversation history context (full thread, not just last message)
- [ ] Better DOM selectors (LinkedIn changes these frequently)
- [ ] Auto-detect more recruiter patterns (job titles, company domains)
- [ ] Option to auto-send (with rate limiting and safety checks)
- [ ] Support for multiple profiles/resumes
- [ ] Web UI for managing prompts and testing replies

## 📄 License

MIT License - feel free to fork, modify, and use.

## 🤝 Contributing

Pull requests are welcome! Please open an issue first to discuss major changes.

---

**Built with**: Rust, Axum, Ollama, TypeScript, Chrome Extensions API
