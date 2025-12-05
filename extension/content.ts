function isRecruiterHeadline(headline: string | null | undefined): boolean {
  if (!headline) return false;
  const lower = headline.toLowerCase();
  const keywords = ["recruiter", "talent", "hiring", "hr", "people partner"];
  return keywords.some(k => lower.includes(k));
}

function findLatestMessageAndHeadline() {
  // Adjust selectors as needed by inspecting LinkedIn DOM:
  const messageElems = document.querySelectorAll<HTMLElement>(".msg-s-message-list__event");
  const lastMessageElem = messageElems[messageElems.length - 1];
  if (!lastMessageElem) return null;

  const message = lastMessageElem.innerText.trim();

  const headlineElem = document.querySelector<HTMLElement>(".msg-s-profile-card__occupation");
  const headline = headlineElem?.innerText?.trim() ?? null;

  return { message, headline };
}

async function maybeGenerateReply() {
  const data = findLatestMessageAndHeadline();
  if (!data) {
    console.log("No message found");
    return;
  }

  console.log("Latest message:", data.message);
  console.log("Detected headline:", data.headline);

  let recruiter = false;

  // 1) Try by headline if present
  if (isRecruiterHeadline(data.headline)) {
    recruiter = true;
  } else {
    // 2) Fallback: look for recruiter-like keywords in the message text
    const lowerMsg = data.message.toLowerCase();
    const recruiterKeywords = [
      "opportunity",
      "role",
      "position",
      "opening",
      "hiring",
      "recruiter",
      "cv",
      "resume",
      "total exp",
      "relevant exp",
      "current ctc",
      "expected ctc",
      "notice period"
    ];
    recruiter = recruiterKeywords.some(k => lowerMsg.includes(k));
  }

  if (!recruiter) {
    console.log("Not detected as recruiter message (headline + text check). Skipping.");
    return;
  }

  chrome.runtime.sendMessage(
    {
      type: "CALL_BACKEND",
      payload: {
        message: data.message,
        thread_context: null,
        sender_headline: data.headline
      }
    },
    (response: any) => {
      if (!response || !response.ok) {
        console.error("Backend call failed from background:", response?.error);
        return;
      }

      const { reply, reason } = response.data as { reply: string; reason: string };

      if (reply === "SKIP_AUTOREPLY") {
        console.log("Model chose to skip auto-reply");
        return;
      }

      const inputBox =
        document.querySelector<HTMLElement>('div.msg-form__contenteditable[contenteditable="true"]') ??
        document.querySelector<HTMLElement>('div.msg-form__contenteditable');

      if (!inputBox) {
        console.log("No input box found");
        return;
      }

      (async () => {
        // If page isn't focused, skip clipboard and use fallback insert
        if (!document.hasFocus()) {
          console.log("Document not focused, using fallback insert.");
          
          // Convert \n to <br> for contenteditable
          inputBox.innerHTML = reply.replace(/\n/g, '<br>');
          inputBox.dispatchEvent(new InputEvent("input", { bubbles: true }));
          console.log("Filled reply (fallback), reason:", reason);
          return;
        }

        let previousClipboard: string | null = null;

        try {
          previousClipboard = await navigator.clipboard.readText();
        } catch {
          // ignore if we can't read
        }

        try {
          await navigator.clipboard.writeText(reply);
        } catch (err) {
          console.log("Could not write to clipboard, falling back to direct insert:", err);
          
          // Convert \n to <br> for contenteditable
          inputBox.innerHTML = reply.replace(/\n/g, '<br>');
          inputBox.dispatchEvent(new InputEvent("input", { bubbles: true }));
          console.log("Filled reply (fallback), reason:", reason);
          return;
        }

        // Focus and replace existing content, then paste
        inputBox.focus();
        document.execCommand("selectAll", false, undefined);
        document.execCommand("paste");

        // Fire an extra input event just in case
        inputBox.dispatchEvent(new InputEvent("input", { bubbles: true }));

        // Restore previous clipboard if we were able to read it
        if (previousClipboard !== null) {
          try {
            await navigator.clipboard.writeText(previousClipboard);
          } catch {
            // ignore
          }
        }

        console.log("Filled reply via paste, reason:", reason);
      })();
    }
  );
}

// ... existing code above ...

chrome.runtime.onMessage.addListener(
  (msg: any, _sender: chrome.runtime.MessageSender, sendResponse: (response: any) => void) => {
    if (msg.type === "GENERATE_REPLY") {
      maybeGenerateReply().then(() => sendResponse({ ok: true }));
      return true; // keep channel open for async
    }
  }
);