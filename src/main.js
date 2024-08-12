
const { invoke } = window.__TAURI__.tauri;

async function sendMessage() {
  const userInputEl = document.querySelector("#user-input");
  const chatHistoryEl = document.querySelector("#chat-history");

  const userMessage = userInputEl.value.trim();
  if (userMessage === "") return;

  // Append user message to the chat history
  appendMessage(chatHistoryEl, "You", userMessage);
  userInputEl.value = "";

  // Get the full chat history text
  const chatHistoryText = chatHistoryEl.innerText;

  // Summarize the chat history
  const summarizedChatHistory = await invoke("summarizer", { chatHistory: chatHistoryText });

  // Call the Rust function to generate the chatbot response using the summarized chat history
  const botResponse = await invoke("generate_response", { question: userMessage, chatHistory: summarizedChatHistory });

  // Append the chatbot's response to the chat history
  appendMessage(chatHistoryEl, "Bot", botResponse);
}

function appendMessage(chatHistoryEl, sender, message) {
  const messageEl = document.createElement("div");
  messageEl.classList.add("message");

  const senderEl = document.createElement("strong");
  senderEl.textContent = `${sender}: `;

  messageEl.appendChild(senderEl);

  if (sender === "Bot") {
    // Insert the bot's response as HTML
    const botMessageEl = document.createElement("span");
    botMessageEl.innerHTML = message; // This allows HTML to be rendered
    messageEl.appendChild(botMessageEl);
  } else {
    // Insert the user's message as plain text
    messageEl.appendChild(document.createTextNode(message));
  }

  chatHistoryEl.appendChild(messageEl);

  // Scroll to the bottom of the chat history
  chatHistoryEl.scrollTop = chatHistoryEl.scrollHeight;
}
window.addEventListener("DOMContentLoaded", () => {
  const sendButtonEl = document.querySelector("#send-button");
  sendButtonEl.addEventListener("click", sendMessage);

  const userInputEl = document.querySelector("#user-input");
  userInputEl.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      sendMessage();
    }
  });
});
