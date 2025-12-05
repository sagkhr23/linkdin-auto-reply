chrome.action.onClicked.addListener((tab: chrome.tabs.Tab) => {
  if (!tab.id || !tab.url || !tab.url.includes("linkedin.com")) {
    console.log("Not a LinkedIn tab; ignoring click");
    return;
  }

  chrome.tabs.sendMessage(
    tab.id,
    { type: "GENERATE_REPLY" },
    (response) => {
      if (chrome.runtime.lastError) {
        console.log("No content script in this tab:", chrome.runtime.lastError.message);
      } else {
        console.log("Content script responded to GENERATE_REPLY:", response);
      }
    }
  );
});

// Handle backend calls from the content script
chrome.runtime.onMessage.addListener(
  (
    msg: any,
    _sender: chrome.runtime.MessageSender,
    sendResponse: (response: any) => void
  ) => {
    if (msg.type === "CALL_BACKEND") {
      const { message, thread_context, sender_headline } = msg.payload;

      fetch("http://localhost:8000/generate_reply", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ message, thread_context, sender_headline })
      })
        .then((r) => r.json())
        .then((data) => {
          sendResponse({ ok: true, data });
        })
        .catch((err) => {
          console.error("Backend fetch failed:", err);
          sendResponse({ ok: false, error: String(err) });
        });

      return true; // keep channel open for async response
    }
  }
);